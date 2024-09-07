use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::store_state::StoreState;
use crate::commands::payloads::on_note_data::{OnNoteMessage, OnNotePayload};
use crate::commands::service_error::ServiceResult;
use std::ops::Deref;
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
    music_id: &str,
    store: State<'_, StoreState>
) -> ServiceResult<()> {
    Ok(CurrentMusicScoreState::reset_attempts(music_id, store.deref())?)
}
