use serde::Serialize;
use strum::{IntoEnumIterator, IntoStaticStr};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, Serialize, IntoStaticStr)]
pub enum Note {
    G3,
    Ab3,
    A3,
    Bb3,
    B3,
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
    None,
}

impl Note {
    pub const STATE_OFF: u8 = 128;
    pub const STATE_ON: u8 = 144;
    const MAX_VELOCITY: u8 = 127;
    pub fn ordinal(&self) -> u8 {
        *self as u8
    }
    pub fn from_byte(byte: u8) -> Option<Self> {
        Note::iter().get(byte as usize - 55)
    }
    pub fn velocity_percentage(velocity: u8) -> f32 {
        match velocity {
            0 => 0f32,
            Self::MAX_VELOCITY => 100f32,
            _ => velocity as f32 / Self::MAX_VELOCITY as f32,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteWrapper {
    pub note: Note,
    byte: u8,
}

impl NoteWrapper {
    pub fn new(note: u8) -> Option<Self> {
        if let Some(n) = Note::from_byte(note) {
            Some(NoteWrapper {
                note: n,
                byte: note,
            })
        } else {
            None
        }
    }
}

impl Default for NoteWrapper {
    fn default() -> Self {
        NoteWrapper {
            note: Note::A3,
            byte: 57,
        }
    }
}
