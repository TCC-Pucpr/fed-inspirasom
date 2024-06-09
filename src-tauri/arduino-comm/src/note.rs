use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ts_rs::TS;

#[derive(EnumIter, Debug, Clone, Copy, Serialize, TS)]
#[ts(export, export_to="../../../src/app/core/model/Note.ts")]
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
    None
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
    pub fn velocity_percentage(velocity: u8) -> u8 {
        match velocity {
            0 => 0,
            Self::MAX_VELOCITY => 100,
            _ => velocity / Self::MAX_VELOCITY
        }
    }
}