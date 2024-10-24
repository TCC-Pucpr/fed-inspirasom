use crate::app_states::store_state::StoreState;
use crate::commands::payloads::service_error::ServiceError;
use crate::commands::ServiceResult;
use crate::constants::store_keys::{KEY_DAYS_LOGGED_IN, KEY_LAST_PLAYED_DAY};
use chrono::{Days, TimeZone, Utc};

pub fn consecutive_days_checker(store_state: &StoreState) -> ServiceResult<()> {
    let last_played: String = store_state.retrieve_default(KEY_LAST_PLAYED_DAY)?;
    let now = Utc::now();
    let epoch_last_played = if let Ok(n) = last_played.parse::<i64>() {
        n
    } else {
        0
    };
    let saved = Utc.timestamp_millis_opt(epoch_last_played).unwrap();
    if now == saved {
        return Ok(())
    }
    if let Some(n) = now.checked_sub_days(Days::new(1)) {
        if n.eq(&saved) {
            let current_consecutive: i32 = store_state.retrieve_default(KEY_DAYS_LOGGED_IN)?;
            store_state.save(KEY_DAYS_LOGGED_IN, &(current_consecutive + 1))?;
            store_state.save(KEY_LAST_PLAYED_DAY, &now.timestamp())?;
            Ok(())
        } else {
            store_state.save(KEY_DAYS_LOGGED_IN, &0)?;
            Ok(())
        }
    } else {
        Err(ServiceError::generic())
    }
}