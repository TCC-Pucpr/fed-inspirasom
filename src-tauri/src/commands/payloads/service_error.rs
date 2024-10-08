use crate::app_states::database_state::DatabaseError;
use crate::constants::errors::{CodedError, COULD_NOT_UPDATE_DATABASE, DATABASE_COULD_NOT_CREATE, DATABASE_COULD_NOT_LOAD, DATABASE_QUERY_ERROR, DEVICE_COULD_NOT_CONNECT, DEVICE_LISTEN_ERROR, DEVICE_NO_INPUT_CONNECTIONS_FOUND, DEVICE_PORT_NOT_FOUND, FILE_ALREADY_PLAYING, FILE_NOT_FOUND, MIDI_NOT_SUPPORTED, MIDI_NO_AVAILABLE_PORTS, MIDI_OUTPUT_CONNECTION_FAILED, MIDI_UNEXPECTED_PLAYBACK_ERROR, STATE_ACQUIRE_ERROR, STORAGE_COULD_NOT_BE_CREATED, STORAGE_COULD_NOT_READ, STORAGE_COULD_NOT_WRITE, STORAGE_HAS_NOT_BEEN_CREATED, STORAGE_KEY_DOES_NOT_EXIST, UNEXPECTED_ERROR};
use anyhow::Error;
use arduino_comm::errors::ArduinoCommunicationError;
use midi_reader::errors::MidiReaderError;
use paris::error;
use persistence::storage::StorageError;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::sync::PoisonError;
use thiserror::Error;
use ts_rs::TS;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../src/app/core/model/ServiceError.ts")]
#[error("Service error: {message} | code: {code}")]
pub struct ServiceError {
    pub code: String,
    pub message: String,
}

impl Default for ServiceError {
    fn default() -> Self {
        Self::generic()
    }
}

#[allow(dead_code)]
impl ServiceError {
    pub fn new_with_message(message: String) -> Self {
        Self {
            code: String::new(),
            message,
        }
    }
    pub fn new_with_str(message: &str) -> Self {
        Self {
            code: String::new(),
            message: String::from(message),
        }
    }
    pub fn new_with_code(code: String) -> Self {
        Self {
            code,
            message: String::new(),
        }
    }
    pub fn new_with_code_str(code: &str) -> Self {
        Self {
            code: code.to_string(),
            message: String::new(),
        }
    }
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
    pub fn generic() -> Self {
        Self::from(UNEXPECTED_ERROR)
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(value: Error) -> Self {
        error!("Unexpected error received: {}", value.to_string());
        Self::new_with_message(value.to_string())
    }
}

impl From<ArduinoCommunicationError> for ServiceError {
    fn from(value: ArduinoCommunicationError) -> Self {
        match value {
            ArduinoCommunicationError::MidiInputError | 
            ArduinoCommunicationError::PortError => Self::from(DEVICE_COULD_NOT_CONNECT),
            ArduinoCommunicationError::PortWithNameNotFound(_) | 
            ArduinoCommunicationError::OcarinaNotFound => Self::from(DEVICE_PORT_NOT_FOUND),
            ArduinoCommunicationError::NoDevicesConnected => Self::from(DEVICE_NO_INPUT_CONNECTIONS_FOUND),
            ArduinoCommunicationError::PortListenError(_) => Self::from(DEVICE_LISTEN_ERROR),
            _ => Self::from(value.to_string())
        }
    }
}

impl From<MidiReaderError> for ServiceError {
    fn from(value: MidiReaderError) -> Self {
        match value {
            MidiReaderError::InvalidMidiFile(_) => Self::from(MIDI_NOT_SUPPORTED),
            MidiReaderError::PlaybackError(_) => Self::from(MIDI_UNEXPECTED_PLAYBACK_ERROR),
            MidiReaderError::AlreadyPlaying => Self::from(FILE_ALREADY_PLAYING),
            MidiReaderError::MidiOutputError(_) => Self::from(MIDI_OUTPUT_CONNECTION_FAILED),
            MidiReaderError::NoPortsFound => Self::from(MIDI_NO_AVAILABLE_PORTS),
            MidiReaderError::FileDoesNotExist(_) => Self::from(FILE_NOT_FOUND),
            _ => Self::from(value.to_string()),
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        match value {
            StorageError::CouldNotCreateStorage(_) |
            StorageError::StorageAlreadyExists(_) => Self::from(STORAGE_COULD_NOT_BE_CREATED),
            StorageError::StorageIsLocked => Self::from(STATE_ACQUIRE_ERROR),
            StorageError::StorageWriteError { .. } |
            StorageError::StorageCommitError(_) => Self::from(STORAGE_COULD_NOT_WRITE),
            StorageError::StorageReadError(_) => Self::from(STORAGE_COULD_NOT_READ),
            StorageError::KeyNotFound(_) => Self::from(STORAGE_KEY_DOES_NOT_EXIST),
            StorageError::StorageDoesNotExist(_) => Self::from(STORAGE_HAS_NOT_BEEN_CREATED)
        }
    }
}

impl From<&str> for ServiceError {
    fn from(value: &str) -> Self {
        error!("Unexpected error: {}", value);
        Self::new_with_str(value)
    }
}

impl From<String> for ServiceError {
    fn from(value: String) -> Self {
        error!("Unexpected error: {}", value);
        Self::new_with_message(value)
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(value: DatabaseError) -> Self {
        match value {
            DatabaseError::CouldNotConnect(_, _) |
            DatabaseError::MigrationError(_, _) => Self::from(DATABASE_COULD_NOT_LOAD),
            DatabaseError::CouldNotCreateFile(_, _) => Self::from(DATABASE_COULD_NOT_CREATE)
        }
    }
}

impl From<DbErr> for ServiceError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::Conn(_) |
            DbErr::ConnectionAcquire(_) |
            DbErr::Migration(_) => Self::from(DATABASE_COULD_NOT_LOAD),
            DbErr::Exec(_) |
            DbErr::Query(_) |
            DbErr::RecordNotFound(_) => Self::from(DATABASE_QUERY_ERROR),
            DbErr::UpdateGetPrimaryKey |
            DbErr::AttrNotSet(_) |
            DbErr::RecordNotInserted |
            DbErr::RecordNotUpdated => Self::from(COULD_NOT_UPDATE_DATABASE),
            _ => Self::from(value.to_string())
        }
    }
}

impl<T: Sized> From<PoisonError<T>> for ServiceError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::from(STATE_ACQUIRE_ERROR)
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(value: std::io::Error) -> Self {
        Self::from(value.to_string())
    }
}

impl From<tauri::Error> for ServiceError {
    fn from(value: tauri::Error) -> Self {
        Self::from(value.to_string())
    }
}

impl From<CodedError> for ServiceError {
    fn from(value: CodedError) -> Self {
        error!("Error received: {}", value.message);
        ServiceError {
            code: value.code.to_string(),
            message: value.message.to_string()
        }
    }
}
