#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voucher {
    #[serde(rename = "id", alias = "_id")]
    pub id: String,
    #[serde(
        rename = "createdAt",
        alias = "create_time",
        deserialize_with = "deserialize_timestamp"
    )]
    pub created_at: String,
    #[serde(rename = "name", alias = "note", default)]
    pub name: String,
    pub code: String,
    #[serde(
        rename = "authorizedGuestLimit",
        alias = "quota",
        default,
        deserialize_with = "deserialize_optional_u64",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_guest_limit: Option<u64>,
    #[serde(rename = "authorizedGuestCount", alias = "used", default)]
    pub authorized_guest_count: u64,
    #[serde(
        rename = "activatedAt",
        alias = "start_time",
        default,
        deserialize_with = "deserialize_optional_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub activated_at: Option<String>,
    #[serde(
        rename = "expiresAt",
        alias = "end_time",
        default,
        deserialize_with = "deserialize_optional_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub expired: bool,
    #[serde(rename = "timeLimitMinutes", alias = "duration", default)]
    pub time_limit_minutes: u64,
    #[serde(
        rename = "dataUsageLimitMBytes",
        alias = "qos_usage_quota",
        default,
        deserialize_with = "deserialize_optional_u64",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_usage_limit_mbytes: Option<u64>,
    #[serde(
        rename = "txRateLimitKbps",
        alias = "qos_rate_max_up",
        default,
        deserialize_with = "deserialize_optional_u64",
        skip_serializing_if = "Option::is_none"
    )]
    pub tx_rate_limit_kbps: Option<u64>,
    #[serde(
        rename = "rxRateLimitKbps",
        alias = "qos_rate_max_down",
        default,
        deserialize_with = "deserialize_optional_u64",
        skip_serializing_if = "Option::is_none"
    )]
    pub rx_rate_limit_kbps: Option<u64>,
    
}

// Custom deserializer for timestamps that can be either integers or strings
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct TimestampVisitor;

    impl<'de> Visitor<'de> for TimestampVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer or string timestamp")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    deserializer.deserialize_any(TimestampVisitor)
}

// Custom deserializer for optional timestamps
fn deserialize_optional_timestamp<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct OptionalTimestampVisitor;

    impl<'de> Visitor<'de> for OptionalTimestampVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional integer or string timestamp")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserialize_timestamp(deserializer).map(Some)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }
    }

    deserializer.deserialize_option(OptionalTimestampVisitor)
}

// Custom deserializer for optional u64 that can handle booleans, integers, or missing values
fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct OptionalU64Visitor;

    impl<'de> Visitor<'de> for OptionalU64Visitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional integer, boolean, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(OptionalU64Visitor)
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Convert boolean to Option: true -> Some(0), false -> None
            if value {
                Ok(Some(0))
            } else {
                Ok(None)
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value >= 0 {
                Ok(Some(value as u64))
            } else {
                Ok(None)
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }
    }

    deserializer.deserialize_option(OptionalU64Visitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVoucherRequest {
    pub count: u32,
    pub name: String,
    #[serde(rename = "authorizedGuestLimit")]
    pub authorized_guest_limit: Option<u64>,
    #[serde(rename = "timeLimitMinutes")]
    pub time_limit_minutes: u64,
    #[serde(rename = "dataUsageLimitMBytes")]
    pub data_usage_limit_mbytes: Option<u64>,
    #[serde(rename = "rxRateLimitKbps")]
    pub rx_rate_limit_kbps: Option<u64>,
    #[serde(rename = "txRateLimitKbps")]
    pub tx_rate_limit_kbps: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVoucherApiResponse {
    pub meta: serde_json::Value,
    pub data: Vec<CreateVoucherData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVoucherData {
    pub create_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVoucherResponse {
    pub vouchers: Vec<Voucher>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVouchersResponse {
    pub data: Vec<Voucher>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    #[serde(rename = "vouchersDeleted")]
    pub vouchers_deleted: u32,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub ids: String,
}

#[derive(Debug, Deserialize)]
pub struct DetailsRequest {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub id: String,
    #[serde(rename = "internalReference")]
    pub internal_reference: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSitesResponse {
    offset: u64,
    limit: u32,
    count: u32,
    #[serde(rename = "totalCount")]
    total_count: u32,
    pub data: Vec<Site>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    #[serde(rename = "statusName")]
    pub status_name: String,
    pub message: String,
    timestamp: String,
    #[serde(rename = "requestPath")]
    request_path: String,
    #[serde(rename = "requestId")]
    request_id: String,
}
