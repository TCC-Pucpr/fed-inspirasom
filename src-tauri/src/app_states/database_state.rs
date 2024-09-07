use anyhow::anyhow;
use crate::RESOURCES_FOLDER;
use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;
use crate::app_states::database_state::DatabaseError::CouldNotConnect;

const PROTOCOL: &str = "sqlite://";
const DATABASE_DIR: &str = "databases/inspire.db";

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Could not connect to database on `{0}`")]
    CouldNotConnect(String, #[source] anyhow::Error),
}

pub struct DatabaseState {
    db: DatabaseConnection,
}

impl DatabaseState {
    fn full_path(context_path: &str) -> String {
        format!(
            "{}{}{}{}",
            PROTOCOL, context_path, RESOURCES_FOLDER, DATABASE_DIR
        )
    }
    pub async fn connect(context_path: &str) -> DatabaseResult<Self> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
        let path = Self::full_path(context_path);
        match Database::connect(path).await {
            Ok(db) => Ok(DatabaseState { db }),
            Err(err) => Err(CouldNotConnect(context_path.to_string(), anyhow!(err))),
        }
    }
}
