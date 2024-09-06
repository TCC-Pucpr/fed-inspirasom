use midi_reader::midi_file::{MidiFile, MidiFilePlayer};
use std::sync::Mutex;
use waitgroup::{WaitGroup, Worker};

#[derive(Default)]
pub struct MidiState {
    pub worker: Mutex<Option<Worker>>,
    pub midi_file: Mutex<Option<MidiFile>>,
}

impl MidiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_working(&self) -> bool {
        if let Ok(w) = self.worker.lock().as_ref() {
            w.is_some()
        } else {
            false
        }
    }

    pub fn set_worker(&self, wait_group: &WaitGroup) -> bool {
        if let Ok(mut g) = self.worker.lock() {
            *g = Some(wait_group.worker());
            true
        } else {
            false
        }
    }

    pub fn release_worker(&self) -> bool {
        if let Ok(mut g) = self.worker.lock() {
            *g = None;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn create_new_file(&self, bytes: Vec<u8>) -> bool {
        if let Ok(mut m) = self.midi_file.lock() {
            *m = match MidiFile::from_bytes_vector(bytes) {
                Ok(mr) => Some(mr),
                Err(_) => return false,
            };
            true
        } else {
            false
        }
    }

    pub fn reset_midi_file(&self) {
        if let Ok(mut m) = self.midi_file.lock() {
            *m = None
        }
    }
}
