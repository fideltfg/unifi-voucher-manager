use axum::{
    Router,
    http::{self, Method},
    routing::{delete, get, post},
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use std::path::Path;

use backend::{
    environment::{ENVIRONMENT, Environment},
    handlers::*,
    tasks::run_daily_purge,
    unifi_api::{UNIFI_API, UnifiAPI},
    voucher_config::{VOUCHER_CONFIG, VoucherConfig},
};

#[tokio::main]
async fn main() {
    // =================================
    // Initialize tracing
    // =================================
    
    // Create logs directory if it doesn't exist
    let log_dir = Path::new("/app/logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("Failed to create logs directory");
    }
    
    // Set up file appender for voucher logs
    let file_appender = tracing_appender::rolling::daily("/app/logs", "vouchers.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    // Leak the guard to keep it alive for the entire program duration
    // This ensures log files are properly flushed
    std::mem::forget(guard);
    
    // Set up console output
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout);
    
    // Set up file output (only INFO and above to file)
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false);
    
    // Combine layers
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_env_var("BACKEND_LOG_LEVEL")
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(console_layer)
        .with(file_layer)
        .init();

    // =================================
    // Setup environment variables manager
    // =================================
    let env = match Environment::try_new() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load environment variables: {e}");
            std::process::exit(1);
        }
    };
    ENVIRONMENT
        .set(env)
        .expect("Failed to set environment variables");
    let environment = ENVIRONMENT.get().expect("Environment not set");

    // =================================
    // Load voucher configuration
    // =================================
    let voucher_config = match VoucherConfig::try_new() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load voucher configuration: {e}");
            std::process::exit(1);
        }
    };
    VOUCHER_CONFIG
        .set(voucher_config)
        .expect("Failed to set voucher configuration");

    // =================================
    // Setup UniFi Controller API connection
    // =================================
    loop {
        match UnifiAPI::try_new().await {
            Ok(api) => {
                UNIFI_API.set(api).expect("Failed to set UnifiAPI");
                info!("Successfully connected to Unifi controller");
                break;
            }
            Err(e) => {
                error!("Failed to initialize UnifiAPI wrapper: {}", e);
                warn!("Retrying connection in 5 seconds...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    // =================================
    // Start scheduled tasks
    // =================================
    tokio::spawn(run_daily_purge(environment.timezone));

    // =================================
    // Setup Axum server
    // =================================
    let cors = CorsLayer::new()
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::POST, Method::GET, Method::DELETE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/health", get(health_check_handler))
        .route("/api/vouchers", get(get_vouchers_handler))
        .route("/api/vouchers", post(create_voucher_handler))
        .route("/api/vouchers/details", get(get_voucher_details_handler))
        .route("/api/vouchers/expired", delete(delete_expired_handler))
        .route(
            "/api/vouchers/expired/rolling",
            delete(delete_expired_rolling_handler),
        )
        .route("/api/vouchers/newest", get(get_newest_voucher_handler))
        .route("/api/vouchers/rolling", get(get_rolling_voucher_handler))
        .route(
            "/api/vouchers/rolling",
            post(create_rolling_voucher_handler),
        )
        .route("/api/vouchers/selected", delete(delete_selected_handler))
        .layer(cors);

    let bind_address = format!(
        "{}:{}",
        environment.backend_bind_host, environment.backend_bind_port
    );

    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Could not bind listener");

    info!("Server running on http://{}", bind_address);

    axum::serve(listener, app)
        .await
        .expect("Axum server should never error");
}
