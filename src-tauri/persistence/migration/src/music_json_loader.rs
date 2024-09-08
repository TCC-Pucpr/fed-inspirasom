use crate::sea_orm::{ActiveValue, EntityTrait};
use entity::{music, score};
use midi_reader::calculate_midi_length;
use sea_orm_migration::prelude::ConnectionTrait;
use sea_orm_migration::DbErr;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;

const DATA_DIR: &str = "/migration/data.json";
const MUSICS_DIR: &str = "../resources/musics";

#[derive(Serialize, Deserialize)]
struct MusicList {
    files: Vec<MusicDataObject>,
}

#[derive(Serialize, Deserialize)]
struct MusicDataObject {
    name: String,
    directory: String,
}

fn load_data_file() -> MusicList {
    let dir = current_dir().unwrap().display().to_string() + DATA_DIR;
    println!("Data file dir: {}", dir);
    let file = File::open(dir).unwrap();
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

pub(crate) async fn add_musics_to_entity(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    let list = load_data_file();
    let mut musics: Vec<music::ActiveModel> = vec![];
    for m in list.files {
        let file = format!("{}{}", MUSICS_DIR, m.directory);
        println!("Loaded file: {}", file);
        let duration = calculate_midi_length(&file);
        let model = music::ActiveModel {
            name: ActiveValue::Set(m.name),
            duration: ActiveValue::Set(duration.as_secs() as i32),
            directory: ActiveValue::Set(m.directory),
            ..Default::default()
        };
        musics.push(model);
    }
    music::Entity::insert_many(musics).exec(db).await?;
    score::Entity::delete_many().exec(db).await?;
    Ok(())
}
