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
    pub const STATE_OFF: u8 = 128;
    pub const STATE_ON: u8 = 144;
    const MAX_VELOCITY: u8 = 127;
    pub fn ordinal(&self) -> u8 {
        *self as u8
    }
    pub fn from_byte(byte: u8) -> Option<Self> {
        Note::iter().get((byte - 55) as usize)
    }
    pub fn velocity_percentage(velocity: u8) -> u8 {
        match velocity { 
            0 => 0,
            Self::MAX_VELOCITY => 100,
            _ => velocity / Self::MAX_VELOCITY
        }
    }
}

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

