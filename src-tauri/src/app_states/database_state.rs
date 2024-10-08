use crate::app_states::database_state::DatabaseError::{CouldNotConnect, MigrationError};
use crate::constants::dirs::DB_PROTOCOL;
use anyhow::anyhow;
use migration::{Migrator, MigratorTrait};
use paris::Logger;
use sea_orm::{Database, DatabaseConnection};
use std::fs::File;
use std::path::Path;
use thiserror::Error;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Could not connect to database on `{0}`")]
    CouldNotConnect(String, #[source] anyhow::Error),
    #[error("Could create database file on `{0}`")]
    CouldNotCreateFile(String, #[source] anyhow::Error),
    #[error("Error while running migrations on `{0}`")]
    MigrationError(String, #[source] anyhow::Error)
}

pub struct DatabaseState {
    pub db: DatabaseConnection,
}

impl DatabaseState {
    pub async fn connect(path: &str) -> DatabaseResult<Self> {
        let mut logger: Logger = Logger::new();
        let f = Path::new(path);
        if !f.exists() {
            logger.info("Database file does not exist, creating it now...");
            if let Err(e) = File::create(f) {
                let e = DatabaseError::CouldNotCreateFile(path.to_string(), anyhow!(e));
                logger.error(e.to_string());
                return Err(e);
            }
            logger.success("Database file created successfully!");
        }
        logger.info("Connecting to database...");
        let db_path = format!("{}{}", DB_PROTOCOL, path);
        match Database::connect(db_path).await {
            Ok(db) => {
                if Self::check_migration(&db, path).await? {
                    logger.done();
                    logger.info("Database is outdated, running migrations...");
                    if let Err(e) = Migrator::up(&db, None).await {
                        logger.done();
                        return Err(MigrationError(path.to_string(), anyhow!(e)));
                    }
                }
                logger.success("Database connected!");
                Ok(DatabaseState { db })
            }
            Err(err) => {
                let e = CouldNotConnect(path.to_string(), anyhow!(err));
                logger.error(e.to_string());
                Err(e)
            }
        }
    }

    async fn check_migration(db: &DatabaseConnection, path: &str) -> DatabaseResult<bool> {
        match Migrator::get_pending_migrations(db).await {
            Ok(migration) => Ok(!migration.is_empty()),
            Err(e) => Err(MigrationError(path.to_string(), anyhow!(e))),
        }
    }
}
