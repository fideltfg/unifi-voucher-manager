use chrono::Utc;
use chrono_tz::Tz;
use tokio::time::sleep;
use tracing::{error, info};

use crate::handlers::delete_expired_rolling_handler;

pub async fn run_daily_purge(timezone: Tz) {
    loop {
        let now = Utc::now().with_timezone(&timezone);
        let next_midnight = now
            .date_naive()
            .succ_opt()
            .expect("Next day is not representable")
            .and_hms_opt(0, 0, 0)
            .expect("Could not get next midnight")
            .and_local_timezone(timezone)
            .latest()
            .expect("Could not convert next midnight time to local timezone");

        let delta = next_midnight - now;
        let duration = delta
            .to_std()
            .expect("Duration to next midnight is less than 0");

        info!(
            "Next purge of expired rolling vouchers at midnight ({}), in {} hours and {} minutes...",
            timezone,
            delta.num_hours(),
            delta.num_minutes() % 60
        );

        sleep(duration).await;

        info!("Purging expired rolling vouchers...");
        match delete_expired_rolling_handler().await {
            Ok(response) => info!("Deleted {} rolling vouchers", response.vouchers_deleted),
            Err(code) => error!("Failed to delete rolling vouchers: {}", code),
        };
    }
}
