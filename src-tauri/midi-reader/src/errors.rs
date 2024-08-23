use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct InvalidMidiFile;
#[derive(Debug, Clone)]
pub struct PlaybackError;
#[derive(Debug, Clone)]
pub struct AlreadyPlaying;
#[derive(Debug, Clone)]
pub struct Interrupted;
impl Display for PlaybackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred during game!")
    }
}
impl Display for InvalidMidiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The selected file is invalid!")
    }
}
impl Display for AlreadyPlaying {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The selected file is invalid!")
    }
}
impl Display for Interrupted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The music has been interrupted!")
    }
}
impl Error for PlaybackError {}
impl Error for InvalidMidiFile {}
impl Error for AlreadyPlaying {}
impl Error for Interrupted {}
