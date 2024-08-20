use std::fs;

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

use midi_reader::reader_service::{MidiFile, MidiFilePlayer, PlayBackCallback};
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
    let (_, file) = read_music_from_id(&handle, music_id)?;
    let file = match MidiFile::from_bytes_vector(file) {
        Ok(mr) => mr,
        Err(err) => return Err(ServiceError::new_with_message(err.to_string())),
    };
    midi_state.update_midi_file(Some(file));
    let mut midi_file = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?;
    let midi_reader = if let Some(mr) = midi_file.as_mut() {
        mr
    } else {
        return Err(ServiceError::generic());
    };
    let sheet_listener = SheetListener {
        window,
        ignore_note_errors: false,
    };
    let res = midi_reader.read_sheet(sheet_listener);
    midi_state.update_midi_file(None);
    if let Err(err) = res {
        Err(ServiceError::new_with_message(err.to_string()))
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn pause_game(midi_state: State<MidiState>) -> Result<(), ServiceError> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .as_mut()
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
pub fn resume_game(midi_state: State<MidiState>) -> Result<(), ServiceError> {
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .as_mut()
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
pub fn stop_game(midi_state: State<MidiState>) -> Result<(), ServiceError> {
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
