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

use midi_reader::{
    errors::Interrupted,
    midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback, ReadingState},
};
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
                return if self.ignore_note_errors {
                    warn!("Midi music sent invalid note, skipping note...");
                    true
                } else {
                    error!("Midi music sent invalid note, throwing error!");
                    false
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

    fn on_finished(&self) {
        info!("Emitting Finished Midi State");
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::FINISHED);
    }

    fn on_pause(&self) {
        info!("Emitting Paused Midi State");
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::PAUSED);
    }
}

#[tauri::command]
pub async fn list_musics<R: Runtime>(app: tauri::AppHandle<R>) -> ServiceResult<MidiMusicList> {
    let mut logger = Logger::new();
    logger.loading("Fetching music list...");
    match music_list(&app) {
        Ok(l) => {
            let msg = format!("List fetched: {:?}", l);
            logger.done().info(msg);
            Ok(l)
        }
        Err(err) => {
            let msg = format!("Error while fetching music list: {}", err);
            logger.done().error(msg);
            Err(ServiceError::from(err))
        }
    }
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
    let (music, file) = match read_music_from_id(&handle, music_id) {
        Ok(m) => m,
        Err(err) => {
            logger.done().error(err.clone());
            return Err(ServiceError::from(err));
        }
    };
    let msg = format!("Music found: {}", music);
    logger.done().info(msg);
    let sheet_listener = SheetListener {
        window,
        ignore_note_errors: true,
    };
    logger.loading("Loading file bytes...");
    let p = if let Ok(mut f) = midi_state.midi_file.lock() {
        let mut m = MidiFile::from_bytes_vector(file).unwrap();
        if let Ok(r) = m.create_sheet_player(sheet_listener) {
            *f = Some(m);
            r
        } else {
            const MSG: &str = "Error occurred while creating a sheet player";
            logger.done().error(MSG);
            return Err(ServiceError::from(MSG));
        }
    } else {
        const MSG: &str = "Error occurred while unlocking midi file state, this is very suspicious";
        logger.done().error(MSG);
        return Err(ServiceError::from(MSG));
    };
    logger
        .done()
        .info("Successfully loaded file, now playing...");
    match p.play() {
        Ok(_) => {
            logger.done().info("Music finished playing");
        }
        Err(err) => {
            if let Some(e) = err.downcast_ref::<Interrupted>() {
                logger.info(e.to_string());
            } else {
                let msg = format!("Error while playing song: {}", err);
                logger.error(msg.clone());
                return Err(ServiceError::from(msg));
            }
        }
    };
    midi_state.reset_midi_file();
    logger.info("Midi file state has been reset!");
    Ok(())
}

#[tauri::command]
pub async fn pause_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Pause called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| {
            logger.done().error("Error while acquiring midi file state");
            ServiceError::generic()
        })?
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
pub async fn resume_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Resume called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| {
            logger.done().error("Error while acquiring midi file state");
            ServiceError::generic()
        })?
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
pub async fn stop_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Stop called, acquiring midi file state...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| {
            logger.done().error("Error while acquiring midi file state");
            ServiceError::generic()
        })?
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
pub async fn music_length(music_id: String, handle: tauri::AppHandle) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Calculating midi file length...");
    let (_, f) = match read_music_from_id(&handle, music_id) {
        Ok(f) => f,
        Err(err) => {
            let msg = format!("Error while fetching midi file bytes: {}", err);
            logger.done().error(msg.clone());
            return Err(ServiceError::new_with_message(msg));
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
pub async fn remaining_time(midi_state: State<'_, MidiState>) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Reading remaining time...");
    if let Some(state) = midi_state
        .midi_file
        .lock()
        .map_err(|_| {
            logger.done().error("Error while acquiring midi file state");
            ServiceError::generic()
        })?
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
) -> Result<(MidiMusic, Vec<u8>), String> {
    let list = music_list(handle)?;
    if let Some(m) = list.files.iter().find(|e| e.id == music_id) {
        match music(handle, &m.directory) {
            Ok(vec) => Ok((m.to_owned(), vec)),
            Err(err) => {
                let msg = format!(
                    "Music with id {} found, but error while loading .mid file: {}",
                    music_id, err
                );
                Err(msg)
            }
        }
    } else {
        Err(format!("Music with id {} does not exist", music_id))
    }
}

fn music_list<R: Runtime>(handle: &tauri::AppHandle<R>) -> Result<MidiMusicList, String> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(DATA_JSON))
    {
        return match MidiMusicList::from_path_resource(&p) {
            Ok(l) => Ok(l),
            Err(err) => {
                let msg = format!(
                    "Error while reading json file: {} on directory: {}",
                    err,
                    p.display()
                );
                Err(msg)
            }
        };
    };
    let msg = "Could not fetch music list".to_string();
    Err(msg)
}

fn music<R: Runtime>(handle: &tauri::AppHandle<R>, music_name: &str) -> Result<Vec<u8>, String> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(music_name))
    {
        return if let Ok(vec) = fs::read(&p) {
            Ok(vec)
        } else {
            let msg = format!(
                "Music file {} is not present in path {}",
                music_name,
                p.display()
            );
            Err(msg)
        };
    };
    Err(String::from("Could not get path resolver"))
}

#[inline]
fn format_resources_music_dir(file_name: &str) -> String {
    format!("{}{}{}", RESOURCES_FOLDER, MUSICS_FOLDER, file_name)
}
