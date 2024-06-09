use serde::Serialize;
use ts_rs::TS;

use crate::note::Note;

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to="../../../src/app/core/model/MidiSignal.ts", rename="MidiSignal")]
pub struct MidiWrapper {
    #[ts(rename="airFlow")]
    pub velocity: u8,
    pub note: Note
}

impl MidiWrapper {
    pub fn new_from_bytes(state: u8, byte: u8, velocity: u8) -> Self {
        let note = if state == Note::STATE_ON {
            *Note::from_byte(byte).get_or_insert(Note::A3)
        } else {
            Note::None
        };
        MidiWrapper {
            velocity,
            note
        }
    }
    pub fn new_from_bytes_with_velocity_percentage(state: u8, byte: u8, velocity: u8) -> Self {
        Self::new_from_bytes(state, byte, Note::velocity_percentage(velocity))
    }
}

