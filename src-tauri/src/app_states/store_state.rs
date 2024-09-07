use crate::app_states::store_state::StorageError::{
    CouldNotCreateDb, DatabaseDoesNotExist, DbAlreadyExists, DbIsLocked, DbReadError, DbWriteError,
    KeyNotFound,
};
use crate::get_resources_path;
use anyhow::anyhow;
use paris::{error, info};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use std::sync::Mutex;
use tauri::App;
use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

const DB_NAME: &str = "inspire_music_data.db";

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database file in path `{0}` does not exist")]
    DatabaseDoesNotExist(String),
    #[error("Error while creating Database file at `{0}`")]
    CouldNotCreateDb(String),
    #[error("Database file at `{0}` already exists")]
    DbAlreadyExists(String),
    #[error("Could not acquire database from mutex")]
    DbIsLocked,
    #[error("Error while writing to key {key}")]
    DbWriteError {
        key: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("Error while reading key `{0}`")]
    DbReadError(String),
    #[error("Value with key `{0}` does not exist")]
    KeyNotFound(String),
}

pub struct StoreState {
    pub db: Mutex<PickleDb>,
}

impl StoreState {
    fn db_file_path(file_path: &str) -> String {
        format!("{}/{}", file_path, DB_NAME)
    }
    pub fn load_or_create_new(file_path: &str) -> StorageResult<Self> {
        Self::from_file(file_path).or_else(move |e| {
            error!("Error when loading store: {}", e);
            Self::create_new(file_path)
        })
    }
    pub fn set_value(&self, key: &str, value: &impl Serialize) -> StorageResult<()> {
        if let Ok(mut db) = self.db.lock() {
            db.set(key, value).map_err(move |e| DbWriteError {
                key: key.to_string(),
                source: anyhow!(e),
            })
        } else {
            Err(DbIsLocked)
        }
    }
    pub fn remove_value(&self, key: &str) -> StorageResult<()> {
        if let Ok(mut db) = self.db.lock() {
            match db.rem(key) {
                Ok(success) => {
                    if success {
                        Ok(())
                    } else {
                        Err(KeyNotFound(key.to_string()))
                    }
                }
                Err(e) => Err(DbWriteError {
                    key: key.to_string(),
                    source: anyhow!(e),
                }),
            }
        } else {
            Err(DbReadError(key.to_string()))
        }
    }
    pub fn get_value<V: DeserializeOwned>(&self, key: &str) -> StorageResult<V> {
        if let Ok(db) = self.db.lock() {
            if let Some(v) = db.get(key) {
                Ok(v)
            } else {
                Err(KeyNotFound(key.to_string()))
            }
        } else {
            Err(DbReadError(key.to_string()))
        }
    }
    pub fn get_or_default<V: Default + DeserializeOwned>(&self, key: &str) -> V {
        if let Ok(v) = self.get_value(key) {
            v
        } else {
            Default::default()
        }
    }
    pub fn create_new(file_path: &str) -> StorageResult<Self> {
        let file = Self::db_file_path(file_path);
        if Path::new(&file).exists() {
            return Err(DbAlreadyExists(file));
        }
        let mut db = PickleDb::new(
            file.clone(),
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        );
        match db.dump() {
            Ok(_) => {
                info!("Database successfully created at {}", file);
                Ok(Self {
                    db: Mutex::from(db),
                })
            }
            Err(_) => Err(CouldNotCreateDb(file)),
        }
    }
    pub fn from_file(file_path: &str) -> StorageResult<Self> {
        let file = Self::db_file_path(file_path);
        match PickleDb::load(
            file.clone(),
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        ) {
            Ok(db) => {
                info!("Database at {} successfully loaded", file);
                Ok(Self {
                    db: Mutex::from(db),
                })
            }
            Err(_) => Err(DatabaseDoesNotExist(file)),
        }
    }
}

impl TryFrom<&str> for StoreState {
    type Error = StorageError;
    fn try_from(value: &str) -> StorageResult<Self> {
        Self::load_or_create_new(value)
    }
}

impl TryFrom<String> for StoreState {
    type Error = StorageError;
    fn try_from(value: String) -> StorageResult<Self> {
        Self::load_or_create_new(&value)
    }
}

impl TryFrom<App> for StoreState {
    type Error = StorageError;
    fn try_from(value: App) -> StorageResult<Self> {
        Self::try_from(&value)
    }
}

impl TryFrom<&App> for StoreState {
    type Error = StorageError;
    fn try_from(value: &App) -> StorageResult<Self> {
        let f = get_resources_path(value);
        let path = f.display().to_string();
        Self::try_from(path)
    }
}
