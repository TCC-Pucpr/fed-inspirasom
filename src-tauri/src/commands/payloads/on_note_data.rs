use serde::{Deserialize, Serialize};
use strum::EnumIter;
use ts_rs::TS;

#[derive(Deserialize, Eq, PartialEq, TS, Copy, Clone, EnumIter)]
#[ts(export, export_to = "../../src/app/core/model/NotePressPrecision.ts")]
pub enum OnNotePrecision {
    Middle,
    Left,
    Right,
    Miss,
    EarlyMiss,
}

impl From<OnNotePrecision> for f32 {
    fn from(value: OnNotePrecision) -> Self {
        match value {
            OnNotePrecision::Middle => 2f32,
            OnNotePrecision::Left => 1.3f32,
            OnNotePrecision::Right => 1.5f32,
            OnNotePrecision::Miss => -0.3f32,
            OnNotePrecision::EarlyMiss => -0.8f32,
        }
    }
}

impl From<OnNotePrecision> for bool {
    fn from(value: OnNotePrecision) -> Self {
        match value {
            OnNotePrecision::Middle | OnNotePrecision::Left | OnNotePrecision::Right => true,
            OnNotePrecision::Miss | OnNotePrecision::EarlyMiss => false,
        }
    }
}

/// Payload que será emitido ao front sobre sempre que o usuario atualizar o seu score
/// o `total_score` é o score total acumulado na sessao atual da musica e o
/// `latest_message_score` é o score ganho/perdido depois do ultimo input.
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/app/core/model/OnScoreUpdateMessage.ts", rename = "OnScoreUpdateMessage")]
pub struct OnNotePayload {
    hit_streak: u32,
    total_score: i64,
    latest_message_score: i32,
}

impl OnNotePayload {
    pub fn new(hit_streak: u32, total_score: i64, latest_message_score: i32) -> Self {
        Self {
            hit_streak,
            total_score,
            latest_message_score,
        }
    }
}
