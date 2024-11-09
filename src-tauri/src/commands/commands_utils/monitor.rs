use crate::app_states::store_state::StoreState;
use crate::commands::payloads::score::DailyScoreData;
use crate::commands::payloads::service_error::ServiceError;
use crate::commands::ServiceResult;
use crate::constants::store_keys::{KEY_DAYS_LOGGED_IN, KEY_HIGHEST_CONSECUTIVE_DAYS, KEY_LAST_PLAYED_DAY};
use chrono::{Days, NaiveTime, TimeZone, Utc};
use entity::prelude::Score;
use entity::score;
use paris::{error, info};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

const DAYS_TO_LOOK: usize = 7;
pub type ScoreDataInDays = Vec<DailyScoreData>;


#[derive(FromQueryResult, Debug)]
pub struct SumAndCountResult {
    pub sum: Option<i32>,
    pub count: i32
}

impl SumAndCountResult {
    pub fn avg(self) -> i32 {
        if let Some(a) = self.sum {
            a / self.count
        } else { 
            0
        }
    }
}

pub async fn days_data<C: ColumnTrait>(
    conn: &DatabaseConnection,
    column: C
) -> ServiceResult<Vec<(SumAndCountResult, DateTimeUtc)>> {
    let last_days = last_n_days(DAYS_TO_LOOK, Utc::now());
    let mut average_scores = Vec::with_capacity(DAYS_TO_LOOK);
    for day in last_days {
        let start = day.with_time(NaiveTime::MIN).unwrap();
        let res = sum_and_count_of(
            conn,
            column,
            score::Column::Date.between(start, day)
        ).await?;
        if let Some(avg_sum) = res {
            average_scores.push((avg_sum, day));
        }
    }
    Ok(average_scores)
}

pub async fn sum_and_count_of<C: ColumnTrait, F: IntoCondition>(
    db_conn: &DatabaseConnection,
    column: C,
    filter: F
) -> ServiceResult<Option<SumAndCountResult>> {
    let res = Score::find()
        .filter(filter)
        .select_only()
        .column_as(column.sum(), "sum")
        .column_as(column.count(), "count")
        .into_model::<SumAndCountResult>()
        .one(db_conn)
        .await?;
    Ok(res)
}

pub fn consecutive_days_checker(store_state: &StoreState) -> ServiceResult<()> {
    let last_played: i64 = store_state.retrieve_default(KEY_LAST_PLAYED_DAY)?;
    info!("Last day played: {}", last_played);
    let now = Utc::now();
    let saved = Utc.timestamp_millis_opt(last_played).unwrap();
    if now == saved {
        return Ok(())
    }
    if let Some(n) = now.checked_sub_days(Days::new(1)) {
        let current = if n.eq(&saved) {
            let current_consecutive: i32 = store_state.retrieve_default(KEY_DAYS_LOGGED_IN)?;
            info!("Last day played was yesterday, incrementing current consecutive days ({})", current_consecutive);
            let c = current_consecutive + 1;
            store_state.save(KEY_DAYS_LOGGED_IN, &c)?;
            store_state.save(KEY_LAST_PLAYED_DAY, &now.timestamp())?;
            c
        } else {
            info!("Last day played was not yesterday, resetting to 1");
            store_state.save(KEY_DAYS_LOGGED_IN, &1)?;
            1
        };
        let highest: i32 = store_state.retrieve_default(KEY_HIGHEST_CONSECUTIVE_DAYS)?;
        if current > highest {
            store_state.save(KEY_HIGHEST_CONSECUTIVE_DAYS, &current)?;
        };
        store_state.commit()?;
        Ok(())
    } else {
        error!("Unexpected error while saving consecutive days played");
        Err(ServiceError::generic())
    }
}

pub fn last_n_days(
    days: usize,
    current_day: DateTimeUtc
) -> Vec<DateTimeUtc> {
    let mut v = Vec::with_capacity(days);
    v.push(current_day);
    let mut previous_day = current_day.checked_sub_days(Days::new(1));
    let mut days_returned = 1;
    while previous_day.is_some() && days_returned < days {
        let prev = previous_day.unwrap();
        v.push(prev);
        previous_day = prev.checked_sub_days(Days::new(1));
        days_returned += 1;
    }
    v
}