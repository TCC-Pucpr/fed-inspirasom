use super::payloads::{
    midi_payload::{MidiFileState, MidiPayload},
    music::{MidiMusic, MidiMusicList},
};
use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::commands::payloads::service_error::{ServiceError, ServiceResult};
use crate::constants::dirs::MUSICS_FOLDER;
use crate::{
    app_states::midi_device_state::MidiState,
    constants::events_name::{MIDI_READ_NOTE, MIDI_READ_STATE},
    get_resources_path, RESOURCES_FOLDER,
};
use anyhow::anyhow;
use convert_case::{Case, Casing};
use entity::prelude::{Music, Score};
use entity::{music, score};
use midi_reader::calculate_midi_length;
use midi_reader::errors::MidiReaderError;
use midi_reader::midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback, ReadingState};
use paris::{error, info, warn, Logger};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use std::fs::exists;
use std::{fs, ops::DerefMut};
use tauri::{AppHandle, Runtime, State, Window};

const STATE_CHANGE_ERROR_LOG_MSG: &str =
    "Could not acquire midi file state, probably because there is no file being played";
const STATE_CHANGE_ERROR_MSG: &str = "There is no file being played";

struct SheetListener<'a> {
    window: &'a Window,
    ignore_note_errors: bool,
}

impl <'a> PlayBackCallback for SheetListener<'a> {
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
pub async fn list_musics(db_state: State<'_, DatabaseState>) -> ServiceResult<MidiMusicList> {
    let mut logger = Logger::new();
    logger.loading("Fetching music list...");
    let list = music_list(&db_state).await.map_err(|e| {
        logger.done().error(e.to_string());
        e
    })?;
    let msg = format!("List fetched: {:?}", list);
    logger.done().success(msg);
    Ok(list)
}

#[tauri::command]
pub async fn start_game<R: Runtime>(
    music_id: i32,
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    db_state: State<'_, DatabaseState>,
    handle: AppHandle<R>,
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
    let (music, file) = read_music_from_id(&db_state, &handle, music_id)
        .await
        .map_err(|e1| {
            logger.done().error(e1.to_string());
            e1
        })?;
    let msg = format!("Music found: {}", music);
    logger.done().info(msg);
    let sheet_listener = SheetListener {
        window: &window,
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
        .success("Successfully loaded file, now playing...");
    score_state.reset();
    const RESET_MSG: &str = "Midi file state has been reset!";
    let finished: bool;
    let _ = window.emit(MIDI_READ_STATE, MidiFileState::PLAYING);
    let res: ServiceResult<()> = match p.play() {
        Ok(_) => {
            logger.done().info("Music finished playing");
            finished = true;
            Ok(())
        }
        Err(err) => {
            finished = false;
            if let MidiReaderError::Interrupted = err {
                logger.info(err.to_string());
                Ok(())
            } else {
                logger.error(err.to_string());
                Err(ServiceError::from(err))
            }
        }
    };
    let model = score_state.create_active_model(finished, music_id)?;
    model.insert(&db_state.db).await?;
    score_state.reset();
    midi_state.reset_midi_file();
    logger.info(RESET_MSG);
    res
}

#[tauri::command]
pub async fn pause_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Pause called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, Box::new(move |logger: &mut Logger, state: &mut MidiFile| {
        state.pause();
        logger.done().success("Midi file playback paused successfully");
    }))
}

#[tauri::command]
pub async fn resume_game(
    midi_state: State<'_, MidiState>,
    window: Window,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Resume called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, Box::new(move |logger: &mut Logger, state: &mut MidiFile| {
        state.unpause();
        let _ = window.emit(MIDI_READ_STATE, MidiFileState::PLAYING);
        logger
            .done()
            .success("Midi file playback resumed successfully");
    }))
}

#[tauri::command]
pub async fn stop_game(midi_state: State<'_, MidiState>) -> ServiceResult<()> {
    let mut logger = Logger::new();
    logger.info("Stop called, acquiring midi file state...");
    acquire_state(&mut logger, midi_state, Box::new(move |logger: &mut Logger, state: &mut MidiFile| {
        state.stop();
        logger
            .done()
            .success("Midi file playback stopped successfully");
    }))
}

#[tauri::command]
pub async fn music_length(
    music_id: i32,
    db_state: State<'_, DatabaseState>,
    handle: AppHandle,
) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.info("Calculating midi file length...");
    let (_, f) = match read_music_from_id(&db_state, &handle, music_id).await {
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
    logger.done().success(msg);
    Ok(length)
}

