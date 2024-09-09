use crate::sea_orm::ActiveValue;
use crate::DbErr;
use entity::music::ActiveModel;
use entity::prelude::{Music, Score};
use entity::{music, score};
use midi_reader::calculate_midi_length;
use sea_orm_migration::sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use sea_orm_migration::SchemaManager;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;

const DATA_DIR: &str = "/migration/jsons/";
const MUSICS_DIR: &str = "../resources/musics";

#[derive(Serialize, Deserialize)]
pub(crate) struct MusicList {
    files: Vec<MusicDataObject>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MusicDataObject {
    pub(crate) name: String,
    directory: String,
}

impl MusicDataObject {
    pub(crate) fn into_active_model(self) -> ActiveModel {
        let file = format!("{}{}", MUSICS_DIR, self.directory);
        println!("Loaded file: {}", file);
        let duration = calculate_midi_length(&file);
        ActiveModel {
            name: ActiveValue::Set(self.name),
            duration: ActiveValue::Set(duration.as_secs() as i32),
            directory: ActiveValue::Set(self.directory),
            ..Default::default()
        }
    }
}

pub(crate) fn load_data_file(json_file: &str) -> Vec<MusicDataObject> {
    let dir = current_dir().unwrap().display().to_string() + DATA_DIR + json_file;
    println!("Data file dir: {}", dir);
    let file = File::open(dir).unwrap();
    let buf_reader = BufReader::new(file);
    let ml: MusicList = serde_json::from_reader(buf_reader).unwrap();
    ml.files
}

pub(crate) async fn add_musics<'a>(manager: &SchemaManager<'a>, json: &str) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let files = load_data_file(json);
    let m: Vec<ActiveModel> = files
        .into_iter()
        .map(move |x| x.into_active_model())
        .collect();
    music::Entity::insert_many(m).exec(db).await?;
    Ok(())
}

pub(crate) async fn remove_musics<'a>(manager: &SchemaManager<'a>, json_file: &str) -> Result<(), DbErr>{
    let db = manager.get_connection();
    let files = load_data_file(json_file);
    let names: Vec<String> = files.into_iter().map(move |x| x.name).collect();
    let txn = db.begin().await?;
    for n in names {
        let m = Music::find()
            .filter(music::Column::Name.eq(&n))
            .one(&txn)
            .await?;
        if let Some(m) = m {
            Score::delete_many()
                .filter(score::Column::MusicId.eq(m.id))
                .exec(&txn)
                .await?;
            m.delete(&txn).await?;
        } else {
            eprintln!("Music file with name {} not present, ignoring...", n);
        }
    }
    txn.commit().await?;
    Ok(())
}
