use paris::info;
use serde::Serialize;
use std::cmp::PartialEq;
use std::iter::Iterator;
use strum::{IntoEnumIterator, IntoStaticStr};
use strum_macros::EnumIter;

use crate::errors::{ArduinoCommResult, ArduinoCommunicationError};

#[derive(EnumIter, Debug, Clone, Copy, Serialize, IntoStaticStr, PartialEq)]
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
}

impl Note {
    pub const STATE_OFF: u8 = 128;
    pub const STATE_ON: u8 = 144;
    const MAX_VELOCITY: u8 = 127;

    pub fn ordinal(&self) -> u8 {
        let mut iter = Note::iter().filter(move |x1| !x1.is_bmol()).rev();
        let self_string: &str = self.into();
        let self_string = self_string.replace("b", "");
        let i = iter.position(move |x| {
            let other_string: &str = x.into();
            self_string == other_string
        });
        i.unwrap() as u8
    }
    pub fn from_byte(byte: u8) -> ArduinoCommResult<Self> {
        #[cfg(feature = "verbose")]
        {
            info!("Received byte: {}", byte)
        }
        let i = if let Some(u) = (byte as usize).checked_sub(55) {
            u
        } else {
            return Err(ArduinoCommunicationError::ByteNotSupported(byte));
        };
        let note_iter = Note::iter();
        if i >= note_iter.len() {
            Err(ArduinoCommunicationError::ByteNotSupported(byte))
        } else {
            Ok(note_iter.get(i).unwrap())
        }
    }
    pub fn velocity_percentage(velocity: u8) -> f32 {
        match velocity {
            0 => 0f32,
            Self::MAX_VELOCITY => 100f32,
            _ => velocity as f32 / Self::MAX_VELOCITY as f32,
        }
    }
    pub fn is_bmol(&self) -> bool {
        let s: &str = self.into();
        s.contains('b')
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteWrapper {
    pub note: Note,
    pub byte: u8,
}

impl NoteWrapper {
    pub fn new(note: u8) -> ArduinoCommResult<Self> {
        let n = Note::from_byte(note)?;
        Ok(NoteWrapper {
            note: n,
            byte: note,
        })
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
