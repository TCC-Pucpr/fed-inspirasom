use crate::storage::StorageError::{
    CouldNotCreateStorage, KeyNotFound, StorageAlreadyExists, StorageCommitError,
    StorageDoesNotExist, StorageWriteError,
};
use anyhow::anyhow;
#[cfg(feature = "verbose")]
use paris::{error, success};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database file in path `{0}` does not exist")]
    StorageDoesNotExist(String),
    #[error("Error while creating Database file at `{0}`")]
    CouldNotCreateStorage(String),
    #[error("Database file at `{0}` already exists")]
    StorageAlreadyExists(String),
    #[error("Could not acquire database from mutex")]
    StorageIsLocked,
    #[error("Error while writing to key {key}")]
    StorageWriteError {
        key: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("Error while reading key `{0}`")]
    StorageReadError(String),
    #[error("Value with key `{0}` does not exist")]
    KeyNotFound(String),
    #[error("Error while dumping storage changes")]
    StorageCommitError(#[source] anyhow::Error),
}

pub struct Store {
    store: PickleDb,
    pub has_pending_changes: bool,
}

pub trait StorageRetrievable
where
    Self: DeserializeOwned,
{
    fn load_from_store(store: &Store, key: &str) -> StorageResult<Self> {
        store.get_value(key)
    }
    fn load_or_else(store: &Store, key: &str, default: Self) -> Self {
        store.get_value(key).unwrap_or(default)
    }
    fn load_or_default(store: &Store, key: &str) -> Self
    where
        Self: Default,
    {
        store.get_or_default(key)
    }
}

pub trait StorageSavable
where
    Self: Serialize + Sized,
{
    /// Get a new key based on the key received to save self in.
    /// If an error is returned here, cancels the entire operation
    fn transform_key(&self, _store: &mut Store, key: &str) -> StorageResult<String> {
        Ok(String::from(key))
    }
    /// Get a new value to saved in place of self, this is called before [`Self::transform_key`],
    /// so the `key` parameter here is the original key.
    /// If an error is returned here, cancels the entire operation
    #[allow(unused_variables)]
    fn transform_value(&self, store: &mut Store, key: &str) -> StorageResult<&Self> {
        Ok(&self)
    }
    fn save_to_store(&self, store: &mut Store, key: &str) -> StorageResult<()> {
        let val = self.transform_value(store, key)?;
        let k = self.transform_key(store, key)?;
        store.set_value(&k, val)
    }
}

impl Store {
    pub fn load_or_create_new(file_path: &str) -> StorageResult<Self> {
        Self::from_file(file_path).or_else(move |e| {
            #[cfg(feature = "verbose")]
            error!("Error when loading store: {}", e);
            Self::create_new(file_path)
        })
    }
    pub fn set_value(&mut self, key: &str, value: &impl Serialize) -> StorageResult<()> {
        let res = self
            .store
            .set(key, value)
            .map_err(move |e| StorageWriteError {
                key: key.to_string(),
                source: anyhow!(e),
            });
        if res.is_ok() {
            self.has_pending_changes = true;
        }
        res
    }
    pub fn remove_value(&mut self, key: &str) -> StorageResult<()> {
        match self.store.rem(key) {
            Ok(success) => {
                if success {
                    self.has_pending_changes = true;
                    Ok(())
                } else {
                    Err(KeyNotFound(key.to_string()))
                }
            }
            Err(e) => Err(StorageWriteError {
                key: key.to_string(),
                source: anyhow!(e),
            }),
        }
    }
    pub fn get_value<V: DeserializeOwned>(&self, key: &str) -> StorageResult<V> {
        if let Some(v) = self.store.get(key) {
            Ok(v)
        } else {
            Err(KeyNotFound(key.to_string()))
        }
    }
    pub fn get_or_default<V: Default + DeserializeOwned>(&self, key: &str) -> V {
        if let Ok(v) = self.get_value(key) {
            v
        } else {
            Default::default()
        }
    }
    pub fn create_new(file: &str) -> StorageResult<Self> {
        let path = Path::new(file);
        if path.exists() {
            return Err(StorageAlreadyExists(file.to_string()));
        }
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).map_err(move |e| CouldNotCreateStorage(file.to_string()))?;
        let mut store = PickleDb::new(
            file,
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        );
        match store.dump() {
            Ok(_) => {
                #[cfg(feature = "verbose")]
                success!("Database successfully created at {}", file);
                Ok(Self {
                    store,
                    has_pending_changes: false,
                })
            }
            Err(_) => Err(CouldNotCreateStorage(file.to_string())),
        }
    }
    pub fn from_file(file: &str) -> StorageResult<Self> {
        match PickleDb::load(
            file,
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        ) {
            Ok(store) => {
                #[cfg(feature = "verbose")]
                success!("Database at {} successfully loaded", file);
                Ok(Self {
                    store,
                    has_pending_changes: false,
                })
            }
            Err(_) => Err(StorageDoesNotExist(file.to_string())),
        }
    }

    pub fn commit(&mut self) -> StorageResult<()> {
        self.store
            .dump()
            .map_err(move |e| StorageCommitError(anyhow!(e)))
    }
}

impl TryFrom<&str> for Store {
    type Error = StorageError;
    fn try_from(value: &str) -> StorageResult<Self> {
        Self::load_or_create_new(value)
    }
}

impl TryFrom<String> for Store {
    type Error = StorageError;
    fn try_from(value: String) -> StorageResult<Self> {
        Self::load_or_create_new(&value)
    }
}

impl StorageSavable for u64 {}
impl StorageSavable for u32 {}
impl StorageSavable for i64 {}
impl StorageSavable for i32 {}
impl StorageSavable for String {}
impl StorageSavable for &str {}
impl StorageRetrievable for u64 {}
impl StorageRetrievable for u32 {}
impl StorageRetrievable for i64 {}
impl StorageRetrievable for i32 {}
impl StorageRetrievable for String {}
