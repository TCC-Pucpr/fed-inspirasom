use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/app/core/model/MidiMusicList.ts")]
pub struct MidiMusicList {
    pub files: Vec<MidiMusic>,
}

/// Struct para uma musica, vai possuir o nome, id, e o diretorio dela para localizala rapidamente.
///
/// Opcionalmente vai ser a sua duração em segundos, esse campo pode ser nulo.
/// Se esse for o caso, chama o comando `music_length`.
///
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/app/core/model/MidiMusic.ts")]
pub struct MidiMusic {
    pub name: String,
    pub id: String,
    pub directory: String,
    pub duration: Option<u64>, // em segundos
}

#[allow(dead_code)]
impl MidiMusicList {
    /// Cria um [`MidiMusicList`] baseado no json dentro do arquivo localizado no
    /// `path`.
    /// O `path` deve sempre ser o diretorio do arquivo com base nos resources do Tauri.
    /// Voce pode tambem criar essa lista com um `&str` chamando a funcao [`Self::from_json_file`]
    pub fn from_path_resource(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)?;
        Ok(res)
    }
    pub fn from_json_file(directory: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(directory)?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)?;
        Ok(res)
    }
}
