use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::BufReader, path::PathBuf};
use thiserror::Error;
use ts_rs::TS;

pub type MidiMusicResult<T> = Result<T, MidiMusicError>;

#[derive(Debug, Error)]
pub enum MidiMusicError {
    #[error("Json file at `{0}` is not valid")]
    InvalidJsonFile(String, #[source] anyhow::Error),
    #[error("Data file at `{0}` does not exist")]
    DataFileDoesNotExist(String, #[source] anyhow::Error),
}

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
    pub fn from_path_resource(path: &PathBuf) -> MidiMusicResult<Self> {
        let dir = path.display().to_string();
        let file =
            File::open(path).map_err(|e| MidiMusicError::DataFileDoesNotExist(dir.clone(), anyhow!(e)))?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)
            .map_err(move |e1| MidiMusicError::InvalidJsonFile(dir, anyhow!(e1)))?;
        Ok(res)
    }
    pub fn from_json_file(directory: &str) -> MidiMusicResult<Self> {
        let dir = directory.to_string();
        let file = File::open(directory)
            .map_err(|e| MidiMusicError::DataFileDoesNotExist(dir.clone(), anyhow!(e)))?;
        let buf_reader = BufReader::new(file);
        let res = serde_json::from_reader(buf_reader)
            .map_err(move |e1| MidiMusicError::InvalidJsonFile(dir, anyhow!(e1)))?;
        Ok(res)
    }
}

impl Display for MidiMusic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {} | Music name: {} | directory: {} | duration: {:?}",
            self.id, self.name, self.directory, self.duration
        )
    }
}
