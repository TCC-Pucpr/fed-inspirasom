use crate::RESOURCES_FOLDER;
use pickledb::error::Error;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use tauri::App;

type ScoreStateResult<T> = Result<T, ScoreStateError>;

const DB_NAME: &str = "scores.db";

#[derive(Debug)]
pub struct ScoreStateError {
    pub msg: String,
}
impl From<Error> for ScoreStateError {
    fn from(value: Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

pub trait Storage {
    fn get<V: DeserializeOwned>(&self, key: &str) -> Option<V>;
    fn set<V: Serialize>(&mut self, key: &str, value: &V) -> bool;
    fn remove(&mut self, key: &str) -> bool;
    fn has_value(&self, key: &str) -> bool;
}

pub struct StoreState {
    pub db: PickleDb,
    has_changed_values: bool,
}

impl StoreState {
    pub fn from_file(file_path: &str) -> ScoreStateResult<Self> {
        let file = file_path.to_string() + DB_NAME;
        let db = if Path::new(&file).exists() {
            PickleDb::new(
                file,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )
        } else {
            PickleDb::load(
                file,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )
            .map_err(move |e| ScoreStateError::from(e))?
        };
        Ok(Self {
            db,
            has_changed_values: false,
        })
    }
}

impl From<&str> for StoreState {
    fn from(value: &str) -> Self {
        Self::from_file(value).unwrap()
    }
}

impl From<String> for StoreState {
    fn from(value: String) -> Self {
        Self::from_file(&value).unwrap()
    }
}

impl From<App> for StoreState {
    fn from(value: App) -> Self {
        Self::from(&value)
    }
}

impl From<&App> for StoreState {
    fn from(value: &App) -> Self {
        let f = value
            .path_resolver()
            .resolve_resource(RESOURCES_FOLDER)
            .unwrap();
        let path = f.display().to_string() + DB_NAME;
        Self::from(path)
    }
}
