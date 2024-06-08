use serde::{Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ts_rs::TS;

#[derive(EnumIter, Debug, Clone, Copy, Serialize, TS)]
#[ts(export, export_to="../../../src/app/core/model/Note.ts")]
pub enum Note {
    G3, Ab3, A3, Bb3, B3, C4, D4, E4, F4, Gb4, G4, Bb4, B4, None
}

impl Note {
    pub fn ordinal(&self) -> u8 {
        *self as u8
    }
    pub fn from_byte(byte: u8) -> Option<Note> {
        Note::iter().get((byte - 55) as usize)
    }
}

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to="../../../src/app/core/model/MidiSignal.ts", rename="MidiSignal")]
pub struct NoteWrapper {
    pub velocity: u8,
    pub note: Note
}

impl NoteWrapper {
    pub fn new_from_bytes(byte: u8, velocity: u8) -> NoteWrapper {
        NoteWrapper {
            velocity,
            note: *Note::from_byte(byte).get_or_insert(Note::A3)
        }
    }
}

