use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::commands::payloads::on_note_data::{OnNoteMessage, OnNotePayload};
use crate::commands::service_error::ServiceResult;
use tauri::State;

#[tauri::command]
pub async fn on_note(
    on_note_message: OnNoteMessage,
    current_music_score: State<'_, CurrentMusicScoreState>,
) -> ServiceResult<OnNotePayload> {
    let (new_total_score, gained_score) = current_music_score.add_to_total_score(
        f32::from(on_note_message.precision),
        !bool::from(on_note_message.precision),
    );
    Ok(OnNotePayload::new(new_total_score, gained_score as i32))
}
