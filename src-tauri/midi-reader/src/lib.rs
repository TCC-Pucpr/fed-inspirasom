use std::{
    error::Error,
    sync::{Arc, Mutex},
};

pub mod errors;
mod game_connection;
mod midi_connection;
pub mod midi_file;
mod midi_length_calc;
mod player_wrapper;
mod test_callback;
mod timer;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub(crate) type ArcMutex<P> = Arc<Mutex<P>>;

#[cfg(test)]
mod tests {
    use crate::midi_file::MidiFile;
    #[cfg(feature = "verbose")]
    use paris::info;

    #[test]
    fn midi_read_test() {
        // let file = "C:\\Users\\KnightLeo\\Downloads\\The Legend of Zelda Ocarina of Time - Great Fairy Fountain.mid";
        // let mut reader = MidiFile::from_file(file).unwrap();
        #[cfg(feature = "verbose")]
        info!("AAAAAAAAA");
        MidiFile::normal_play_file(
            "/home/knight_leo/Documents/TCC/fed-inspirasom/src-tauri/resources/musics/teste2.mid",
        );
    }
}
