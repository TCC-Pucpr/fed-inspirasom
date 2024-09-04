// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_states::midi_device_state::MidiState;
use services::{midi_connection::*, midi_reader_service::*};
use tauri::Manager;

mod app_states;
mod constants;
mod services;

pub const RESOURCES_FOLDER: &str = "resources/";

fn main() {
    tauri::Builder::default()
        .manage(MidiState::new())
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
            remaining_time
        ])
        .setup(move |app| {
            // app.manage(StoreState::from(app as &App));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
