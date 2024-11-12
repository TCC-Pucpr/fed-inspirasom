use crate::note::NoteWrapper;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct MidiWrapper {
    pub air_strength: u8,
    pub note: NoteWrapper,
    pub state: u8,
}

impl MidiWrapper {
    pub fn new_from_bytes(state: u8, byte: u8, velocity: u8) -> Self {
        MidiWrapper {
            air_strength: velocity,
            note: NoteWrapper::new(byte).unwrap_or_else(move |_| Default::default()),
            state,
        }
    }
}

impl Display for MidiWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State: {} | Note: {:?} | Velocity: {}", self.state, self.note, self.air_strength)
    }
}
