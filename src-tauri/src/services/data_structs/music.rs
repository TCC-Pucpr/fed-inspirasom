use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MidiMusicList {
    pub files: Vec<MidiMusic>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
