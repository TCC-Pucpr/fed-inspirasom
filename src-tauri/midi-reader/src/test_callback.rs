#[cfg(test)]
use crate::midi_file::PlayBackCallback;
#[cfg(test)]
use nodi::{Connection, MidiEvent};
#[cfg(feature = "verbose")]
#[cfg(test)]
use paris::info;

#[cfg(test)]
pub(crate) struct TestCallback;

#[cfg(test)]
impl PlayBackCallback for TestCallback {
    #[allow(unused_variables)]
    fn on_note(&self, on: bool, key: u8, vel: u8) -> bool {
        #[cfg(feature = "verbose")]
        {
            info!(
                "on_note called: note_on: {} | key: {} | velocity: {}",
                on, key, vel
            )
        }
        true
    }

    fn on_interrupted(&self) {
        #[cfg(feature = "verbose")]
        {
            info!("on_interrupted called")
        }
    }

    fn on_finished(&self) {
        #[cfg(feature = "verbose")]
        {
            info!("on_finished called")
        }
    }

    fn on_pause(&self) {
        #[cfg(feature = "verbose")]
        {
            info!("on_pause called")
        }
    }
}

#[cfg(test)]
impl Connection for TestCallback {
    fn play(&mut self, event: MidiEvent) -> bool {
        #[cfg(feature = "paris")]
        info!("Event sent: {:?}", event);
        true
    }
}
