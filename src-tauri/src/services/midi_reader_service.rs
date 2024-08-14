use std::fs;

use crate::{constants::events_name::MIDI_READ_NOTE, services::service_error::ServiceError};

use super::{
    data_structs::{
        midi_signal::MidiPayload,
        music::{MidiMusic, MidiMusicList},
    },
    service_error::ServiceResult,
};

use midi_reader::reader_service::{MidiFile, PlayBackCallback};
use tauri::{Runtime, Window};

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
}

#[tauri::command]
pub fn list_musics<R: Runtime>(app: tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    music_list(&app)
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: String,
    handle: tauri::AppHandle<R>,
    window: Window,
) -> ServiceResult<()> {
    let (_, file) = read_music_from_id(&handle, music_id)?;
    let midi_reader = match MidiFile::from_bytes_vector(file) {
        Ok(mr) => mr,
        Err(err) => return Err(ServiceError::new_with_message(err.to_string())),
    };
    let sheet_listener = Box::new(SheetListener {
        window,
        ignore_note_errors: false,
    });
    let res = midi_reader.read_sheet(sheet_listener);
    if let Err(err) = res {
        Err(ServiceError::new_with_message(err.to_string()))
    } else {
        Ok(())
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
