pub use sea_orm_migration::prelude::*;

mod music_json_loader;
mod m20220101_000001_create_table;
mod m20240908_003611_load_musics;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240908_003611_load_musics::Migration),
        ]
    }
}