#[tauri::command]
pub async fn remaining_time(midi_state: State<'_, MidiState>) -> ServiceResult<u64> {
    let mut logger = Logger::new();
    logger.loading("Reading remaining time...");
    acquire_state(&mut logger, midi_state, Box::new(move |logger: &mut Logger, state: &mut MidiFile| {
        let dur = state.remaining_time().as_secs();
        let msg = format!("Remaining time obtained: {} seconds left", dur);
        logger.done().info(msg);
        dur
    }))
}

#[tauri::command]
pub async fn add_new_music<R: Runtime>(
    music_name: &str,
    file_path: &str,
    app_handle: AppHandle<R>,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<MidiMusic> {
    if let Ok(e) = exists(file_path) {
        if !e {
            return Err(ServiceError::from(format!(
                "Midi file on {} does not exist",
                file_path
            )));
        }
        if Music::find()
            .filter(music::Column::Name.eq(music_name))
            .one(&db_state.db)
            .await
            .is_ok_and(move |t| t.is_some())
        {
            return Err(ServiceError::from(format!(
                "Midi file with name {} already exist",
                music_name
            )));
        }
        let mut path = get_resources_path(&app_handle);
        path.push(music_name.to_case(Case::Snake));
        fs::copy(file_path, &path)?;
        let dur = calculate_midi_length(file_path);
        let model = music::ActiveModel {
            id: Default::default(),
            name: ActiveValue::Set(music_name.to_string()),
            duration: ActiveValue::Set(dur.as_secs() as i32),
            directory: ActiveValue::Set(path.display().to_string()),
        };
        let new = model.insert(&db_state.db).await?;
        Ok(new.into())
    } else {
        Err(ServiceError::from("Could not find midi file"))
    }
}

#[tauri::command]
pub async fn remove_music<R: Runtime>(
    music_id: i32,
    app_handle: AppHandle<R>,
    db_state: State<'_, DatabaseState>,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    let music = if let Some(m) = Music::find_by_id(music_id).one(&db_state.db).await? {
        m
    } else {
        return Err(ServiceError::from(format!("Music with id {} does not exist", music_id)));
    };
    let mut p = get_resources_path(&app_handle);
    p.push(&music.directory);
    if let Err(_) = fs::remove_file(&p) {
        logger.error(format!(
            "Could not remove midi file at {}, continuing removal from database...",
            p.display().to_string()
        ));
    }
    drop(p);
    logger.loading("Starting removal of music and its scores...");
    let txn = db_state.db.begin().await?;
    Score::delete_many().filter(score::Column::MusicId.eq(music_id)).exec(&txn).await?;
    music.delete(&txn).await?;
    txn.commit().await?;
    logger.done().success(format!("Midi file with id {} removed", music_id));
    Ok(())
}

#[inline]
fn acquire_state<T>(
    logger: &mut Logger,
    midi_state: State<'_, MidiState>,
    on_acquired: Box<dyn Fn(&mut Logger, &mut MidiFile) -> T>,
) -> ServiceResult<T> {
    if let Some(state) = midi_state.midi_file.lock()?.deref_mut() {
        Ok(on_acquired(logger, state))
    } else {
        logger.done().error(STATE_CHANGE_ERROR_LOG_MSG);
        Err(ServiceError::from(STATE_CHANGE_ERROR_MSG))
    }
}

async fn read_music_from_id<R: Runtime>(
    db_state: &State<'_, DatabaseState>,
    handle: &AppHandle<R>,
    music_id: i32,
) -> anyhow::Result<(MidiMusic, Vec<u8>)> {
    let list = music_list(db_state).await?;
    if let Some(m) = list.files.iter().find(|e| e.id == music_id) {
        match music(handle, &m.directory) {
            Ok(vec) => Ok((m.to_owned(), vec)),
            Err(err) => {
                let msg = format!(
                    "Music with id {} found, but error while loading midi file: {}",
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

async fn music_list(db_state: &State<'_, DatabaseState>) -> anyhow::Result<MidiMusicList> {
    let a = Music::find().all(&db_state.db).await?;
    Ok(MidiMusicList::from(a))
}

fn music<R: Runtime>(handle: &AppHandle<R>, music_name: &str) -> Result<Vec<u8>, String> {
    if let Some(p) = handle.path_resolver().resolve_resource(format!(
        "{}{}{}",
        RESOURCES_FOLDER, MUSICS_FOLDER, music_name
    )) {
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
