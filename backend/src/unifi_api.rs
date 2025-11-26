use axum::http::HeaderValue;
use chrono::DateTime;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use reqwest::{Client, ClientBuilder, StatusCode, cookie::Jar};
use std::{sync::{Arc, OnceLock, RwLock}, time::Duration};
use tracing::{debug, error, info, warn};

use crate::{
    environment::{ENVIRONMENT, Environment},
    models::{
        CreateVoucherApiResponse, CreateVoucherRequest, CreateVoucherResponse, DeleteResponse,
        ErrorResponse, GetSitesResponse, GetVouchersResponse, Voucher,
    },
};

const UNIFI_API_ROUTE: &str = "api/s";
const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const ROLLING_VOUCHER_NAME_PREFIX: &str = "[ROLLING]";
const FRAGMENT: &AsciiSet = &CONTROLS.add(b'[').add(b']');

pub static UNIFI_API: OnceLock<UnifiAPI> = OnceLock::new();

#[derive(Debug, Clone)]
enum RequestType {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone)]
pub struct UnifiAPI<'a> {
    client: Client,
    cookie_jar: Arc<Jar>,
    session_expiry: Arc<RwLock<Option<std::time::Instant>>>,
    sites_api_url: String,
    voucher_api_url: String,
    environment: &'a Environment,
}

impl<'a> UnifiAPI<'a> {
    pub async fn try_new() -> Result<Self, String> {
        let environment: &Environment = ENVIRONMENT.get().expect("Environment not set");

        let cookie_jar = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .cookie_provider(Arc::clone(&cookie_jar))
            .danger_accept_invalid_certs(!environment.unifi_has_valid_cert)
            .use_rustls_tls()
            .build()
            .expect("Failed to build UniFi reqwest client");

        let mut unifi_api = Self {
            client,
            cookie_jar,
            session_expiry: Arc::new(RwLock::new(None)),
            sites_api_url: format!("{}/{}", environment.unifi_controller_url, UNIFI_API_ROUTE),
            voucher_api_url: String::new(),
            environment,
        };

        // Authenticate immediately
        unifi_api.login().await?;

        let site_id = match environment.unifi_site_id.to_lowercase().as_str() {
            "default" => {
                info!("Using 'default' as site ID");
                "default".to_string()
            }
            _ => environment.unifi_site_id.clone(),
        };

        unifi_api.voucher_api_url =
            format!("{}/{}/cmd/hotspot", unifi_api.sites_api_url, site_id);

        Ok(unifi_api)
    }

    async fn login(&self) -> Result<(), String> {
        let login_url = format!("{}/api/login", self.environment.unifi_controller_url);
        
        info!("Authenticating with UniFi Controller at: {}", login_url);
        
        let login_body = serde_json::json!({
            "username": self.environment.unifi_username,
            "password": self.environment.unifi_password,
            "remember": false
        });
        
        let response = self.client
            .post(&login_url)
            .json(&login_body)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("UniFi authentication failed: {} - {}", status, error_text);
            return Err(format!("Authentication failed with status {}: {}", status, error_text));
        }
        
        // Set session expiry to 30 minutes from now
        if let Ok(mut expiry) = self.session_expiry.write() {
            *expiry = Some(std::time::Instant::now() + Duration::from_secs(30 * 60));
        }
        
