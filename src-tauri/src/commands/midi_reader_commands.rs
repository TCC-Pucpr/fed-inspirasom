use super::payloads::{
    midi_payload::MidiFileState,
    music::{MidiMusic, MidiMusicList},
};
use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::app_states::monitoring_state::MonitoringState;
use crate::commands::commands_utils::database_queries::music_list;
use crate::commands::commands_utils::midi_file_utils::{check_midi_file, end_game as finish, load_file, play_game, read_music_from_id, SheetListener};
use crate::commands::payloads::service_error::ServiceResult;
use crate::constants::errors::{FILE_COULD_NOT_READ_PATH, FILE_ID_NOT_FOUND, FILE_NAME_ALREADY_EXIST, FILE_NOT_FOUND};
use crate::{
    app_states::midi_device_state::MidiState,
    constants::events_name::MIDI_READ_STATE,
    get_resources_path,
};
use convert_case::{Case, Casing};
use entity::prelude::{Music, Score};
use entity::{music, score};
use midi_reader::midi_file::{MidiFile, MidiFilePlayer};
use paris::{info, success, Logger};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use std::fs;
use std::fs::exists;
use tauri::{AppHandle, Runtime, State, Window};

#[tauri::command]
pub async fn list_musics(db_state: State<'_, DatabaseState>) -> ServiceResult<MidiMusicList> {
    info!("Fetching music list...");
    let list = music_list(&db_state).await?;
    info!("List fetched: {:?}", list);
    Ok(list)
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: i32,
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    db_state: State<'_, DatabaseState>,
    handle: AppHandle<R>,
    window: Window,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    midi_state.is_playing_midi_file()?;
    let file = load_file(music_id, &*db_state, handle, &mut logger).await?;
    let p = midi_state.create_new_file_player(
        music_id,
        file,
        SheetListener::new(&window, true)
    )?;
    score_state.reset();
    let _ = window.emit(MIDI_READ_STATE, MidiFileState::PLAYING);
    play_game(p, &mut logger)?;
    Ok(())
}

#[tauri::command]
pub async fn end_game(
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    db_state: State<'_, DatabaseState>,
    monitor_state: State<'_, MonitoringState>,
) -> ServiceResult<()> {
    finish(true, midi_state, score_state, db_state, monitor_state).await
}

#[tauri::command]
pub async fn pause_game(
    midi_state: State<'_, MidiState>,
    window: Window
) -> ServiceResult<()> {
    info!("Pause called...");
    midi_state.change_file_state(MidiFileState::PAUSED)?;
    window.emit(MIDI_READ_STATE, MidiFileState::PAUSED)?;
    success!("Midi file playback paused successfully");
    Ok(())
}

#[tauri::command]
pub async fn resume_game(
    midi_state: State<'_, MidiState>,
    window: Window,
) -> ServiceResult<()> {
    info!("Resume called...");
    midi_state.change_file_state(MidiFileState::PLAYING)?;
    window.emit(MIDI_READ_STATE, MidiFileState::PLAYING)?;
    success!("Midi file playback resumed successfully");
    Ok(())
}

#[tauri::command]
pub async fn stop_game(
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    db_state: State<'_, DatabaseState>,
    monitor_state: State<'_, MonitoringState>,
) -> ServiceResult<()> {
    info!("Stop called...");
    midi_state.change_file_state(MidiFileState::INTERRUPTED)?;
    success!("Midi file playback stopped successfully");
    finish(false, midi_state, score_state, db_state, monitor_state).await
}

#[tauri::command]
pub async fn music_length(
    music_id: i32,
    db_state: State<'_, DatabaseState>,
    handle: AppHandle,
) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.info("Calculating midi file length...");
    let (_, f) = read_music_from_id(&db_state, &handle, music_id).await?;
    let midi_file = MidiFile::from_bytes_vector(f)?;
    let length = midi_file.file_length().as_secs();
    logger.success(format!("Successfully calculated length: {}", length));
    Ok(length)
}

#[tauri::command]
pub async fn remaining_time(midi_state: State<'_, MidiState>) -> ServiceResult<u64> {
    info!("Reading remaining time...");
    let dur = midi_state.time_left()?.as_secs();
    success!("Remaining time left: {} seconds", dur);
    Ok(dur)
}

#[tauri::command]
pub async fn add_new_music<R: Runtime>(
    music_name: &str,
    file_path: &str,
    app_handle: AppHandle<R>,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<MidiMusic> {
    if let Ok(e) = exists(file_path) {
        if !e {
            return Err(FILE_NOT_FOUND.into());
        }
        if Music::find()
            .filter(music::Column::Name.eq(music_name))
            .one(&db_state.db)
            .await
            .is_ok_and(move |t| t.is_some())
        {
            return Err(FILE_NAME_ALREADY_EXIST.into());
        }
        let mut path = get_resources_path(&app_handle)?;
        path.push(music_name.to_case(Case::Snake));
        let dur = check_midi_file(file_path)?;
        fs::copy(file_path, &path)?;
        let model = music::ActiveModel {
            id: Default::default(),
            name: ActiveValue::Set(music_name.to_string()),
            duration: ActiveValue::Set(dur as i32),
            directory: ActiveValue::Set(path.display().to_string()),
        };
        let new = model.insert(&db_state.db).await?;
        Ok(new.into())
    } else {
        Err(FILE_COULD_NOT_READ_PATH.into())
    }
}

#[tauri::command]
pub async fn remove_music<R: Runtime>(
    music_id: i32,
    app_handle: AppHandle<R>,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    let Some(music) = Music::find_by_id(music_id).one(&db_state.db).await? else {
        return Err(FILE_ID_NOT_FOUND.into());
    };
    let mut p = get_resources_path(&app_handle)?;
    p.push(&music.directory);
    if let Err(_) = fs::remove_file(&p) {
        logger.error(format!(
            "Could not remove midi file at {}, continuing removal from database...",
            p.display().to_string()
        ));
    }
    drop(p);
    logger.info("Starting removal of music and its scores...");
    let txn = db_state.db.begin().await?;
    Score::delete_many().filter(score::Column::MusicId.eq(music_id)).exec(&txn).await?;
    music.delete(&txn).await?;
    txn.commit().await?;
    logger.done().success(format!("Midi file with id {} removed", music_id));
    Ok(())
}
