use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../src/app/core/model/ServiceError.ts")]
pub struct ServiceError {
    pub code: String,
    pub message: String,
}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | Code: {}", self.message, self.code)
    }
}
impl Error for ServiceError {}

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
