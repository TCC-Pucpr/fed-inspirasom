use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub enum Note {
    Ab3, B4, Bb3, A4, F4, Bb4, E4, A3, G3, C4, G4, D4, Gb4, None
}

pub fn byte_to_note(b: u8) -> Note {
    Note::A3
}

#[derive(Clone, Serialize)]
pub struct NoteWrapper {
    pub velocity: u8,
    pub note: Note
}

impl NoteWrapper {
    pub fn new_from_bytes(byte: u8, velocity: u8) -> NoteWrapper {
        NoteWrapper {
            velocity,
            note: byte_to_note(byte)
        }
    }
}

