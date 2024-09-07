use crate::{
    app_states::midi_device_state::MidiState,
    commands::service_error::ServiceError,
    constants::events_name::{MIDI_READ_NOTE, MIDI_READ_STATE},
    RESOURCES_FOLDER,
};
use anyhow::anyhow;
use std::{
    fs,
    ops::{Deref, DerefMut},
};

use super::{
    payloads::{
        midi_payload::{MidiFileState, MidiPayload},
        music::{MidiMusic, MidiMusicList},
    },
    service_error::ServiceResult,
};

use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::store_state::StoreState;
use midi_reader::errors::MidiReaderError;
use midi_reader::midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback, ReadingState};
use paris::{error, info, warn, Logger};
use tauri::{Runtime, State, Window};

const MUSICS_FOLDER: &str = "musics/";
const DATA_JSON: &str = "data.json";

const STATE_CHANGE_ERROR_LOG_MSG: &str =
    "Could not acquire midi file state, probably because there is no file being played";
const STATE_CHANGE_ERROR_MSG: &str = "There is no file being played";

struct SheetListener {
    window: Window,
    ignore_note_errors: bool,
}

impl PlayBackCallback for SheetListener {
    fn on_note(&self, on: bool, key: u8, vel: u8) -> bool {
        let payload = match MidiPayload::from_note(key, vel, on) {
            Ok(p) => p,
            Err(err) => {
                return if self.ignore_note_errors {
                    warn!("{}, skipping note...", err);
                    true
                } else {
                    error!("{}, throwing error!", err);
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
    let list = music_list(&app).map_err(|e| {
        logger.done().error(e.to_string());
        e
    })?;
    let msg = format!("List fetched: {:?}", list);
    logger.done().info(msg);
    Ok(list)
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: String,
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    store: State<'_, StoreState>,
    handle: tauri::AppHandle<R>,
    window: Window,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.loading("Loading new file...");
    if let Ok(Some(state)) = midi_state.midi_file.lock().as_deref() {
        match state.current_state() {
            ReadingState::Paused | ReadingState::Playing => {
                return Err(ServiceError::from(MidiReaderError::AlreadyPlaying))
            }
            _ => {}
        }
    };
    let (music, file) = read_music_from_id(&handle, &music_id).map_err(|e1| {
        logger.done().error(e1.to_string());
        e1
    })?;
    let msg = format!("Music found: {}", music);
    logger.done().info(msg);
    let sheet_listener = SheetListener {
        window,
        ignore_note_errors: true,
    };
    logger.loading("Loading file bytes...");
    let p = if let Ok(mut f) = midi_state.midi_file.lock() {
        let mut m = MidiFile::from_bytes_vector(file).map_err(|e| {
            logger.done().error(e.to_string());
            e
        })?;
        let player_wrapper = m.create_sheet_player(sheet_listener).map_err(|e2| {
            logger.done().error(e2.to_string());
            e2
        })?;
        *f = Some(m);
        player_wrapper
    } else {
        const MSG: &str = "Error occurred while unlocking midi file state, this is very suspicious";
        logger.done().error(MSG);
        return Err(ServiceError::from(MSG));
    };
    logger
        .done()
        .info("Successfully loaded file, now playing...");
    score_state.reset();
    const RESET_MSG: &str = "Midi file state has been reset!";
    let res: ServiceResult<()> = match p.play() {
        Ok(_) => {
            logger.done().info("Music finished playing");
            Ok(store.save(&music_id, score_state.deref())?)
        }
        Err(err) => {
            if let MidiReaderError::Interrupted = err {
                logger.info(err.to_string());
                Ok(())
            } else {
                logger.error(err.to_string());
                Err(ServiceError::from(err))
            }
        }
    };
    store.commit()?;
    score_state.reset();
    midi_state.reset_midi_file();
    logger.info(RESET_MSG);
    res
}

#[tauri::command]
pub async fn pause_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Pause called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, move |logger, state| {
        state.pause();
        logger.done().info("Midi file playback paused successfully");
    })
}

#[tauri::command]
pub async fn resume_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Resume called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, move |logger, state| {
        state.unpause();
        logger
            .done()
            .info("Midi file playback resumed successfully");
    })
}

