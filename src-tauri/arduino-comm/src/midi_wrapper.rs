use serde::Serialize;
use ts_rs::TS;

use crate::note::NoteWrapper;

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to="../../../src/app/core/model/MidiSignal.ts", rename="MidiSignal")]
pub struct MidiWrapper {
    #[ts(rename="airStrength")]
    pub air_strength: u8,
    pub note: NoteWrapper,
    pub state: u8
}

impl MidiWrapper {
    pub fn new_from_bytes(state: u8, byte: u8, velocity: u8) -> Self {
        MidiWrapper {
            air_strength: velocity,
            note: *NoteWrapper::new(byte).get_or_insert(Default::default()),
            state
        }
    }
}

