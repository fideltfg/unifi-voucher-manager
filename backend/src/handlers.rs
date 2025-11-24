use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use tracing::{debug, error, info};

use crate::{models::*, unifi_api::UNIFI_API};

pub async fn get_vouchers_handler() -> Result<Json<GetVouchersResponse>, StatusCode> {
    debug!("Received request to get vouchers");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.get_all_vouchers().await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to get vouchers: {}", e);
            Err(e)
        }
    }
}

pub async fn get_rolling_voucher_handler() -> Result<Json<Voucher>, StatusCode> {
    debug!("Received request to get rolling voucher");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.get_rolling_voucher().await {
        Ok(Some(voucher)) => Ok(Json(voucher)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get rolling voucher: {}", e);
            Err(e)
        }
    }
}

pub async fn get_newest_voucher_handler() -> Result<Json<Voucher>, StatusCode> {
    debug!("Received request to get newest voucher");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.get_newest_voucher().await {
        Ok(voucher) => Ok(Json(voucher)),
        Err(e) => {
            error!("Failed to get newest voucher: {}", e);
            Err(e)
        }
    }
}

pub async fn get_voucher_details_handler(
    Query(params): Query<DetailsRequest>,
) -> Result<Json<Voucher>, StatusCode> {
    debug!("Received request to get voucher details");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.get_voucher_details(params.id).await {
        Ok(voucher) => Ok(Json(voucher)),
        Err(e) => {
            error!("Failed to get voucher details: {}", e);
            Err(e)
        }
    }
}

pub async fn create_voucher_handler(
    headers: HeaderMap,
    Json(request): Json<CreateVoucherRequest>,
) -> Result<Json<CreateVoucherResponse>, StatusCode> {
    debug!("Received request to create voucher");
    
    // Extract hostname and IP for logging
    let hostname = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown");
    
    info!("Creating voucher - hostname: {}, client_ip: {}, count: {}, duration: {}min", 
        hostname, client_ip, request.count, request.time_limit_minutes);
    
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.create_voucher(request.clone()).await {
        Ok(response) => {
            info!("Voucher creation successful - hostname: {}, vouchers_created: {}", 
                hostname, response.vouchers.len());
            if let Some(first_voucher) = response.vouchers.first() {
                info!("Voucher created - hostname: {}, id: {}, code: {}", 
                    hostname, first_voucher.id, first_voucher.code);
            }
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to create voucher - hostname: {}, error: {}", hostname, e);
            Err(e)
        }
    }
}

pub async fn create_rolling_voucher_handler(
    headers: HeaderMap,
) -> Result<Json<Voucher>, StatusCode> {
    debug!("Received request to create rolling voucher");

    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    
    // Extract hostname for logging
    let hostname = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(ip) = forwarded.to_str() {
            debug!("Client IP from x-forwarded-for: {}", ip);
            
            info!("Creating rolling voucher - hostname: {}, client_ip: {}", hostname, ip);

            // Check if user already rotated the rolling voucher
            if client.check_rolling_voucher_ip(ip).await? {
                info!("Rolling voucher already rotated - hostname: {}, ip: {}", hostname, ip);
                return Err(StatusCode::FORBIDDEN);
            }

            // Voucher rotation allowed, create a new rolling voucher
            match client.create_rolling_voucher(ip).await {
                Ok(response) => {
                    info!("Rolling voucher created - hostname: {}, ip: {}, voucher_id: {}, code: {}", 
                        hostname, ip, response.id, response.code);
                    return Ok(Json(response));
                }
                Err(e) => {
                    error!("Failed to create rolling voucher - hostname: {}, ip: {}, error: {}", 
                        hostname, ip, e);
                    return Err(e);
                }
            }
        }
    }

    error!("Invalid x-forwarded-for header - hostname: {}", hostname);
    Err(StatusCode::BAD_REQUEST)
}

pub async fn delete_selected_handler(
    Query(params): Query<DeleteRequest>,
) -> Result<Json<DeleteResponse>, StatusCode> {
    debug!("Received request to delete selected vouchers");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    let ids = params.ids.split(',').map(|s| s.to_string()).collect();
    match client.delete_vouchers_by_ids(ids).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to delete selected vouchers: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_expired_handler() -> Result<Json<DeleteResponse>, StatusCode> {
    debug!("Received request to delete expired vouchers");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.delete_expired_vouchers().await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to delete expired vouchers: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_expired_rolling_handler() -> Result<Json<DeleteResponse>, StatusCode> {
    debug!("Received request to delete expired rolling voucher");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    match client.delete_expired_rolling_vouchers().await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to delete expired rolling voucher: {}", e);
            Err(e)
        }
    }
}

pub async fn health_check_handler() -> Result<Json<HealthCheckResponse>, StatusCode> {
    debug!("Received health check request");
    let response = HealthCheckResponse {
        status: "ok".to_string(),
    };
    Ok(Json(response))
}