#[tauri::command]
pub async fn stop_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Stop called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, move |logger, state| {
        state.stop();
        logger
            .done()
            .info("Midi file playback stopped successfully");
    })
}

#[tauri::command]
pub async fn music_length(music_id: &str, handle: tauri::AppHandle) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.info("Calculating midi file length...");
    let (_, f) = match read_music_from_id(&handle, music_id) {
        Ok(f) => f,
        Err(err) => {
            let msg = format!("Error while fetching midi file bytes: {}", err);
            logger.error(msg.clone());
            return Err(ServiceError::new_with_message(msg));
        }
    };
    let midi_file = MidiFile::from_bytes_vector(f).map_err(|e| {
        logger.done().error(e.to_string());
        e
    })?;
    let length = midi_file.file_length().as_secs();
    let msg = format!("Successfully calculated length: {}", length);
    logger.done().info(msg);
    Ok(length)
}

#[tauri::command]
pub async fn remaining_time(midi_state: State<'_, MidiState>) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Reading remaining time...");
    acquire_state(&mut logger, midi_state, move |logger, state| {
        let dur = state.remaining_time().as_secs();
        let msg = format!("Remaining time obtained: {} seconds left", dur);
        logger.done().info(msg);
        dur
    })
}

#[inline]
fn acquire_state<T>(
    logger: &mut Logger,
    midi_state: State<'_, MidiState>,
    on_acquired: fn(&mut Logger, &mut MidiFile) -> T,
) -> ServiceResult<T> {
    if let Some(state) = midi_state.midi_file.lock()?.deref_mut() {
        Ok(on_acquired(logger, state))
    } else {
        logger.done().error(STATE_CHANGE_ERROR_LOG_MSG);
        Err(ServiceError::from(STATE_CHANGE_ERROR_MSG))
    }
}

fn read_music_from_id<R: Runtime>(
    handle: &tauri::AppHandle<R>,
    music_id: &str,
) -> anyhow::Result<(MidiMusic, Vec<u8>)> {
    let list = music_list(handle)?;
    if let Some(m) = list.files.iter().find(|e| e.id == music_id) {
        match music(handle, &m.directory) {
            Ok(vec) => Ok((m.to_owned(), vec)),
            Err(err) => {
                let msg = format!(
                    "Music with id {} found, but error while loading midI file: {}",
                    music_id, err
                );
                Err(anyhow!(msg))
            }
        }
    } else {
        Err(anyhow!(format!(
            "Music with id {} does not exist",
            music_id
        )))
    }
}

fn music_list<R: Runtime>(handle: &tauri::AppHandle<R>) -> anyhow::Result<MidiMusicList> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(DATA_JSON))
    {
        return MidiMusicList::from_path_resource(&p).map_err(move |e| anyhow!(e));
    };
    let msg = "Could not fetch music list".to_string();
    Err(anyhow!(msg))
}

fn music<R: Runtime>(handle: &tauri::AppHandle<R>, music_name: &str) -> Result<Vec<u8>, String> {
    if let Some(p) = handle
        .path_resolver()
        .resolve_resource(format_resources_music_dir(music_name))
    {
        if let Ok(vec) = fs::read(&p) {
            Ok(vec)
        } else {
            let msg = format!(
                "Music file {} is not present in path {}",
                music_name,
                p.display()
            );
            Err(msg)
        }
    } else {
        Err(String::from("Could not get path resolver"))
    }
}

#[inline]
fn format_resources_music_dir(file_name: &str) -> String {
    format!("{}{}{}", RESOURCES_FOLDER, MUSICS_FOLDER, file_name)
}
