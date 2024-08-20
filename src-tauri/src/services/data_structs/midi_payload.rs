use arduino_comm::{midi_wrapper::MidiWrapper, note::Note};
use serde::Serialize;
use ts_rs::TS;

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
}

#[derive(TS, Serialize, Clone)]
#[ts(
    export,
    export_to = "../../src/app/core/model/MidiSignal.ts",
    rename = "MidiSignal"
)]
pub struct MidiPayload {
    note_index: u8,
    is_bmol: bool,
    note_name: String,
    #[ts(rename = "airStrength")]
    velocity: u8,
    #[ts(rename = "on")]
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

    pub fn from_note(note: u8, velocity: u8, state: bool) -> Option<Self> {
        let note = match Note::from_byte(note) {
            Some(n) => n,
            None => return None,
        };
        let note_name: &str = note.into();
        Some(Self {
            note_index: note.ordinal(),
            is_bmol: note.is_bmol(),
            note_name: note_name.to_string(),
            velocity,
            state,
        })
    }
}
