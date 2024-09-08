use crate::music_json_loader::add_musics_to_entity;
use entity::music;
use sea_orm_migration::sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MusicList {
    files: Vec<MusicDataObject>,
}

#[derive(Serialize, Deserialize)]
struct MusicDataObject {
    name: String,
    directory: String,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        add_musics_to_entity(db).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        music::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
