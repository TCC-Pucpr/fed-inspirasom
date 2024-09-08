use crate::sea_orm::ActiveValue;
use entity::music::ActiveModel;
use midi_reader::calculate_midi_length;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;

const DATA_DIR: &str = "/migration/data.json";
const MUSICS_DIR: &str = "../resources/musics";

#[derive(Serialize, Deserialize)]
pub(crate) struct MusicList {
    pub(crate) files: Vec<MusicDataObject>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MusicDataObject {
    name: String,
    directory: String,
}

impl MusicDataObject {
    pub(crate) fn into_active_model(self) -> ActiveModel {
        let file = format!("{}{}", MUSICS_DIR, self.directory);
        println!("Loaded file: {}", file);
        let duration = calculate_midi_length(&file);
        ActiveModel {
            name: ActiveValue::Set(self.name),
            duration: ActiveValue::Set(duration.as_secs() as i32),
            directory: ActiveValue::Set(self.directory),
            ..Default::default()
        }
    }
}

pub(crate) fn load_data_file() -> MusicList {
    let dir = current_dir().unwrap().display().to_string() + DATA_DIR;
    println!("Data file dir: {}", dir);
    let file = File::open(dir).unwrap();
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}
