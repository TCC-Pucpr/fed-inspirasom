use std::{
    error::Error,
    sync::{Arc, Mutex},
};

mod errors;
mod game_connection;
mod midi_connection;
pub mod midi_file;
mod midi_length_calc;
mod player_wrapper;
mod timer;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub(crate) type ArcMutex<P> = Arc<Mutex<P>>;

#[cfg(test)]
mod tests {
    #[cfg(verbose)]
    use paris::info;

    use crate::midi_file::{MidiFile, MidiFilePlayer, PlayBackCallback};

    struct Callback;

    impl PlayBackCallback for Callback {
        #[allow(unused_variables)]
        fn on_note(&self, on: bool, key: u8, vel: u8) -> bool {
            #[cfg(verbose)]
            {
                info!(
                    "on_note called: note_on: {} | key: {} | velocity: {}",
                    on, key, vel
                )
            }
            true
        }

        fn on_interrupted(&self) {
            #[cfg(verbose)]
            {
                info!("on_interrupted called")
            }
        }

        fn on_pause(&self) {
            #[cfg(verbose)]
            {
                info!("on_pause called")
            }
        }

        fn on_finished(&self) {
            #[cfg(verbose)]
            {
                info!("on_finished called")
            }
        }
    }

    #[test]
    fn midi_read_test() {
        let file = "C:\\Users\\KnightLeo\\Downloads\\The Legend of Zelda Ocarina of Time - Great Fairy Fountain.mid";
        let mut reader = MidiFile::from_file(file).unwrap();
        let p = reader.create_sheet_player(Callback).unwrap();
        drop(reader);
        let _ = p.play();
    }
}
