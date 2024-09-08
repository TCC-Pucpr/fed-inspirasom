use thiserror::Error;

pub type ArduinoCommResult<T> = Result<T, ArduinoCommunicationError>;

#[derive(Debug, Error)]
pub enum ArduinoCommunicationError {
    #[error("Note with byte `{0}` cannot be played because the ocarina does not support it")]
    ByteNotSupported(u8),
    #[error("Error while connecting to midi in")]
    MidiInputError,
    #[error("Unexpected error while reading a midi port")]
    PortError,
    #[error("Port with name `{0}` was not found")]
    PortWithNameNotFound(String),
    #[error("Could not find any device that matches the ocarina")]
    OcarinaNotFound,
    #[error("There is no midi devices connected")]
    NoDevicesConnected,
    #[error("Error while listening to port `{0}`")]
    PortListenError(String),
}
