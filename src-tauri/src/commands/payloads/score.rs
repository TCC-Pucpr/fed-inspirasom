use entity::score::Model;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
    pub finished: bool
}

impl From<Model> for ScorePayload {
    fn from(value: Model) -> Self {
        Self {
            total: value.total,
            date_achieved: value.date.format("%H:%M:%S | %d/%m/%Y").to_string(),
            highest_streak: value.highest_streak,
            finished: value.completed,
        }
    }
}
