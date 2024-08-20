use midi_reader::reader_service::MidiFile;
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
        self.worker.lock().as_ref().is_ok()
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

    pub fn update_midi_file(&self, midi_file: Option<MidiFile>) -> bool {
        if let Ok(mut m) = self.midi_file.lock() {
            *m = midi_file;
            true
        } else {
            false
        }
    }
}
