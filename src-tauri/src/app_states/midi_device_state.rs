use std::sync::Mutex;
use waitgroup::{WaitGroup, Worker};

pub struct MidiState {
    pub worker: Mutex<Option<Worker>>,
}

impl MidiState {
    pub fn new() -> Self {
        Self {
            worker: Default::default(),
        }
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
}
