// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::store_state::StoreState;
use app_states::midi_device_state::MidiState;
use commands::{midi_connection_commands::*, midi_reader_commands::*, score_commands::*};
use std::path::PathBuf;
use tauri::{App, Manager};

mod app_states;
mod commands;
mod constants;
mod services;

pub const RESOURCES_FOLDER: &str = "resources/";

pub fn get_resources_path(app: &App) -> PathBuf {
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
            on_note
        ])
        .setup(move |app| {
            let store = StoreState::try_from(app as &App)?;
            app.manage(store);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
