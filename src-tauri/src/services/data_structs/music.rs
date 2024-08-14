use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/app/core/model/MidiMusicList.ts")]
pub struct MidiMusicList {
    pub files: Vec<MidiMusic>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/app/core/model/MidiMusic.ts")]
pub struct MidiMusic {
    pub name: String,
    pub id: String,
    pub directory: String,
}

impl MidiMusicList {
    pub fn from_path_resource(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)?;
        Ok(res)
    }
    pub fn from_json(directory: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(directory)?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)?;
        Ok(res)
    }
}
