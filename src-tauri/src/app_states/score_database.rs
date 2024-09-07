use crate::RESOURCES_FOLDER;
use sea_orm::{Database, DatabaseConnection};

const PROTOCOL: &str = "sqlite://";
const DATABASE_DIR: &str = "databases/inspire.db";

struct MainDatabaseState {
    db: DatabaseConnection,
}

impl MainDatabaseState {
    fn full_path(context_path: &str) -> String {
        format!(
            "{}{}{}{}",
            PROTOCOL, context_path, RESOURCES_FOLDER, DATABASE_DIR
        )
    }
    pub async fn connect(context_path: &str) -> Self {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
        let path = Self::full_path(context_path);
        match Database::connect(path).await {
            Ok(db) => MainDatabaseState { db },
            Err(err) => panic!("{}", err),
        }
    }
}