        info!("UniFi authentication successful, session will expire in 30 minutes");
        Ok(())
    }

    async fn ensure_authenticated(&self) -> Result<(), String> {
        // Check if session is still valid
        let needs_reauth = {
            if let Ok(expiry) = self.session_expiry.read() {
                if let Some(exp_time) = *expiry {
                    std::time::Instant::now() >= exp_time
                } else {
                    true
                }
            } else {
                true
            }
        };
        
        if needs_reauth {
            info!("Session expired or not authenticated, re-authenticating...");
            self.login().await?;
        }
        
        Ok(())
    }

    fn format_unifi_date(&self, timestamp_string: &str) -> String {
        // Try parsing as Unix timestamp (in seconds)
        if let Ok(timestamp) = timestamp_string.parse::<i64>() {
            if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                let local_time = dt.with_timezone(&self.environment.timezone);
                return local_time.format(DATE_TIME_FORMAT).to_string();
            }
        }
        
        // Fallback: try parsing as RFC3339 (for backwards compatibility)
        match DateTime::parse_from_rfc3339(timestamp_string) {
            Ok(dt) => {
                let local_time = dt.with_timezone(&self.environment.timezone);
                local_time.format(DATE_TIME_FORMAT).to_string()
            }
            Err(_) => {
                error!("Failed to parse date: {}", timestamp_string);
                timestamp_string.to_string()
            }
        }
    }

    fn process_voucher(&self, voucher: &mut Voucher) {
        // Convert Unix timestamp strings to formatted dates
        voucher.created_at = self.format_unifi_date(&voucher.created_at);
        
        if let Some(ref activated_at) = voucher.activated_at {
            voucher.activated_at = Some(self.format_unifi_date(activated_at));
        }
        
        if let Some(ref expires_at) = voucher.expires_at {
            // Check if expired based on timestamp
            if let Ok(end_timestamp) = expires_at.parse::<i64>() {
                let now = chrono::Utc::now().timestamp();
                voucher.expired = end_timestamp < now;
            }
            voucher.expires_at = Some(self.format_unifi_date(expires_at));
        }
    }

    fn process_vouchers(&self, mut vouchers: Vec<Voucher>) -> Vec<Voucher> {
        vouchers.iter_mut().for_each(|voucher| {
            self.process_voucher(voucher);
        });
        vouchers
    }

    async fn make_request_raw<T: serde::ser::Serialize + Sized>(
        &self,
        request_type: RequestType,
        url: &str,
        body: Option<&T>,
    ) -> Result<String, StatusCode> {
        // Ensure we have a valid session
        self.ensure_authenticated().await.map_err(|e| {
            error!("Authentication failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        // Make request
        let response_result = match request_type {
            RequestType::Get => self.client.get(url).send().await,
            RequestType::Post => {
                if let Some(b) = body {
                    self.client.post(url).json(b).send().await
                } else {
                    error!("Body is required for POST requests");
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            RequestType::Delete => self.client.delete(url).send().await,
        };

        // Check if the request was successful
        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                let status = e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                error!(
                    "Request failed with status {}: {:?}",
                    status,
                    e.without_url()
                );
                return Err(status);
            }
        };

        // The request was successful, now check the status code
        let clean_response = match response.error_for_status_ref().is_ok() {
            true => response,
            false => {
                let status = response.status();
                error!("Request failed with status: {}", status);

                if let Ok(body) = response.json::<ErrorResponse>().await {
                    error!("Error response message: {}", body.message);
                } else {
                    error!("Failed to parse error response body");
                }
                return Err(status);
            }
        };

        // It's a successful response, return raw text
        clean_response.text().await.map_err(|e| {
            error!("Failed to read response body: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    async fn make_request<
        T: serde::ser::Serialize + Sized,
        U: serde::de::DeserializeOwned + Sized,
    >(
        &self,
        request_type: RequestType,
        url: &str,
        body: Option<&T>,
    ) -> Result<U, StatusCode> {
        // Try the request, and if we get a 401, re-authenticate and retry once
        match self.make_request_internal(request_type.clone(), url, body).await {
            Err(StatusCode::UNAUTHORIZED) => {
                warn!("Got 401, re-authenticating and retrying...");
                // Clear session expiry to force re-authentication
                if let Ok(mut expiry) = self.session_expiry.write() {
                    *expiry = None;
                }
                // Re-authenticate
                self.ensure_authenticated().await.map_err(|e| {
                    error!("Re-authentication failed: {}", e);
                    StatusCode::UNAUTHORIZED
                })?;
                // Retry the request
                self.make_request_internal(request_type, url, body).await
            }
            other => other,
        }
    }

    async fn make_request_internal<
        T: serde::ser::Serialize + Sized,
        U: serde::de::DeserializeOwned + Sized,
    >(
        &self,
        request_type: RequestType,
        url: &str,
        body: Option<&T>,
    ) -> Result<U, StatusCode> {
        // Ensure we have a valid session
        self.ensure_authenticated().await.map_err(|e| {
            error!("Authentication failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        // Make request
        let response_result = match request_type {
            RequestType::Get => self.client.get(url).send().await,
            RequestType::Post => {
                if let Some(b) = body {
                    self.client.post(url).json(b).send().await
                } else {
                    error!("Body is required for POST requests");
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            RequestType::Delete => self.client.delete(url).send().await,
        };

        // Check if the request was successful
        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                let status = e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                error!(
                    "Request failed with status {}: {:?}",
                    status,
                    e.without_url()
                );
                return Err(status);
            }
        };

        // The request was successful, now check the status code
        let clean_response = match response.error_for_status_ref().is_ok() {
            true => response,
            false => {
                let status = response.status();
                error!("Request failed with status: {}", status);

                if let Ok(body) = response.json::<ErrorResponse>().await {
                    error!("Error response message: {}", body.message);
                } else {
                    error!("Failed to parse error response body");
                }
                return Err(status);
            }
        };

        // It's a successful response, now get the response body
        let response_text = clean_response.text().await.map_err(|e| {
            error!("Failed to read response body: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // Parse the response body as JSON
        let response_json: serde_json::Value =
            serde_json::from_str(&response_text).map_err(|e| {
                error!("Failed to parse response body as JSON: {:?}", e);
                debug!("Response body: {}", response_text);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        // Parse the JSON into the expected structure
        serde_json::from_value::<U>(response_json.clone()).map_err(|e| {
            error!("Failed to parse response JSON structure: {:?}", e);
            error!("Response JSON: {:?}", response_json);
            debug!("Response body: {}", response_text);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    pub async fn get_all_vouchers(&self) -> Result<GetVouchersResponse, StatusCode> {
        let url = format!(
            "{}/{}/stat/voucher",
            self.sites_api_url,
            self.environment.unifi_site_id
        );
        let mut result: GetVouchersResponse = self
            .make_request(RequestType::Get, &url, None::<&()>)
            .await?;
        result.data = self.process_vouchers(result.data);
        Ok(result)
    }

    async fn get_all_vouchers_raw(&self) -> Result<GetVouchersResponse, StatusCode> {
        let url = format!(
            "{}/{}/stat/voucher",
            self.sites_api_url,
            self.environment.unifi_site_id
        );
        self.make_request(RequestType::Get, &url, None::<&()>).await
    }

    pub async fn get_rolling_voucher(&self) -> Result<Option<Voucher>, StatusCode> {
        let response = self.get_all_vouchers().await?;

        // Find the most recent unused rolling voucher
        let rolling = response
            .data
            .iter()
            .filter(|voucher| {
                voucher.name.starts_with(ROLLING_VOUCHER_NAME_PREFIX) 
                    && voucher.authorized_guest_count == 0  // Only unused vouchers
                    && !voucher.expired  // Only non-expired vouchers
            })
            .max_by_key(|voucher| {
                DateTime::parse_from_str(&voucher.created_at, DATE_TIME_FORMAT)
                    .unwrap_or_else(|_| DateTime::UNIX_EPOCH.fixed_offset())
            })
            .cloned();

        Ok(rolling)
    }

    pub async fn get_all_unused_rolling_vouchers(&self) -> Result<Vec<Voucher>, StatusCode> {
        let response = self.get_all_vouchers().await?;

        // Get all unused rolling vouchers, sorted by creation time (oldest first)
        let mut vouchers: Vec<Voucher> = response
            .data
            .into_iter()
            .filter(|voucher| {
                voucher.name.starts_with(ROLLING_VOUCHER_NAME_PREFIX)
                    && voucher.authorized_guest_count == 0
                    && !voucher.expired
            })
            .collect();

        vouchers.sort_by(|a, b| {
            let time_a = DateTime::parse_from_str(&a.created_at, DATE_TIME_FORMAT)
                .unwrap_or_else(|_| DateTime::UNIX_EPOCH.fixed_offset());
            let time_b = DateTime::parse_from_str(&b.created_at, DATE_TIME_FORMAT)
                .unwrap_or_else(|_| DateTime::UNIX_EPOCH.fixed_offset());
            time_a.cmp(&time_b)
        });

        Ok(vouchers)
    }

    pub async fn get_rolling_voucher_by_index(&self, index: usize) -> Result<Option<Voucher>, StatusCode> {
        let vouchers = self.get_all_unused_rolling_vouchers().await?;
        Ok(vouchers.get(index).cloned())
    }

    pub async fn get_newest_voucher(&self) -> Result<Voucher, StatusCode> {
        let response = self.get_all_vouchers().await?;

        if response.data.is_empty() {
            warn!("No vouchers found when fetching the newest voucher");
            return Err(StatusCode::NOT_FOUND);
        }

        // Find the newest voucher
        let newest = response
            .data
            .iter()
            .max_by_key(|voucher| {
                DateTime::parse_from_str(&voucher.created_at, DATE_TIME_FORMAT)
                    .unwrap_or_else(|_| DateTime::UNIX_EPOCH.fixed_offset())
            })
            .cloned()
            .expect("At least one voucher should exist");

        Ok(newest)
    }

    pub async fn get_voucher_details(&self, id: String) -> Result<Voucher, StatusCode> {
        // Traditional API doesn't have individual voucher endpoint, get all and filter
        let response = self.get_all_vouchers().await?;
        
        response
            .data
            .into_iter()
            .find(|v| v.id == id)
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub async fn create_voucher(
        &self,
        request: CreateVoucherRequest,
    ) -> Result<CreateVoucherResponse, StatusCode> {
        // Traditional API uses POST with cmd=create-voucher
        let mut body = serde_json::json!({
            "cmd": "create-voucher",
            "n": request.count,
            "expire": request.time_limit_minutes,
        });

        // Add optional fields only if they have meaningful values
        if !request.name.is_empty() {
            body["note"] = serde_json::json!(request.name);
        }
        
        if let Some(quota) = request.authorized_guest_limit {
            if quota > 0 {
                body["quota"] = serde_json::json!(quota);
            }
        }
        
        if let Some(up) = request.tx_rate_limit_kbps {
            if up > 0 {
                body["up"] = serde_json::json!(up);
            }
        }
        
        if let Some(down) = request.rx_rate_limit_kbps {
            if down > 0 {
                body["down"] = serde_json::json!(down);
            }
        }
        
        if let Some(bytes) = request.data_usage_limit_mbytes {
            if bytes > 0 {
                body["bytes"] = serde_json::json!(bytes);
            }
        }

        // Make the create voucher request
        let api_response: CreateVoucherApiResponse = self
            .make_request(RequestType::Post, &self.voucher_api_url, Some(&body))
            .await?;
        
        // The UniFi API only returns create_time, not the full voucher details
        // So we need to fetch all vouchers and find the newly created ones
        // Get the create timestamps from the API response
        let create_times: Vec<i64> = api_response.data.iter().map(|d| d.create_time).collect();
        
        info!("API returned {} create_times: {:?}", create_times.len(), create_times);
        
        // Get raw vouchers without timestamp processing
        let all_vouchers_raw = self.get_all_vouchers_raw().await?;
        
        // Find vouchers matching the create_time from the response
        let mut newly_created_raw: Vec<Voucher> = all_vouchers_raw
            .data
            .into_iter()
            .filter(|v| {
                // Parse the created_at field (which is still a timestamp string at this point)
                if let Ok(created_timestamp) = v.created_at.parse::<i64>() {
                    create_times.contains(&created_timestamp)
                } else {
                    false
                }
            })
            .take(request.count as usize)
            .collect();
        
        // Now process the newly created vouchers to format timestamps
        newly_created_raw = self.process_vouchers(newly_created_raw);
        
        info!("Returning {} newly created vouchers", newly_created_raw.len());
        
        Ok(CreateVoucherResponse {
            vouchers: newly_created_raw,
        })
    }

    pub async fn check_rolling_voucher_ip(&self, ip: &str) -> Result<bool, StatusCode> {
        let response = self.get_all_vouchers().await?;

        // Find a rolling voucher that contains the given IP address
        let rolling = response
            .data
            .iter()
            .find(|voucher| {
                !voucher.expired
                    && voucher.name.starts_with(ROLLING_VOUCHER_NAME_PREFIX)
                    && voucher.name.ends_with(ip)
            })
            .cloned();

        Ok(rolling.is_some())
    }

    pub async fn create_rolling_voucher(&self, ip: &str) -> Result<Voucher, StatusCode> {
        let voucher_config = crate::voucher_config::VOUCHER_CONFIG
            .get()
            .expect("Voucher config not initialized");

        let request = CreateVoucherRequest {
            count: 1,
            name: format!(
                "{} {}-{}",
                ROLLING_VOUCHER_NAME_PREFIX,
                chrono::Local::now().format("%Y%m%d%H%M%S"),
                ip
            ),
            time_limit_minutes: voucher_config.duration_minutes(),
            authorized_guest_limit: None,
            data_usage_limit_mbytes: voucher_config.data_limit_mb(),
            tx_rate_limit_kbps: voucher_config.download_kbps(),
            rx_rate_limit_kbps: voucher_config.upload_kbps(),
        };

        let rolling = self
            .create_voucher(request)
            .await?
            .vouchers
            .first()
            .cloned();

        match rolling {
            Some(v) => Ok(v),
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn create_new_rolling_voucher_if_needed(&self) -> Result<Option<Voucher>, StatusCode> {
        let voucher_config = crate::voucher_config::VOUCHER_CONFIG
            .get()
            .expect("Voucher config not initialized");

        let min_vouchers = voucher_config.rolling_voucher.min_rolling_vouchers as usize;
        let unused_vouchers = self.get_all_unused_rolling_vouchers().await?;
        let current_count = unused_vouchers.len();

        if current_count >= min_vouchers {
            // We already have enough unused rolling vouchers
            debug!("Already have {} unused rolling vouchers (min: {}), no action needed", current_count, min_vouchers);
            return Ok(None);
        }

        // Need to create more rolling vouchers
        let vouchers_to_create = min_vouchers - current_count;
        info!("Creating {} rolling voucher(s) to maintain minimum of {} (current: {})", vouchers_to_create, min_vouchers, current_count);

        // Create vouchers one at a time to ensure unique names
        let mut created_vouchers = Vec::new();
        for i in 0..vouchers_to_create {
            let request = CreateVoucherRequest {
                count: 1,
                name: format!(
                    "{} {}-auto-{}",
                    ROLLING_VOUCHER_NAME_PREFIX,
                    chrono::Local::now().format("%Y%m%d%H%M%S"),
                    i
                ),
                time_limit_minutes: voucher_config.duration_minutes(),
                authorized_guest_limit: None,
                data_usage_limit_mbytes: voucher_config.data_limit_mb(),
                tx_rate_limit_kbps: voucher_config.download_kbps(),
                rx_rate_limit_kbps: voucher_config.upload_kbps(),
            };

            match self.create_voucher(request).await {
                Ok(result) => {
                    if let Some(voucher) = result.vouchers.first() {
                        info!("Created rolling voucher {}/{}: id={}, code={}", i+1, vouchers_to_create, voucher.id, voucher.code);
                        created_vouchers.push(voucher.clone());
                    }
                }
                Err(e) => {
                    error!("Failed to create rolling voucher {}/{}: {}", i+1, vouchers_to_create, e);
                }
            }
            
            // Small delay between creations to ensure unique timestamps
            if i < vouchers_to_create - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        
        Ok(created_vouchers.first().cloned())
    }

    pub async fn delete_vouchers_by_ids(
        &self,
        ids: Vec<String>,
    ) -> Result<DeleteResponse, StatusCode> {
        if ids.is_empty() || (ids.len() == 1 && ids[0].is_empty()) {
            return Ok(DeleteResponse {
                data: vec![],
                meta: crate::models::DeleteMeta {
                    rc: "ok".to_string(),
                },
            });
        }

        info!("Sending delete request to UniFi for voucher IDs: {:?}", ids);
        
        // Try deleting vouchers one at a time
        let mut deleted_count = 0;
        let mut all_successful = true;
        
        for id in &ids {
            let body = serde_json::json!({
                "cmd": "delete-voucher",
                "_id": id,
            });
            info!("Delete request body for ID {}: {}", id, body);
            
            let result: Result<DeleteResponse, StatusCode> = self.make_request(RequestType::Post, &self.voucher_api_url, Some(&body))
                .await;
            
            match &result {
                Ok(response) => {
                    info!("UniFi delete response for {}: data.len={}, meta.rc={}", id, response.data.len(), response.meta.rc);
                    if response.meta.rc == "ok" {
                        deleted_count += 1;
                    } else {
                        all_successful = false;
                    }
                }
                Err(e) => {
                    error!("UniFi delete error for {}: {}", id, e);
                    all_successful = false;
                }
            }
        }
        
        info!("Delete operation completed: {}/{} vouchers deleted", deleted_count, ids.len());
        
        // Create a response with the count of successfully deleted vouchers
        // UniFi API returns empty data array, so we populate it with dummy entries to indicate count
        let data_array: Vec<serde_json::Value> = (0..deleted_count).map(|_| serde_json::json!({})).collect();
        
        Ok(DeleteResponse {
            data: data_array,
            meta: crate::models::DeleteMeta {
                rc: "ok".to_string(),
            },
        })
    }

    pub async fn delete_expired_vouchers(&self) -> Result<DeleteResponse, StatusCode> {
        let response = self.get_all_vouchers().await?;
        let expired_ids: Vec<String> = response
            .data
            .into_iter()
            .filter(|v| v.expired)
            .map(|v| v.id)
            .collect();

        self.delete_vouchers_by_ids(expired_ids).await
    }

    pub async fn delete_expired_rolling_vouchers(&self) -> Result<DeleteResponse, StatusCode> {
        let response = self.get_all_vouchers().await?;
        let expired_rolling_ids: Vec<String> = response
            .data
            .into_iter()
            .filter(|v| v.expired && v.name.starts_with(ROLLING_VOUCHER_NAME_PREFIX))
            .map(|v| v.id)
            .collect();

        self.delete_vouchers_by_ids(expired_rolling_ids).await
    }
}
