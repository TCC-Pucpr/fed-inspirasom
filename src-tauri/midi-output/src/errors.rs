use crate::errors::MidiOutputErrors::Connecting;
use midir::InitError;
use thiserror::Error;

pub type MidiOutputResult<T> = Result<T, MidiOutputErrors>;

#[derive(Error, Debug)]
pub enum MidiOutputErrors {
    #[error("Error while trying to find a midi port")]
    Connecting,
    #[error("Error while trying to connect to midi port `{0}`")]
    ConnectToPort(String),
    #[error("Port `{0}` was not found")]
    PortNotFound(String),
    #[error("Byte `{0}` was unable to be sent")]
    CouldNotSendByte(u8),
    #[error("No available output ports")]
    NoAvailablePorts,
}

impl From<InitError> for MidiOutputErrors {
    fn from(_: InitError) -> Self {
        Connecting
    }
}