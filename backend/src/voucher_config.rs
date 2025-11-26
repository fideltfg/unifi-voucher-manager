use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;
use tracing::{error, info};

pub static VOUCHER_CONFIG: OnceLock<VoucherConfig> = OnceLock::new();

const DEFAULT_ROLLING_DURATION_HOURS: f64 = 24.0;
const CONFIG_FILE_PATH: &str = "/app/frontend/public/voucher-tiers.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingVoucherConfig {
    pub enabled: bool,
    pub duration_hours: f64,
    pub download_mbps: Option<u64>,
    pub upload_mbps: Option<u64>,
    pub data_limit_mb: Option<u64>,
    #[serde(default = "default_min_rolling_vouchers")]
    pub min_rolling_vouchers: u32,
}

fn default_min_rolling_vouchers() -> u32 {
    1
}

impl Default for RollingVoucherConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            duration_hours: DEFAULT_ROLLING_DURATION_HOURS,
            download_mbps: None,
            upload_mbps: None,
            data_limit_mb: None,
            min_rolling_vouchers: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTier {
    pub id: String,
    pub name: String,
    pub description: String,
    pub duration_hours: f64,
    pub download_mbps: Option<u64>,
    pub upload_mbps: Option<u64>,
    pub data_limit_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherConfigFile {
    pub rolling_voucher: Option<RollingVoucherConfig>,
    pub tiers: Vec<VoucherTier>,
}

#[derive(Debug, Clone)]
pub struct VoucherConfig {
    pub rolling_voucher: RollingVoucherConfig,
}

impl VoucherConfig {
    pub fn try_new() -> Result<Self, String> {
        let config_file = match fs::read_to_string(CONFIG_FILE_PATH) {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to read voucher config file: {}", e);
                info!("Using default rolling voucher configuration");
                return Ok(Self {
                    rolling_voucher: RollingVoucherConfig::default(),
                });
            }
        };

        let config: VoucherConfigFile = match serde_json::from_str(&config_file) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to parse voucher config file: {}", e);
                info!("Using default rolling voucher configuration");
                return Ok(Self {
                    rolling_voucher: RollingVoucherConfig::default(),
                });
            }
        };

        let rolling_voucher = config.rolling_voucher.unwrap_or_default();
        
        info!(
            "Loaded rolling voucher config: enabled={}, duration={}h, download={}Mbps, upload={}Mbps, data_limit={}MB",
            rolling_voucher.enabled,
            rolling_voucher.duration_hours,
            rolling_voucher.download_mbps.map(|v| v.to_string()).unwrap_or_else(|| "unlimited".to_string()),
            rolling_voucher.upload_mbps.map(|v| v.to_string()).unwrap_or_else(|| "unlimited".to_string()),
            rolling_voucher.data_limit_mb.map(|v| v.to_string()).unwrap_or_else(|| "unlimited".to_string()),
        );

        Ok(Self { rolling_voucher })
    }

    pub fn duration_minutes(&self) -> u64 {
        (self.rolling_voucher.duration_hours * 60.0) as u64
    }

    pub fn download_kbps(&self) -> Option<u64> {
        self.rolling_voucher.download_mbps.map(|mbps| mbps * 1000)
    }

    pub fn upload_kbps(&self) -> Option<u64> {
        self.rolling_voucher.upload_mbps.map(|mbps| mbps * 1000)
    }

    pub fn data_limit_mb(&self) -> Option<u64> {
        self.rolling_voucher.data_limit_mb
    }
}
