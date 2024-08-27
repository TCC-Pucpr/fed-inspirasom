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
use paris::{error, info, warn, Logger};
use tauri::{Runtime, State, Window};

const RESOURCES_FOLDER: &str = "resources/";
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
            None => {
                if self.ignore_note_errors {
                    warn!("Midi music sent invalid note, skipping note...");
                    return true;
                } else {
                    error!("Midi music sent invalid note, throwing error!");
                    return false;
                }
            }
        };
        info!("Emitting MidiPayload: {}", payload);
        self.window.emit(MIDI_READ_NOTE, payload).is_ok()
    }

    fn on_interrupted(&self) {
        info!("Emitting Interrupted Midi State");
        let _ = self
            .window
            .emit(MIDI_READ_STATE, MidiFileState::INTERRUPTED);
    }

    fn on_pause(&self) {
        info!("Emitting Paused Midi State");
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::PAUSED);
    }

    fn on_finished(&self) {
        info!("Emitting Finished Midi State");
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::FINISHED);
    }
}

#[tauri::command]
pub fn list_musics<R: Runtime>(app: tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    let list = music_list(&app);
    info!("List fetched: {:?}", list);
    list
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: String,
    midi_state: State<'_, MidiState>,
    handle: tauri::AppHandle<R>,
    window: Window,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Loading new file...");
    if let Ok(Some(state)) = midi_state.midi_file.lock().as_deref() {
        match state.current_state() {
            ReadingState::Paused | ReadingState::Playing => {
                logger.error("There is already an active music being played");
                return Err(ServiceError::new_with_str("Already playing"));
            }
            _ => {}
        }
    };
    let (music, file) = read_music_from_id(&handle, music_id)?;
    let msg = format!("Music found: {}", music);
    logger.done().info(msg);
    let sheet_listener = SheetListener {
        window,
        ignore_note_errors: false,
    };
    logger.loading("Loading file bytes...");
    let p = if let Ok(mut f) = midi_state.midi_file.lock() {
        let mut m = MidiFile::from_bytes_vector(file).unwrap();
        if let Ok(r) = m.create_sheet_player(sheet_listener) {
            *f = Some(m);
            r
        } else {
            let msg = String::from("Error occurred while creating a sheet player");
            logger.done().error(msg.clone());
            return Err(ServiceError::from(msg));
        }
    } else {
        let msg =
            String::from("Error occurred while unlocking midi file state, this is very suspicious");
        logger.done().error(msg.clone());
        return Err(ServiceError::from(msg));
    };
    logger
        .done()
        .loading("Successfully loaded file, now playing...");
    match p.play() {
        Ok(_) => {
            logger.done().info("Music finished playing");
        }
        Err(err) => {
            let msg = format!("Error while playing song: {}", err);
            logger.done().error(msg.clone());
            return Err(ServiceError::new("0002".to_string(), msg));
        }
    };
    midi_state.reset_midi_file();
    logger.info("Midi file state has been reset!");
    Ok(())
}

#[tauri::command]
pub fn pause_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Pause called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref_mut()
    {
        state.pause();
        logger.done().info("Midi file playback paused successfully");
        Ok(())
    } else {
        logger.done().error(
            "Could not acquire midi file state, probably because there is no file being played",
        );
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn resume_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Resume called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref_mut()
    {
        state.unpause();
        logger
            .done()
            .info("Midi file playback resumed successfully");
        Ok(())
    } else {
        logger.done().error(
            "Could not acquire midi file state, probably because there is no file being played",
        );
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn stop_game(midi_state: State<MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Stop called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .as_mut()
    {
        state.stop();
        logger
            .done()
            .info("Midi file playback stopped successfully");
        Ok(())
    } else {
        logger.done().error(
            "Could not acquire midi file state, probably because there is no file being played",
        );
        Err(ServiceError::new_with_message(String::from(
            "There is no file being played",
        )))
    }
}

#[tauri::command]
pub fn music_length(music_id: String, handle: tauri::AppHandle) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Calculating midi file length...");
    let (_, f) = match read_music_from_id(&handle, music_id) {
        Ok(f) => f,
        Err(err) => {
            let msg = format!("Error while fetching midi file bytes: {}", err);
            logger.done().error(msg);
            return Err(err);
        }
    };
    let midi_file = match MidiFile::from_bytes_vector(f) {
        Ok(mf) => mf,
        Err(err) => {
            let msg = format!("Error while reading midi file: {}", err);
            logger.done().error(msg.clone());
            return Err(ServiceError::from(msg));
        }
    };
    let length = midi_file.file_length().as_secs();
    let msg = format!("Successfully calculated length: {}", length);
    logger.done().info(msg);
    Ok(length)
}

#[tauri::command]
pub fn remaining_time(midi_state: State<MidiState>) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Reading remaining time...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| ServiceError::generic())?
        .deref()
    {
        let dur = state.remaining_time().as_secs();
        let msg = format!("Remaining time obtained: {} seconds left", dur);
        logger.done().info(msg);
        Ok(dur)
    } else {
        logger.done().error(
            "Could not acquire midi file state, probably because there is no file being played",
        );
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
            info!("Successfully obtained music with id {}", music_id);
            return Ok((m.to_owned(), vec));
        } else {
            let msg = format!(
                "Error while fetching music with id {} or the music does not exist",
                music_id
            );
            error!("{}", msg);
            return Err(ServiceError::new_with_message(msg));
        }
    };
    Err(ServiceError::new_with_message(format!(
        "Music with id {} does not exist",
        music_id
    )))
}

fn music_list<R: Runtime>(handle: &tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(DATA_JSON))
    {
        info!("Resources folder found, reading music list...");
        return match MidiMusicList::from_path_resource(&p) {
            Ok(l) => Ok(l),
            Err(err) => {
                let msg = format!(
                    "Error while reading json file: {} on directory: {}",
                    err,
                    p.display()
                );
                error!("{}", msg);
                Err(ServiceError::new_with_message(msg))
            }
        };
    };
    let msg = "Could not fetch music list".to_string();
    error!("{}", msg);
    Err(ServiceError::new_with_message(msg))
}

fn music<R: Runtime>(handle: &tauri::AppHandle<R>, music_name: &str) -> Option<Vec<u8>> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(music_name))
    {
        if let Ok(vec) = fs::read(&p) {
            return Some(vec);
        }
    };
    None
}

fn format_resources_music_dir(file_name: &str) -> String {
    format!("{}{}{}", RESOURCES_FOLDER, MUSICS_FOLDER, file_name)
}
