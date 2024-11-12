use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::app_states::monitoring_state::MonitoringState;
use crate::commands::payloads::music::MidiMusicList;
use crate::commands::payloads::service_error::ServiceResult;
use crate::constants::errors::FILE_ID_NOT_FOUND;
use anyhow::{anyhow, Error};
use entity::music::Model;
use entity::prelude::Music;
use entity::score::ActiveModel;
use paris::warn;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use std::ops::Deref;
use tauri::State;

pub async fn get_music(music_id: i32, db_state: &State<'_, DatabaseState>) -> ServiceResult<Model> {
    if let Some(music) = Music::find_by_id(music_id).one(&db_state.db).await? {
        Ok(music)
    } else {
        Err(FILE_ID_NOT_FOUND.into())
    }
}

pub trait ScoreSaver {
    async fn save_score(
        &self,
        finished: bool,
        music_id: i32,
        score_state: &CurrentMusicScoreState,
        monitor_state: &MonitoringState
    ) -> Result<(), Error>;
}

impl ScoreSaver for DatabaseState {
    async fn save_score(
        &self, 
        finished: bool, 
        music_id: i32, 
        score_state: &CurrentMusicScoreState, 
        monitor_state: &MonitoringState
    ) -> Result<(), Error> {
        let model = if let Ok(score) = score_state.score.lock() {
            if let Ok(monitor) = monitor_state.data.lock() {
                let model = ActiveModel {
                    total: ActiveValue::Set(score.total_score),
                    date: ActiveValue::Set(Utc::now()),
                    completed: ActiveValue::Set(finished),
                    highest_streak: ActiveValue::Set(score.highest_streak as i32),
                    music_id: ActiveValue::Set(music_id),
                    ..Default::default()
                };
                if let Some(m_data) = monitor.deref() {
                    ActiveModel {
                        breath_average_strength: ActiveValue::Set(Some(m_data.average_strength)),
                        total_close_hits: ActiveValue::Set(Some(m_data.total_close_scores as i32)),
                        total_great_hits: ActiveValue::Set(Some(m_data.total_great_scores as i32)),
                        total_ok_hits: ActiveValue::Set(Some(m_data.total_ok_scores as i32)),
                        highest_breathing_duration: ActiveValue::Set(Some(m_data.highest_breath_time.as_secs() as i32)),
                        total_breathing_duration: ActiveValue::Set(Some(m_data.total_breath_time.as_secs() as i32)),
                        total_early_misses: ActiveValue::Set(Some(m_data.total_early_misses as i32)),
                        total_misses: ActiveValue::Set(Some(m_data.total_misses as i32)),
                        ..model
                    }
                } else {
                    warn!("No monitoring data recorded");
                    model
                }
            } else {
                return Err(anyhow!("Could not acquire monitoring data"))
            }
        } else {
            return Err(anyhow!("Could not acquire score data"))
        };
        model.insert(&self.db).await?;
        Ok(())
    }
}

pub async fn music_list(db_state: &DatabaseState) -> anyhow::Result<MidiMusicList> {
    let a = Music::find().all(&db_state.db).await?;
    Ok(MidiMusicList::from(a))
}