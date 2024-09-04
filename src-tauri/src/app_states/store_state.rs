use crate::app_states::store_state::ScoreStateError::{
    CouldNotCreateDb, DatabaseDoesNotExist, DbAlreadyExists,
};
use crate::get_resources_path;
use paris::{error, info};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;
use tauri::App;
use thiserror::Error;

type ScoreStateResult<T> = Result<T, ScoreStateError>;

const DB_NAME: &str = "scores.db";


#[derive(Debug, Error)]
pub enum ScoreStateError {
    #[error("Database file in path `{0}` does not exist")]
    DatabaseDoesNotExist(String),
    #[error("Error while creating Database file at `{0}`")]
    CouldNotCreateDb(String),
    #[error("Database file at `{0}` already exists")]
    DbAlreadyExists(String),
}

pub struct StoreState {
    pub db: PickleDb,
}

impl StoreState {
    fn db_file_path(file_path: &str) -> String {
        format!("{}/{}", file_path, DB_NAME)
    }
    pub fn create_new_or_load(file_path: &str) -> ScoreStateResult<Self> {
        Self::create_new(file_path).or_else(move |e| {
            error!("Error when trying to create db: {}", e);
            Self::from_file(file_path)
        })
    }
    pub fn create_new(file_path: &str) -> ScoreStateResult<Self> {
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
                Ok(Self { db })
            }
            Err(_) => Err(CouldNotCreateDb(file)),
        }
    }
    pub fn from_file(file_path: &str) -> ScoreStateResult<Self> {
        let file = Self::db_file_path(file_path);
        match PickleDb::load(
            file.clone(),
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        ) {
            Ok(db) => {
                info!("Database at {} successfully loaded", file);
                Ok(Self { db })
            }
            Err(_) => Err(DatabaseDoesNotExist(file)),
        }
    }
}

impl TryFrom<&str> for StoreState {
    type Error = ScoreStateError;
    fn try_from(value: &str) -> ScoreStateResult<Self> {
        Self::create_new_or_load(value)
    }
}

impl TryFrom<String> for StoreState {
    type Error = ScoreStateError;
    fn try_from(value: String) -> ScoreStateResult<Self> {
        Self::create_new_or_load(&value)
    }
}

impl TryFrom<App> for StoreState {
    type Error = ScoreStateError;
    fn try_from(value: App) -> ScoreStateResult<Self> {
        Self::try_from(&value)
    }
}

impl TryFrom<&App> for StoreState {
    type Error = ScoreStateError;
    fn try_from(value: &App) -> ScoreStateResult<Self> {
        let f = get_resources_path(value);
        let path = f.display().to_string();
        Self::try_from(path)
    }
}
