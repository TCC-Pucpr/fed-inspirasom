use crate::music_json_loader::{add_musics, remove_musics};
use sea_orm_migration::prelude::*;

const JSON: &str = "09092024_defaults.json";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        add_musics(manager, JSON).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        remove_musics(manager, JSON).await
    }
}
