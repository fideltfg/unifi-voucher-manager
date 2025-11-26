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

pub async fn get_rolling_voucher_handler(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Voucher>, StatusCode> {
    debug!("Received request to get rolling voucher");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    
    // Check if an index was provided for multi-kiosk support
    let index = params.get("index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    
    debug!("Getting rolling voucher at index {}", index);
    match client.get_rolling_voucher_by_index(index).await {
        Ok(Some(voucher)) => {
            info!("Returning rolling voucher at index {}: id={}, code={}", index, voucher.id, voucher.code);
            Ok(Json(voucher))
        }
        Ok(None) => {
            info!("No rolling voucher found at index {}", index);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Failed to get rolling voucher at index {}: {}", index, e);
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

pub async fn get_all_rolling_vouchers_handler() -> Result<Json<Vec<Voucher>>, StatusCode> {
    debug!("Received request to get all unused rolling vouchers");
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    
    match client.get_all_unused_rolling_vouchers().await {
        Ok(vouchers) => {
            debug!("Found {} unused rolling vouchers", vouchers.len());
            Ok(Json(vouchers))
        }
        Err(e) => {
            error!("Failed to get unused rolling vouchers: {}", e);
            Err(e)
        }
    }
}

pub async fn rotate_rolling_voucher_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    debug!("Received request to check and rotate rolling voucher if needed");

    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    
    match client.create_new_rolling_voucher_if_needed().await {
        Ok(Some(voucher)) => {
            info!("New rolling voucher created: id={}, code={}", voucher.id, voucher.code);
            Ok(Json(serde_json::json!({
                "status": "created",
                "voucher": voucher
            })))
        }
        Ok(None) => {
            debug!("No new rolling voucher needed, minimum count already met");
            Ok(Json(serde_json::json!({
                "status": "no_action_needed",
                "message": "Minimum rolling vouchers already exist"
            })))
        }
        Err(e) => {
            error!("Failed to check/create rolling voucher: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_selected_handler(
    Query(params): Query<DeleteRequest>,
) -> Result<Json<DeleteResponse>, StatusCode> {
    info!("Received request to delete selected vouchers: ids={}", params.ids);
    let client = UNIFI_API.get().expect("UnifiAPI not initialized");
    let ids: Vec<String> = params.ids.split(',').map(|s| s.to_string()).collect();
    info!("Parsed {} voucher IDs to delete", ids.len());
    match client.delete_vouchers_by_ids(ids).await {
        Ok(response) => {
            info!("Successfully deleted vouchers, response data length: {}", response.data.len());
            Ok(Json(response))
        }
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
