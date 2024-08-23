use std::{
    fs,
    ops::{Deref, DerefMut},
};

use crate::{
    app_states::midi_device_state::MidiState,
    constants::events_name::{MIDI_READ_NOTE, MIDI_READ_STATE},
    services::service_error::ServiceError,
};

use super::{
    data_structs::{
        midi_payload::{MidiFileState, MidiPayload},
        music::{MidiMusic, MidiMusicList},
    },
    service_error::ServiceResult,
};

use midi_reader::midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback, ReadingState};
use tauri::{Runtime, State, Window};

const MUSICS_FOLDER: &str = "musics/";
const DATA_JSON: &str = "data.json";

struct SheetListener {
    window: Window,
    ignore_note_errors: bool,
}

impl PlayBackCallback for SheetListener {
    fn on_note(&self, on: bool, key: u8, vel: u8) -> bool {
        let payload = match MidiPayload::from_note(key, vel, on) {
            Some(p) => p,
            None => return !self.ignore_note_errors,
        };
        self.window.emit(MIDI_READ_NOTE, payload).is_ok()
    }

    fn on_interrupted(&self) {
        let _ = self
            .window
            .emit(MIDI_READ_STATE, MidiFileState::INTERRUPTED);
    }

    fn on_pause(&self) {
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::PAUSED);
    }

    fn on_finished(&self) {
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::FINISHED);
    }
}

#[tauri::command]
pub fn list_musics<R: Runtime>(app: tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    music_list(&app)
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: String,
    midi_state: State<'_, MidiState>,
    handle: tauri::AppHandle<R>,
    window: Window,
) -> ServiceResult<()> {
    if let Ok(Some(state)) = midi_state.midi_file.lock().as_deref() {
        match state.current_state() {
            ReadingState::Paused | ReadingState::Playing => {
                return Err(ServiceError::new_with_str("Already playing"));
            }
            _ => {}
        }
    };
    let (_, file) = read_music_from_id(&handle, music_id)?;
    let sheet_listener = SheetListener {
        window,
        ignore_note_errors: false,
    };
    let p = if let Ok(mut f) = midi_state.midi_file.lock() {
        let mut m = MidiFile::from_bytes_vector(file).unwrap();
        if let Ok(r) = m.create_sheet_player(sheet_listener) {
            *f = Some(m);
            r
        } else {
            return Err(ServiceError::generic());
        }
    } else {
        return Err(ServiceError::generic());
    };
    p.play()
        .map_err(|_| ServiceError::new_with_code(String::from("0002")))?;
    midi_state.reset_midi_file();
    Ok(())
}

#[tauri::command]
pub fn pause_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref_mut()
    {
        state.pause();
        Ok(())
    } else {
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn resume_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref_mut()
    {
        state.unpause();
        Ok(())
    } else {
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn stop_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .as_mut()
    {
        state.stop();
        Ok(())
    } else {
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn music_length(music_id: String, handle: tauri::AppHandle) -> ServiceResult<u64> {
    let (_, f) = read_music_from_id(&handle, music_id)?;
    let midi_file = MidiFile::from_bytes_vector(f).map_err(|_| ServiceError::generic())?;
    Ok(midi_file.file_length().as_secs())
}

#[tauri::command]
pub fn remaining_time(midi_state: State<MidiState>) -> ServiceResult<u64> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref()
    {
        let dur = state.remaining_time();
        Ok(dur.as_secs())
    } else {
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

fn read_music_from_id<R: Runtime>(
    handle: &tauri::AppHandle<R>,
    music_id: String,
) -> ServiceResult<(MidiMusic, Vec<u8>)> {
    let list = music_list(handle)?;
    if let Some(m) = list.files.iter().find(|e| e.id == music_id) {
        if let Some(vec) = music(handle, &m.directory) {
            return Ok((m.to_owned(), vec));
        };
    };
    Err(ServiceError::new_with_message(format!(
        "Music with id {} does not exist",
        music_id
    )))
}

fn music_list<R: Runtime>(handle: &tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(String::from(MUSICS_FOLDER) + DATA_JSON)
    {
        if let Ok(l) = MidiMusicList::from_path_resource(&p) {
            return Ok(l);
        }
    };
    Err(ServiceError::new_with_message(String::from(
        "Could not fetch music list",
    )))
}

fn music<R: Runtime>(handle: &tauri::AppHandle<R>, music_name: &str) -> Option<Vec<u8>> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(String::from(MUSICS_FOLDER) + music_name)
    {
        if let Ok(vec) = fs::read(&p) {
            return Some(vec);
        }
    };
    None
}
