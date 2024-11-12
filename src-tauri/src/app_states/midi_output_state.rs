use std::sync::Mutex;
use arduino_comm::note::NoteWrapper;
use midi_output::output_connection::MidiOutputConnectionHolder;
use crate::commands::ServiceResult;
use crate::constants::errors::{MIDI_OUTPUT_ALREADY_CONNECTED, MIDI_OUTPUT_NOT_CONNECTED, STATE_ACQUIRE_ERROR};

#[derive(Default)]
pub struct MidiOutputState {
    midi_output_connection: Mutex<Option<MidiOutputConnectionHolder>>,
}

impl MidiOutputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_output_connection(&self) -> ServiceResult<()> {
        if let Ok(mut mf) = self.midi_output_connection.lock() {
            let a = MidiOutputConnectionHolder::new_with_first_port()?;
            *mf = Some(a);
            Ok(())
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn create_output_connection_with_name(&self, port_name: &str) -> ServiceResult<()> {
        if let Ok(mut mf) = self.midi_output_connection.lock() {
            if mf.is_some() {
                Err(MIDI_OUTPUT_ALREADY_CONNECTED.into())
            } else {
                let a = MidiOutputConnectionHolder::new_with_port_name(String::from(port_name))?;
                *mf = Some(a);
                Ok(())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn send_output_note(&self, note_wrapper: NoteWrapper, on: bool, velocity: u8) -> ServiceResult<()> {
        if let Ok(mut mf) = self.midi_output_connection.lock() {
            if let Some(conn) = mf.as_mut() {
                conn.send_note(on, note_wrapper.byte, velocity)?;
                Ok(())
            } else {
                Err(MIDI_OUTPUT_NOT_CONNECTED.into())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }

    pub fn drop_output_connection(&self) -> ServiceResult<()> {
        if let Ok(mut mf) = self.midi_output_connection.lock() {
            if let None = mf.take() {
                Err(MIDI_OUTPUT_NOT_CONNECTED.into())
            } else {
                Ok(())
            }
        } else {
            Err(STATE_ACQUIRE_ERROR.into())
        }
    }
}