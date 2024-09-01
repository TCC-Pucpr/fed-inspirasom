use std::{
    collections::HashSet,
    ops::Deref
    ,
};

use crate::midi_file::{PlayBackCallback, ReadingState};
use crate::ArcMutex;
use midly::MidiMessage;
use nodi::{Connection, MidiEvent};

pub(crate) struct GamePlayer<P: PlayBackCallback> {
    callback: ArcMutex<P>,
    on_notes: HashSet<u8>,
    reading_state: ArcMutex<ReadingState>,
}

impl<P: PlayBackCallback> GamePlayer<P> {
    pub fn new(callback: ArcMutex<P>, reading_state: ArcMutex<ReadingState>) -> Self {
        Self {
            callback,
            on_notes: HashSet::new(),
            reading_state,
        }
    }
}

impl<P: PlayBackCallback> Connection for GamePlayer<P> {
    fn play(&mut self, event: MidiEvent) -> bool {
        if let Ok(m) = self.reading_state.lock() {
            match *m {
                ReadingState::Stoped => {
                    return false;
                }
                _ => {}
            }
        }
        match event.message {
            MidiMessage::NoteOff { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.remove(&k);
                if let Ok(c) = self.callback.deref().lock() {
                    c.on_note(false, k, vel.into())
                } else {
                    false
                }
            }
            MidiMessage::NoteOn { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.insert(k);
                if let Ok(c) = self.callback.deref().lock() {
                    c.on_note(true, k, vel.into())
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    fn all_notes_off(&mut self) {
        if let Ok(c) = self.callback.deref().lock() {
            for note in &self.on_notes {
                c.on_note(false, *note, 0);
            }
        }
        self.on_notes.clear();
    }
}
