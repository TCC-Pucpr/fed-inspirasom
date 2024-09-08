use crate::music_json_loader::load_data_file;
use entity::music::ActiveModel;
use entity::{music, score};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EntityTrait;
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
        let files = load_data_file();
        music::Entity::delete_many().exec(db).await?;
        let m: Vec<ActiveModel> = files
            .files
            .into_iter()
            .map(move |x| x.into_active_model())
            .collect();
        music::Entity::insert_many(m).exec(db).await?;
        score::Entity::delete_many().exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        music::Entity::delete_many().exec(db).await?;
        score::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
