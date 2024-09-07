use crate::app_states::database_state::DatabaseError;
use anyhow::Error;
use arduino_comm::errors::ArduinoCommunicationError;
use midi_reader::errors::MidiReaderError;
use serde::{Deserialize, Serialize};
use std::sync::PoisonError;
use storage::storage::StorageError;
use thiserror::Error;
use ts_rs::TS;

pub type ServiceResult<T> = Result<T, ServiceError>;

const MUTEX_ERROR_MESSAGE: &str = "Mutex is poisoned";

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
        Self {
            code: String::from("0001"),
            message: String::from("Erro inesperado"),
        }
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(value: Error) -> Self {
        Self::new_with_message(value.to_string())
    }
}

impl From<ArduinoCommunicationError> for ServiceError {
    fn from(value: ArduinoCommunicationError) -> Self {
        Self::new_with_message(value.to_string())
    }
}

impl From<MidiReaderError> for ServiceError {
    fn from(value: MidiReaderError) -> Self {
        Self::new_with_message(value.to_string())
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        Self::new_with_message(value.to_string())
    }
}

impl From<&str> for ServiceError {
    fn from(value: &str) -> Self {
        Self::new_with_str(value)
    }
}

impl From<String> for ServiceError {
    fn from(value: String) -> Self {
        Self::new_with_message(value)
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(value: DatabaseError) -> Self {
        Self::new_with_message(value.to_string())
    }
}

impl<T: Sized> From<PoisonError<T>> for ServiceError {
    fn from(_value: PoisonError<T>) -> Self {
        ServiceError::from(MUTEX_ERROR_MESSAGE)
    }
}
