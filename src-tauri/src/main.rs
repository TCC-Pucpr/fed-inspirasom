// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::{DatabaseResult, DatabaseState};
use crate::app_states::monitoring_state::MonitoringState;
use crate::app_states::store_state::StoreState;
use crate::constants::dirs::{DATA_FOLDER, DB_NAME, RESOURCES_FOLDER, STORE_NAME};
use crate::constants::errors::{CodedError, COULDNT_GET_PATH};
use app_states::midi_device_state::MidiState;
use commands::{midi_connection_commands::*, midi_reader_commands::*, score_commands::*};
use persistence::storage::StorageResult;
use std::path::PathBuf;
use tauri::async_runtime::block_on;
use tauri::{App, AppHandle, Manager, Runtime};

mod app_states;
mod commands;
mod constants;

pub fn get_context_path(app: &App) -> Result<PathBuf, CodedError> {
    if let Some(p) = app.path_resolver().resolve_resource(RESOURCES_FOLDER) {
        Ok(p)
    } else {
        Err(COULDNT_GET_PATH)
    }
}

pub fn get_resources_path<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf, CodedError> {
    if let Some(p) = handle.path_resolver().resolve_resource(RESOURCES_FOLDER) {
        Ok(p)
    } else {
        Err(COULDNT_GET_PATH)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(MidiState::new())
        .manage(CurrentMusicScoreState::default())
        .manage(MonitoringState::default())
        .invoke_handler(tauri::generate_handler![
            start_listening_midi,
            connect_to_midi,
            disconnect_midi,
            list_midi_devices,
            start_game,
            end_game,
            pause_game,
            stop_game,
            resume_game,
            list_musics,
            music_length,
            remaining_time,
            on_note_played,
            reset_music_score,
            list_scores,
            add_new_music,
            remove_music,
            consecutive_days_played
        ])
        .setup(move |app| {
            let mut context_resources_path = get_context_path(app)?;
            context_resources_path.push(DATA_FOLDER);
            let store = create_storage(&context_resources_path)?;
            let db = create_db(&context_resources_path)?;
            drop(context_resources_path);
            app.manage(store);
            app.manage(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_storage(context_path: &PathBuf) -> StorageResult<StoreState> {
    let store_path = context_path.join(STORE_NAME);
    StoreState::try_from(store_path.display().to_string())
}

fn create_db(context_path: &PathBuf) -> DatabaseResult<DatabaseState> {
    let db_path: PathBuf = context_path.join(DB_NAME);
    block_on(DatabaseState::connect(&db_path.display().to_string()))
}
