use std::sync::Mutex;

const DEFAULT_SCORE: i64 = 10;

#[derive(Default)]
pub struct CurrentMusicScoreState {
    pub score: Mutex<CurrentMusicScore>,
}

impl CurrentMusicScoreState {
    pub fn add_to_total_score(&self, multipler: f32, is_miss: bool) -> (i64, i64) {
        if let Ok(mut score) = self.score.lock() {
            if is_miss {
                score.hit_streak = 0;
            } else {
                score.hit_streak += 1;
            };
            let score_to_add = DEFAULT_SCORE * (score.hit_streak as i64) * (multipler as i64);
            score.total_score += score_to_add;
            (score.total_score, score_to_add)
        } else {
            (0, 0)
        }
    }
}

#[derive(Default)]
pub struct CurrentMusicScore {
    total_score: i64,
    hit_streak: u64,
}

impl CurrentMusicScore {
    pub fn new() -> Self {
        Self::default()
    }
}
