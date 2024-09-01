use std::collections::HashSet;

use crate::midi_file::{PlayBackCallback, ReadingState};
use midly::MidiMessage;
use nodi::{Connection, MidiEvent};
use utils::mutable_arc::MutableArc;

pub(crate) struct GamePlayer<P: PlayBackCallback> {
    callback: MutableArc<P>,
    on_notes: HashSet<u8>,
    reading_state: MutableArc<ReadingState>,
}

impl<P: PlayBackCallback> GamePlayer<P> {
    pub fn new(callback: MutableArc<P>, reading_state: MutableArc<ReadingState>) -> Self {
        Self {
            callback,
            on_notes: HashSet::new(),
            reading_state,
        }
    }
}

impl<P: PlayBackCallback> Connection for GamePlayer<P> {
    fn play(&mut self, event: MidiEvent) -> bool {
        if let Some(m) = self.reading_state.get_data() {
            match *m {
                ReadingState::Stoped | ReadingState::NotRunning => {
                    return false;
                }
                _ => {}
            }
        }
        match event.message {
            MidiMessage::NoteOff { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.remove(&k);
                if let Some(c) = self.callback.get_data() {
                    c.on_note(false, k, vel.into())
                } else {
                    false
                }
            }
            MidiMessage::NoteOn { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.insert(k);
                if let Some(c) = self.callback.get_data() {
                    c.on_note(true, k, vel.into())
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    fn all_notes_off(&mut self) {
        if let Some(c) = self.callback.get_data() {
            for note in &self.on_notes {
                c.on_note(false, *note, 0);
            }
        }
        self.on_notes.clear();
    }
}
