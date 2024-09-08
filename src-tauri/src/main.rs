// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::app_states::store_state::StoreState;
use crate::constants::dirs::RESOURCES_FOLDER;
use app_states::midi_device_state::MidiState;
use commands::{midi_connection_commands::*, midi_reader_commands::*, score_commands::*};
use std::path::PathBuf;
use tauri::async_runtime::block_on;
use tauri::{App, Manager};

mod app_states;
mod commands;
mod constants;

pub fn get_context_path(app: &App) -> PathBuf {
    app.path_resolver()
        .resolve_resource(RESOURCES_FOLDER)
        .unwrap()
}

fn main() {
    tauri::Builder::default()
        .manage(MidiState::new())
        .manage(CurrentMusicScoreState::default())
        .invoke_handler(tauri::generate_handler![
            start_listening_midi,
            disconnect_midi,
            list_midi_devices,
            start_game,
            pause_game,
            stop_game,
            resume_game,
            list_musics,
            music_length,
            remaining_time,
            on_note,
            reset_music_score
        ])
        .setup(move |app| {
            let context_resources_path = get_context_path(app).display().to_string();
            let store = StoreState::try_from(context_resources_path.as_str())?;
            let db = block_on(DatabaseState::connect(context_resources_path.as_str()))?;
            app.manage(store);
            app.manage(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
