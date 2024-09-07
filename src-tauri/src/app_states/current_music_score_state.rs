use crate::app_states::store_state::{StorageError, StorageResult, StoreState};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Mutex;

const DEFAULT_SCORE: i64 = 10;
const SCORE_KEY: &str = "__score";
const NUMBER_OF_ATTEMPTS_KEY: &str = "__attempts";

#[derive(Default)]
pub struct CurrentMusicScoreState {
    pub score: Mutex<CurrentMusicScore>,
}

impl CurrentMusicScoreState {
    pub fn save_to_store(&self, music_id: &str, store_state: &StoreState) -> StorageResult<()> {
        match self.score.lock() {
            Ok(score) => {
                let key = Self::get_music_id_key(music_id, SCORE_KEY);
                Self::append_attempts(music_id, store_state)?;
                store_state.set_value(&key, score.deref())
            }
            Err(e) => Err(StorageError::DbWriteError {
                key: SCORE_KEY.to_string(),
                source: anyhow!(e.to_string()),
            }),
        }
    }
    fn get_music_id_key(music_id: &str, key_type: &str) -> String {
        format!("{}{}", music_id, key_type)
    }
    pub fn append_attempts(music_id: &str, store_state: &StoreState) -> StorageResult<()> {
        let key = Self::get_music_id_key(music_id, NUMBER_OF_ATTEMPTS_KEY);
        let current: i64 = store_state.get_or_default(&key);
        store_state.set_value(&key, &(current + 1))
    }
    pub fn reset_attempts(music_id: &str, store_state: &StoreState) -> StorageResult<()> {
        store_state.remove_value(&Self::get_music_id_key(music_id, NUMBER_OF_ATTEMPTS_KEY))
    }
    pub fn reset(&self) {
        if let Ok(mut score) = self.score.lock() {
            score.reset()
        }
    }
    pub fn add_to_total_score(&self, multiplier: f32, is_miss: bool) -> (i64, i64) {
        if let Ok(mut score) = self.score.lock() {
            if is_miss {
                score.hit_streak = 0;
            } else {
                score.hit_streak += 1;
            };
            let score_to_add = DEFAULT_SCORE * (score.hit_streak as i64) * (multiplier as i64);
            score.total_score += score_to_add;
            (score.total_score, score_to_add)
        } else {
            (0, 0)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CurrentMusicScore {
    total_score: i64,
    hit_streak: u64,
}

impl CurrentMusicScore {
    pub fn reset(&mut self) {
        self.hit_streak = u64::default();
        self.total_score = i64::default();
    }
}
