use persistence::storage::{StorageResult, StorageSavable, Store};
use serde::ser::Error;
use serde::{Serialize, Serializer};
use std::sync::Mutex;

const DEFAULT_SCORE: Score = 10;
const SCORE_KEY: &str = "__score";
const NUMBER_OF_ATTEMPTS_KEY: &str = "__attempts";

type Score = i32;

#[derive(Default)]
pub struct CurrentMusicScore {
    pub total_score: Score,
    pub hit_streak: u32,
    pub highest_streak: u32,
}

#[derive(Default)]
pub struct CurrentMusicScoreState {
    pub score: Mutex<CurrentMusicScore>,
}

impl StorageSavable for CurrentMusicScore {}
impl StorageSavable for CurrentMusicScoreState {
    fn transform_key(&self, store: &mut Store, key: &str) -> StorageResult<String> {
        let attempts_key = Self::get_music_id_key(key, NUMBER_OF_ATTEMPTS_KEY);
        let current: i64 = store.get_or_default(&attempts_key);
        let k = format!("{}{}", Self::get_music_id_key(key, SCORE_KEY), current + 1);
        Ok(k)
    }
    fn transform_value(&self, store: &mut Store, key: &str) -> StorageResult<&Self> {
        let attempts_key = Self::get_music_id_key(key, NUMBER_OF_ATTEMPTS_KEY);
        let current: i64 = store.get_or_default(&attempts_key);
        store.set_value(&attempts_key, &(current + 1))?;
        Ok(self)
    }
}

impl CurrentMusicScoreState {
    fn get_music_id_key(music_id: &str, key_type: &str) -> String {
        format!("{}{}", music_id, key_type)
    }
    pub fn reset(&self) {
        if let Ok(mut score) = self.score.lock() {
            *score = Default::default();
        }
    }
    pub fn add_to_total_score(&self, multiplier: f32, is_miss: bool) -> (Score, Score, u32) {
        if let Ok(mut score) = self.score.lock() {
            if is_miss {
                score.hit_streak = 0;
            } else {
                score.hit_streak += 1;
                if score.hit_streak > score.highest_streak {
                    score.highest_streak = score.highest_streak;
                }
            };
            let score_to_add = DEFAULT_SCORE * (score.hit_streak as Score) * (multiplier as Score);
            score.total_score += score_to_add;
            (score.total_score, score_to_add, score.hit_streak)
        } else {
            (0, 0, 0)
        }
    }
}

impl Serialize for CurrentMusicScoreState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(score) = self.score.lock() {
            score.serialize(serializer)
        } else {
            Err(Error::custom("mutex poisoned"))
        }
    }
}

impl Serialize for CurrentMusicScore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Score::serialize(&self.total_score, serializer)
    }
}
