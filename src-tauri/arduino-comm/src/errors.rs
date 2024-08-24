use std::{error::Error, fmt::Display};

use strum::{EnumMessage, EnumString};

pub(crate) type MidiConnectionResult<S> = Result<S, MidiDeviceConnectionError>;

#[derive(EnumString, EnumMessage, Debug, PartialEq, Eq)]
pub enum Errors {
    #[strum(message = "No midi ports where found")]
    NoPortsFound,
    #[strum(message = "Port with name not found")]
    PortNotFound,
    #[strum(message = "There is already a device connected")]
    AlreadyConnected,
    #[strum(message = "Unexpected error occurred")]
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MidiDeviceConnectionError {
    pub error: Errors,
}

impl From<Errors> for MidiDeviceConnectionError {
    fn from(value: Errors) -> Self {
        Self { error: value }
    }
}

impl Display for MidiDeviceConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Midi connection error: {}",
            self.error.get_message().unwrap()
        )
    }
}
impl Error for MidiDeviceConnectionError {}
