use std::error::Error;

mod midi_connection;
pub mod reader_service;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(test)]
mod tests {
    #[cfg(feature = "verbose")]
    use paris::info;

    use crate::reader_service::{MidiFile, MidiFilePlayer, PlayBackCallback};

    struct Callback;

    impl PlayBackCallback for Callback {
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

        fn on_pause(&self) {
            #[cfg(feature = "verbose")]
            {
                info!("on_pause called")
            }
        }

        fn on_finished(&self) {
            #[cfg(feature = "verbose")]
            {
                info!("on_finished called")
            }
        }
    }

    #[test]
    fn midi_read_test() {
        let file = "C:\\Users\\KnightLeo\\Downloads\\The Legend of Zelda Ocarina of Time - Great Fairy Fountain.mid";
        let mut reader = MidiFile::from_file(file).unwrap();
        let _ = reader.read_sheet(Callback);
    }
}
