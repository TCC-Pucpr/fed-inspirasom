use entity::score::Model;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub fn format_date(date: DateTimeUtc) -> String {
    date.format("%d/%m").to_string()
}

#[derive(Debug, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/app/core/model/ScoreOrder.ts",
    rename = "ScoreOrderType"
)]
pub enum OrderType {
    DATE,
    SCORE,
    STREAK,
    NONE,
}
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/app/core/model/DailyScoreData.ts",
    rename = "DailyScoreData"
)]
pub struct DailyScoreData {
    pub date: String,
    pub data: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/app/core/model/Score.ts",
    rename = "Score"
)]
pub struct ScorePayload {
    pub total: i32,
    pub date_achieved: String,
    pub highest_streak: i32,
    pub finished: bool,
}

impl From<Model> for ScorePayload {
    fn from(value: Model) -> Self {
        Self {
            total: value.total,
            date_achieved: format_date(value.date),
            highest_streak: value.highest_streak,
            finished: value.completed,
        }
    }
}
