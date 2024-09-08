use const_format::concatcp;

pub(crate) const RESOURCES_FOLDER: &str = "resources/";
pub(crate) const MUSICS_FOLDER: &str = "/musics/";
const DATA_FOLDER: &str = "/data/";

const STORE_NAME: &str = "inspire_music_data.db";

pub(crate) const DB_PROTOCOL: &str = "sqlite:";
const DB_NAME: &str = "inspire.db";

pub(crate) const STORE_DIR: &str = concatcp!(DATA_FOLDER, STORE_NAME);
pub(crate) const DATABASE_DIR: &str = concatcp!(DATA_FOLDER, DB_NAME);
