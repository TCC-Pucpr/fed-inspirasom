use std::error::Error;

mod midi_connection;
pub mod reader_service;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(test)]
mod tests {
    use crate::reader_service::MidiFile;

    #[test]
    fn midi_read_test() {
        let file = "C:\\Users\\KnightLeo\\Downloads\\The Legend of Zelda Ocarina of Time - Great Fairy Fountain.mid";
        let reader = MidiFile::from_file(file).unwrap();
        let _ = reader.play_music();
    }
}
