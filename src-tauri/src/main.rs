// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{State, Window};
use waitgroup::{WaitGroup, Worker};

use arduino_comm::midi_connection::connect;

use crate::constants::events_name::MIDI_NOTE;

mod constants;

struct MidiState {
    worker: Mutex<Option<Worker>>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_midi_device(window: Window, state: State<'_, MidiState>) -> Result<(), ()> {
    if state.worker.lock().as_ref().unwrap().is_some() {
        return Ok(())
    }
    let wg = WaitGroup::new();
    *state.worker.lock().unwrap() = Some(wg.worker());
    let _conn = connect(move |wrapper| {
        println!("{} - {} - {:?}", wrapper.state, wrapper.air_strength, wrapper.note);
        window.emit(MIDI_NOTE, wrapper).expect("Could not send midi event!");
    });
    wg.wait().await;
    Ok(())
}

#[tauri::command]
fn stop_midi(state: State<MidiState>) -> bool {
    *state.worker.lock().unwrap() = None;
    true
}

fn main() {
    tauri::Builder::default()
        .manage(MidiState { worker: Default::default() })
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_midi_device,
            stop_midi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
