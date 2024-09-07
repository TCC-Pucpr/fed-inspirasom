use crate::get_resources_path;
use std::ops::DerefMut;
use std::sync::Mutex;
use storage::storage::{StorageError, StorageResult, StorageRetrievable, StorageSavable, Store};
use tauri::App;

const DB_NAME: &str = "inspire_music_data.db";

pub struct StoreState {
    store: Mutex<Store>,
}

impl StoreState {
    fn db_file_path(file_path: &str) -> String {
        format!("{}/{}", file_path, DB_NAME)
    }
    pub fn load_or_create_new(file_path: &str) -> StorageResult<Self> {
        let store = Store::load_or_create_new(&Self::db_file_path(file_path))?;
        Ok(Self {
            store: Mutex::new(store),
        })
    }
    pub fn commit(&self) -> StorageResult<()> {
        if let Ok(mut store) = self.store.lock() {
            store.commit()
        } else {
            Err(StorageError::StorageIsLocked)
        }
    }
    pub fn save<V: StorageSavable>(&self, key: &str, value: &V) -> StorageResult<()> {
        if let Ok(mut store) = self.store.lock() {
            value.save_to_store(store.deref_mut(), key)
        } else {
            Err(StorageError::StorageIsLocked)
        }
    }
    pub fn retrieve<V: StorageRetrievable>(&self, key: &str) -> StorageResult<V> {
        if let Ok(store) = self.store.lock() {
            V::load_from_store(&store, key)
        } else {
            Err(StorageError::StorageIsLocked)
        }
    }
    pub fn retrieve_default<V: StorageRetrievable + Default>(&self, key: &str) -> StorageResult<V> {
        if let Ok(store) = self.store.lock() {
            Ok(V::load_or_default(&store, key))
        } else {
            Err(StorageError::StorageIsLocked)
        }
    }
    pub fn remove(&self, key: &str) -> StorageResult<()> {
        if let Ok(mut store) = self.store.lock() {
            store.remove_value(key)
        } else {
            Err(StorageError::StorageIsLocked)
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
