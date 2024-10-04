use anyhow::anyhow;
use arduino_comm::{midi_wrapper::MidiWrapper, note::Note};
use serde::Serialize;
use std::fmt::Display;
use thiserror::Error;
use ts_rs::TS;

#[derive(Debug, Error)]
#[error("Invalid note {byte} received")]
pub struct MidiNoteError {
    byte: u8,
    #[source]
    source: anyhow::Error,
}

#[derive(TS, Serialize, Clone)]
#[ts(
    export,
    export_to = "../../src/app/core/model/MidiState.ts",
    rename = "MidiState"
)]
pub enum MidiFileState {
    INTERRUPTED,
    FINISHED,
    PAUSED,
    PLAYING
}

#[derive(TS, Serialize, Clone)]
#[ts(
    export,
    export_to = "../../src/app/core/model/MidiSignal.ts",
    rename = "MidiSignal"
)]
/// O payload para enviar dados da nota midi para o front
pub struct MidiPayload {
    note_index: u8,
    is_bmol: bool,
    note_name: String,
    velocity: u8,
    state: bool,
}

impl MidiPayload {
    pub fn from_midi_wrapper(midi_wrapper: MidiWrapper) -> Self {
        let note = midi_wrapper.note.note;
        let note_name: &str = note.into();
        Self {
            note_index: note.ordinal(),
            is_bmol: note.is_bmol(),
            note_name: note_name.to_string(),
            velocity: midi_wrapper.air_strength,
            state: midi_wrapper.state == Note::STATE_ON,
        }
    }

    pub fn from_note(note: u8, velocity: u8, state: bool) -> Result<Self, MidiNoteError> {
        let s = if velocity == 0 { false } else { state };
        let note = Note::from_byte(note).map_err(|e| MidiNoteError {
            byte: note,
            source: anyhow!(e),
        })?;
        let note_name: &str = note.into();
        Ok(Self {
            note_index: note.ordinal(),
            is_bmol: note.is_bmol(),
            note_name: note_name.to_string(),
            velocity,
            state: s,
        })
    }
}

impl Display for MidiPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = if self.state { "on" } else { "off" };
        write!(
            f,
            "Note index: {} | isBmol: {} | Note name: {} | velocity: {} | state: {}",
            self.note_index, self.is_bmol, self.note_name, self.velocity, state
        )
    }
}
