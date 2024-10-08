use crate::app_states::current_music_score_state::CurrentMusicScoreState;
use crate::app_states::database_state::DatabaseState;
use crate::app_states::midi_device_state::MidiState;
use crate::app_states::monitoring_state::MonitoringState;
use crate::commands::commands_utils::database_queries::{music_list, ScoreSaver};
use crate::commands::payloads::midi_payload::{MidiFileState, MidiPayload};
use crate::commands::payloads::music::MidiMusic;
use crate::commands::payloads::service_error::{ServiceError, ServiceResult};
use crate::constants::dirs::MUSICS_FOLDER;
use crate::constants::errors::{FILE_ID_NOT_FOUND, FILE_IS_NOT_A_MIDI, FILE_LOAD_ERROR, FILE_NOT_FOUND, FILE_TOO_LONG};
use crate::constants::events_name::{MIDI_READ_NOTE, MIDI_READ_STATE};
use crate::constants::limits::MIDI_LENGTH_SECONDS_LIMIT;
use crate::get_resources_path;
use midi_reader::calculate_midi_length;
use midi_reader::errors::MidiReaderError;
use midi_reader::midi_file::PlayBackCallback;
use midi_reader::player_wrapper::PlayerWrapper;
use paris::{error, info, warn, Logger};
use std::fs;
use tauri::{AppHandle, Runtime, State, Window};

pub(crate) struct SheetListener<'a> {
    window: &'a Window,
    ignore_note_errors: bool,
}

impl <'a> SheetListener<'a> {
    pub fn new(window: &'a Window, ignore_errors: bool) -> Self {
        Self { window, ignore_note_errors: ignore_errors }
    }
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
        let _ = self.window.emit(MIDI_READ_STATE, MidiFileState::INTERRUPTED);
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

#[inline]
pub async fn load_file<R: Runtime>(
    music_id: i32,
    db_state: &DatabaseState,
    handle: AppHandle<R>,
    logger: &mut Logger<'_>
) -> ServiceResult<Vec<u8>> {
    logger.info(format!("Looking for midi file with id {music_id}..."));
    let (music, file) = read_music_from_id(db_state, &handle, music_id)
        .await
        .map_err(|e1| {
            logger.done().error(e1.to_string());
            e1
        })?;
    logger.success(format!("Music found: {}", music));
    Ok(file)
}

#[inline]
pub fn play_game(
    player: PlayerWrapper<SheetListener>,
    logger: &mut Logger,
) -> ServiceResult<bool> {
    logger.info("Playing music...");
    match player.play() {
        Ok(_) => {
            logger.info("Music finished playing");
            Ok(true)
        }
        Err(err) => {
            if let MidiReaderError::Interrupted = err {
                logger.info(err.to_string());
                Ok(false)
            } else {
                logger.error(err.to_string());
                Err(ServiceError::from(err))
            }
        }
    }
}

pub async fn read_music_from_id<R: Runtime>(
    db_state: &DatabaseState,
    handle: &AppHandle<R>,
    music_id: i32,
) -> ServiceResult<(MidiMusic, Vec<u8>)> {
    let list = music_list(db_state).await?;
    if let Some(m) = list.files.iter().find(|e| e.id == music_id) {
        match music(handle, &m.directory) {
            Ok(vec) => Ok((m.to_owned(), vec)),
            Err(err) => {
                error!(
                    "Music with id {} found, but an error occurred while loading midi file: {}",
                    music_id, err
                );
                Err(FILE_LOAD_ERROR.into())
            }
        }
    } else {
        Err(FILE_ID_NOT_FOUND.into())
    }
}

pub fn music<R: Runtime>(handle: &AppHandle<R>, music_name: &str) -> ServiceResult<Vec<u8>> {
    let path = get_resources_path(handle)?.join(MUSICS_FOLDER).join(music_name);
    if let Ok(vec) = fs::read(&path) {
        Ok(vec)
    } else {
        error!(
            "Music file {} is not present in path {}",
            music_name, path.display()
        );
        Err(FILE_NOT_FOUND.into())
    }
}

pub fn check_midi_file(
    file_location: &str
) -> ServiceResult<u64> {
    if !file_location.ends_with(".mid") {
        Err(FILE_IS_NOT_A_MIDI.into())
    } else {
        let duration_secs = calculate_midi_length(file_location).as_secs();
        if duration_secs > MIDI_LENGTH_SECONDS_LIMIT {
            Err(FILE_TOO_LONG.into())
        } else {
            Ok(duration_secs)
        }
    }
}

pub async fn end_game(
    finished: bool,
    midi_state: State<'_, MidiState>,
    score_state: State<'_, CurrentMusicScoreState>,
    db_state: State<'_, DatabaseState>,
    monitor_state: State<'_, MonitoringState>,
) -> ServiceResult<()> {
    let mut logger = Logger::new();
    let music_id = midi_state.current_midi_file_id()?;
    logger.info("Saving score and monitoring data...");
    db_state.save_score(finished, music_id, &score_state, &monitor_state).await?;
    logger.success("Finished saving score and monitoring data");
    logger.info("Resetting states...");
    score_state.reset();
    monitor_state.reset();
    midi_state.reset_midi_file()?;
    logger.success("Game states has been reset!");
    Ok(())
}