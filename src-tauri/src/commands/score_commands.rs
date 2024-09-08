use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::commands::payloads::on_note_data::{OnNoteMessage, OnNotePayload};
use crate::commands::payloads::score::{OrderType, ScorePayload};
use crate::commands::payloads::service_error::{ServiceError, ServiceResult};
use entity::prelude::Score;
use entity::{music, score};
use migration::Order;
use sea_orm::{EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use tauri::State;

#[tauri::command]
pub async fn on_note(
    on_note_message: OnNoteMessage,
    current_music_score: State<'_, CurrentMusicScoreState>,
) -> ServiceResult<OnNotePayload> {
    let (new_total_score, gained_score, hit_streak) = current_music_score.add_to_total_score(
        f32::from(on_note_message.precision),
        !bool::from(on_note_message.precision),
    );
    Ok(OnNotePayload::new(
        hit_streak,
        new_total_score,
        gained_score as i32,
    ))
}

#[tauri::command]
pub async fn reset_music_score(
    music_id: i32,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<()> {
    let music = if let Some(m) = music::Entity::find_by_id(music_id)
        .one(&db_state.db)
        .await?
    {
        m
    } else {
        return Err(ServiceError::from("Music does not exist"));
    };
    let res = score::Entity::delete_many()
        .belongs_to(&music)
        .exec(&db_state.db)
        .await?;
    if res.rows_affected > 0 {
        Ok(())
    } else {
        let msg = format!("Music with id {} doesnt have any scores", music_id);
        Err(ServiceError::from(msg))
    }
}

#[tauri::command]
pub async fn list_scores(
    music_id: i32,
    order_type: OrderType,
    ascending: bool,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<Vec<ScorePayload>> {
    let music_model = if let Some(m) = music::Entity::find_by_id(music_id)
        .one(&db_state.db)
        .await?
    {
        m
    } else {
        let msg = format!("Music with id {} doesnt exist", music_id);
        return Err(ServiceError::from(msg));
    };
    let related_q = music_model.find_related(Score);
    let order = if ascending { Order::Asc } else { Order::Desc };
    let related_q = match order_type {
        OrderType::DATE => related_q.order_by(score::Column::Date, order),
        OrderType::SCORE => related_q.order_by(score::Column::Total, order),
        OrderType::STREAK => related_q.order_by(score::Column::HighestStreak, order),
    };
    let res = related_q.all(&db_state.db).await?;
    let mut v: Vec<ScorePayload> = Vec::with_capacity(res.len());
    for r in res {
        v.push(ScorePayload::from(r));
    }
    Ok(v)
}
