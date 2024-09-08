use crate::app_states::database_state::DatabaseError::{CouldNotConnect, MigrationError};
use crate::constants::dirs::{DATABASE_DIR, DB_PROTOCOL};
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
    MigrationError(String, #[source] anyhow::Error),
    #[error("Could not create database model")]
    CouldNotCreateActiveModel,
}

pub struct DatabaseState {
    pub db: DatabaseConnection,
}

impl DatabaseState {
    fn full_path(context_path: &str) -> String {
        format!("{}{}{}", DB_PROTOCOL, context_path, DATABASE_DIR)
    }
    fn full_path_no_protocol(context_path: &str) -> String {
        format!("{}{}", context_path, DATABASE_DIR)
    }
    pub async fn connect(context_path: &str) -> DatabaseResult<Self> {
        let mut logger: Logger = Logger::new();
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
        let path = Self::full_path_no_protocol(context_path);
        let f = Path::new(&path);
        if !f.exists() {
            logger.loading("Database file does not exist, creating it now...");
            if let Err(e) = File::create(f) {
                logger.done();
                return Err(DatabaseError::CouldNotCreateFile(path, anyhow!(e)));
            }
            drop(path);
            logger.done().info("Database file created successfully!");
        }
        logger.loading("Connecting to database...\n");
        let db_path = Self::full_path(context_path);
        match Database::connect(&db_path).await {
            Ok(db) => {
                if Self::check_migration(&db, &db_path).await? {
                    logger.loading("Database is outdated, running migrations...");
                    if let Err(e) = Migrator::up(&db, None).await {
                        logger.done();
                        return Err(MigrationError(db_path, anyhow!(e)));
                    }
                }
                logger.done().info("Database connected!");
                Ok(DatabaseState { db })
            }
            Err(err) => {
                logger.done();
                Err(CouldNotConnect(context_path.to_string(), anyhow!(err)))
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
