use crate::commands::{MidiFileState, ServiceResult};
use crate::constants::errors::{DEVICE_ALREADY_CONNECTED, DEVICE_NOT_CONNECTED, FILE_ALREADY_PLAYING, NO_FILE_BEING_PLAYED, STATE_ACQUIRE_ERROR};
use arduino_comm::midi_connection::{ConnectionHolder, MidiConnection};
use arduino_comm::midi_wrapper::MidiWrapper;
use midi_reader::midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback, ReadingState};
use midi_reader::player_wrapper::PlayerWrapper;
use paris::{error, info, success, warn};
use std::sync::Mutex;
use std::time::Duration;

#[derive(Default)]
pub struct MidiState {
    midi_input_conn: Mutex<Option<MidiConnection>>,
    midi_connection_holder: Mutex<Option<ConnectionHolder>>,
    midi_file: Mutex<Option<(MidiFile, i32)>>,
}

impl MidiState {
    pub fn new() -> Self {
        Self::default()
    }

    fn check_connection(&self) -> ServiceResult<()> {
        match self.midi_connection_holder.lock() {
            Ok(c) => {
                if c.is_some() {
                    Err(DEVICE_ALREADY_CONNECTED.into())
                } else {
                    Ok(())
                }
            },
            Err(_) => Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn has_connection(&self) -> bool {
        match self.midi_connection_holder.lock() {
            Ok(c) => c.is_some(),
            Err(_) => false
        }
    }

    pub fn connected_port_name(&self) -> ServiceResult<String> {
        if let Ok(con) = self.midi_input_conn.lock() {
            if let Some(c) = con.as_ref() {
                Ok(c.port_name.to_owned())
            } else {
                Err(DEVICE_NOT_CONNECTED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn start_listening_to_device<C: Fn(MidiWrapper) + Send + 'static>(&self, callback: C) -> ServiceResult<()> {
        if let Ok(mut m) = self.midi_input_conn.lock() {
            if let Some(c) = m.take() {
                if let Ok(mut m) = self.midi_connection_holder.lock() {
                    if  m.is_some() {
                        Err(DEVICE_ALREADY_CONNECTED.into())
                    } else {
                        *m = Some(c.start_connection(callback)?);
                        Ok(())
                    }
                } else {
                    Err(STATE_ACQUIRE_ERROR.into())
                }
            } else {
                Err(DEVICE_NOT_CONNECTED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn set_device_connection(&self, connection: MidiConnection) -> ServiceResult<()> {
        self.check_connection()?;
        match self.midi_input_conn.lock() {
            Ok(mut m) => {
                *m = Some(connection);
                Ok(())
            }
            Err(_) => {
                Err(STATE_ACQUIRE_ERROR.into())
            }
        }
    }

    pub fn drop_device_connection(&self) {
        info!("Dropping connection...");
        if let Ok(mut ch) = self.midi_connection_holder.lock() {
            if ch.take().is_none() {
                warn!("There is no active input device connection...");
            } else {
                success!("Dropped active input device connection");
            }
        }
    }
    
    pub fn current_midi_file_id(&self) -> ServiceResult<i32> {
        if let Ok(m) = self.midi_file.lock() {
            if let Some((_, id)) = m.as_ref() {
                Ok(id.to_owned())
            } else {
                Err(NO_FILE_BEING_PLAYED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn is_playing_midi_file(&self) -> ServiceResult<()> {
        if let Ok(m) = self.midi_file.lock() {
            if let Some((file, _)) = m.as_ref() {
                match file.current_state() {
                    ReadingState::Playing |
                    ReadingState::Paused => Err(FILE_ALREADY_PLAYING.into()),
                    ReadingState::Stoped |
                    ReadingState::NotRunning => Ok(())
                }
            } else {
                Ok(())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }
    
    pub fn create_new_file_player<P: PlayBackCallback>(
        &self, 
        music_id: i32,
        bytes: Vec<u8>,
        playback: P
    ) -> ServiceResult<PlayerWrapper<P>> {
        if let Ok(mut f) = self.midi_file.lock() {
            let m = MidiFile::from_bytes_vector(bytes)?;
            *f = Some((m, music_id));
            let player_wrapper = f
                .as_mut()
                .unwrap()
                .0
                .create_sheet_player(playback)?;
            Ok(player_wrapper)
        } else {
            error!("{}", STATE_ACQUIRE_ERROR.message);
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn reset_midi_file(&self) -> ServiceResult<()> {
        if let Ok(mut m) = self.midi_file.lock() {
            if m.take().is_none() {
                Err(NO_FILE_BEING_PLAYED.into())
            } else { 
                Ok(())
            }
        } else { 
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn change_file_state(&self, state: MidiFileState) -> ServiceResult<()> {
        if let Ok(mut mf) = self.midi_file.lock() {
            if let Some((c, _)) = mf.as_mut() {
                match state {
                    MidiFileState::FINISHED |
                    MidiFileState::INTERRUPTED => c.stop(),
                    MidiFileState::PAUSED => c.pause(),
                    MidiFileState::PLAYING => c.unpause()
                }
                Ok(())
            } else {
                Err(NO_FILE_BEING_PLAYED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn time_left(&self) -> ServiceResult<Duration> {
        if let Ok(mf) = self.midi_file.lock() {
            if let Some((c, _)) = mf.as_ref() {
                Ok(c.remaining_time())
            } else {
                Err(NO_FILE_BEING_PLAYED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }
}
