use thiserror::Error;

pub type MidiReaderResult<T> = Result<T, MidiReaderError>;

#[derive(Debug, Error)]
pub enum MidiReaderError {
    #[error("File is invalid")]
    InvalidMidiFile(#[source] anyhow::Error),
    #[error("Error while playing file: `{0}`")]
    PlaybackError(String),
    #[error("This file is already being played")]
    AlreadyPlaying,
    #[error("File playback has been interrupted")]
    Interrupted,
    #[error("An error occurred while connect to this devices midi output")]
    MidiOutputError(#[source] anyhow::Error),
    #[error("No output ports was found")]
    NoPortsFound,
    #[error("Path `{0}` does not exist")]
    FileDoesNotExist(String),
}
