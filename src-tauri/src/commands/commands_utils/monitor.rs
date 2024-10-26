use crate::app_states::store_state::StoreState;
use crate::commands::payloads::service_error::ServiceError;
use crate::commands::ServiceResult;
use crate::constants::store_keys::{KEY_DAYS_LOGGED_IN, KEY_HIGHEST_CONSECUTIVE_DAYS, KEY_LAST_PLAYED_DAY};
use chrono::{Days, TimeZone, Utc};

pub fn consecutive_days_checker(store_state: &StoreState) -> ServiceResult<()> {
    let last_played: i64 = store_state.retrieve_default(KEY_LAST_PLAYED_DAY)?;
    let now = Utc::now();
    let saved = Utc.timestamp_millis_opt(last_played).unwrap();
    if now == saved {
        return Ok(())
    }
    if let Some(n) = now.checked_sub_days(Days::new(1)) {
        let current = if n.eq(&saved) {
            let current_consecutive: i32 = store_state.retrieve_default(KEY_DAYS_LOGGED_IN)?;
            let c = current_consecutive + 1;
            store_state.save(KEY_DAYS_LOGGED_IN, &c)?;
            store_state.save(KEY_LAST_PLAYED_DAY, &now.timestamp())?;
            c
        } else {
            store_state.save(KEY_DAYS_LOGGED_IN, &0)?;
            0
        };
        let highest: i32 = store_state.retrieve_default(KEY_HIGHEST_CONSECUTIVE_DAYS)?;
        if current > highest {
            store_state.save(KEY_HIGHEST_CONSECUTIVE_DAYS, &current)?;
        };
        Ok(())
    } else {
        Err(ServiceError::generic())
    }
}