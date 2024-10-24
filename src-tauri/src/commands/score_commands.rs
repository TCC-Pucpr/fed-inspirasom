use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::app_states::monitoring_state::MonitoringState;
use crate::app_states::store_state::StoreState;
use crate::commands::commands_utils::database_queries::get_music;
use crate::commands::payloads::on_note_data::OnNotePayload;
use crate::commands::payloads::score::{OrderType, ScorePayload};
use crate::commands::payloads::service_error::ServiceResult;
use crate::commands::OnNotePrecision;
use crate::constants::errors::{DATABASE_NO_VALUES_FOUND, INVALID_PARAMETER};
use crate::constants::store_keys::KEY_DAYS_LOGGED_IN;
use entity::prelude::Score;
use entity::score;
use migration::Order;
use paris::error;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use strum::IntoEnumIterator;
use tauri::State;

#[tauri::command]
pub async fn consecutive_days_played(
    store_state: State<'_, StoreState>
) -> ServiceResult<usize> {
    let n: String = store_state.retrieve_default(KEY_DAYS_LOGGED_IN)?;
    Ok(n)
}

#[tauri::command]
pub async fn on_note_played(
    on_note_message: usize,
    current_music_score: State<'_, CurrentMusicScoreState>,
    monitoring_state: State<'_, MonitoringState>,
) -> ServiceResult<OnNotePayload> {
    let mut iter = OnNotePrecision::iter();
    if on_note_message >= iter.len() {
        return Err(INVALID_PARAMETER.into());
    }
    let precision = iter.nth(on_note_message).unwrap();
    let (new_total_score, gained_score, hit_streak) = current_music_score.add_to_total_score(
        f32::from(precision),
        !bool::from(precision),
    );
    monitoring_state.receive_score(precision)?;
    Ok(OnNotePayload::new(
        hit_streak,
        new_total_score as i64,
        gained_score as i32,
    ))
}

#[tauri::command]
pub async fn reset_music_score(
    music_id: i32,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<()> {
    let music = get_music(music_id, &db_state).await?;
    let res = Score::delete_many().belongs_to(&music).exec(&db_state.db).await?;
    if res.rows_affected <= 0 {
        error!("Music with id {} doesnt have any scores", music_id);
        Err(DATABASE_NO_VALUES_FOUND.into())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn list_scores(
    music_id: i32,
    order_type: OrderType,
    ascending: Option<bool>,
    completed: Option<bool>,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<Vec<ScorePayload>> {
    let music_model = get_music(music_id, &db_state).await?;
    let related = music_model.find_related(Score);
    let query = if let Some(asc) = ascending {
        let order = if asc { Order::Asc } else { Order::Desc };
        match order_type {
            OrderType::DATE => related.order_by(score::Column::Date, order),
            OrderType::SCORE => related.order_by(score::Column::Total, order),
            OrderType::STREAK => related.order_by(score::Column::HighestStreak, order),
            _ => related
        }
    } else {
        related
    };
    let query = if let Some(completed) = completed {
        query.filter(score::Column::Completed.eq(completed))
    } else {
        query
    };
    let res = query.all(&db_state.db).await?;
    let mut v: Vec<ScorePayload> = Vec::with_capacity(res.len());
    for r in res {
        v.push(ScorePayload::from(r));
    }
    Ok(v)
}
