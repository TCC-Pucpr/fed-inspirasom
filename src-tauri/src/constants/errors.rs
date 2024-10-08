use const_format::concatcp;
use std::fmt::Display;
use thiserror::Error;

const FILE_ERRORS_PREFIX: char = 'F';
const MIDI_ERRORS: char = 'M';
const STATE_ERRORS_PREFIX: char = 'S';
const TAURI_ERRORS_PREFIX: char = 'T';
const STORAGE_ERRORS_PREFIX: &str = "ST";
const INPUT_DEVICE_ERRORS_PREFIX: char = 'I';
const DATABASE_RELATED_ERRORS_PREFIX: char = 'D';

#[derive(Error, Debug)]
pub struct CodedError {
    pub code: &'static str,
    pub message: &'static str,
}

impl Display for CodedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Code: {} | Message: {}", self.code, self.message)
    }
}

pub const UNEXPECTED_ERROR: CodedError = CodedError {
    code: "0",
    message: "Unexpected internal error",
};

// TAURI RELATED ERRORS
pub const COULDNT_GET_PATH: CodedError = CodedError {
    code: concatcp!(TAURI_ERRORS_PREFIX, "01"),
    message: "Could not get current path",
};

// MIDI FILE RELATED ERRORS
pub const FILE_TOO_LONG: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "01"),
    message: "Midi file length is too long",
};

pub const FILE_IS_NOT_A_MIDI: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "02"),
    message: "File is not a midi",
};

pub const FILE_NOT_FOUND: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "03"),
    message: "file not found",
};

pub const FILE_ID_NOT_FOUND: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "04"),
    message: "Music ID not found",
};

pub const FILE_LOAD_ERROR: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "05"),
    message: "Error while trying to load the file",
};

pub const NO_FILE_BEING_PLAYED: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "06"),
    message: "There is no file being played",
};

pub const FILE_ALREADY_PLAYING: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "07"),
    message: "There is already a file being played",
};

pub const FILE_NAME_ALREADY_EXIST: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "08"),
    message: "A file with this name already exists"
};

pub const FILE_COULD_NOT_READ_PATH: CodedError = CodedError {
    code: concatcp!(FILE_ERRORS_PREFIX, "09"),
    message: "Could not read file location",
};

// STATE RELATED ERRORS
pub const STATE_ACQUIRE_ERROR: CodedError = CodedError {
    code: concatcp!(STATE_ERRORS_PREFIX, "01"),
    message: "Couldnt acquire state",
};

// MIDI CONNECTION RELATED ERRORS
pub const MIDI_OUTPUT_CONNECTION_FAILED: CodedError = CodedError {
    code: concatcp!(MIDI_ERRORS, "01"),
    message: "Failed to connect to a midi output",
};
pub const MIDI_NO_AVAILABLE_PORTS: CodedError = CodedError {
    code: concatcp!(MIDI_ERRORS, "02"),
    message: "There are no midi output ports available on this device",
};
pub const MIDI_UNEXPECTED_PLAYBACK_ERROR: CodedError = CodedError {
    code: concatcp!(MIDI_ERRORS, "03"),
    message: "An error occurred during file playback",
};
pub const MIDI_NOT_SUPPORTED: CodedError = CodedError {
    code: concatcp!(MIDI_ERRORS, "03"),
    message: "This midi file is not supported",
};

// KEY VALUE STORAGE RELATED ERRORS
pub const STORAGE_COULD_NOT_BE_CREATED: CodedError = CodedError {
    code: concatcp!(STORAGE_ERRORS_PREFIX, "01"),
    message: "Could not create key value storage",
};

pub const STORAGE_COULD_NOT_WRITE: CodedError = CodedError {
    code: concatcp!(STORAGE_ERRORS_PREFIX, "02"),
    message: "Could not write to storage",
};
pub const STORAGE_COULD_NOT_READ: CodedError = CodedError {
    code: concatcp!(STORAGE_ERRORS_PREFIX, "03"),
    message: "Could not read storage",
};
pub const STORAGE_KEY_DOES_NOT_EXIST: CodedError = CodedError {
    code: concatcp!(STORAGE_ERRORS_PREFIX, "04"),
    message: "Key does not have a value set",
};
pub const STORAGE_HAS_NOT_BEEN_CREATED: CodedError = CodedError {
    code: concatcp!(STORAGE_ERRORS_PREFIX, "05"),
    message: "Storage file has not been created",
};

// MIDI DEVICE RELATED ERRORS
pub const DEVICE_NO_INPUT_CONNECTIONS_FOUND: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "01"),
    message: "There are no available input connections",
};

pub const DEVICE_LISTEN_ERROR: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "02"),
    message: "Error occurred while listening to device input",
};

pub const DEVICE_PORT_NOT_FOUND: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "03"),
    message: "Midi input device could not be found",
};
pub const DEVICE_COULD_NOT_CONNECT: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "04"),
    message: "Could not connect to device",
};
pub const DEVICE_ALREADY_CONNECTED: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "05"),
    message: "There is another device already connected",
};
pub const DEVICE_NOT_CONNECTED: CodedError = CodedError {
    code: concatcp!(INPUT_DEVICE_ERRORS_PREFIX, "06"),
    message: "There is no device connected",
};

// DATABASE RELATED ERRORS
pub const DATABASE_COULD_NOT_LOAD: CodedError = CodedError {
    code: concatcp!(DATABASE_RELATED_ERRORS_PREFIX, "01"),
    message: "Could not load database",
};
pub const DATABASE_COULD_NOT_CREATE: CodedError = CodedError {
    code: concatcp!(DATABASE_RELATED_ERRORS_PREFIX, "02"),
    message: "Could not create database",
};
pub const COULD_NOT_UPDATE_DATABASE: CodedError = CodedError {
    code: concatcp!(DATABASE_RELATED_ERRORS_PREFIX, "03"),
    message: "Could not insert into database",
};
pub const DATABASE_QUERY_ERROR: CodedError = CodedError {
    code: concatcp!(DATABASE_RELATED_ERRORS_PREFIX, "04"),
    message: "Error while performing query operation",
};
pub const DATABASE_NO_VALUES_FOUND: CodedError = CodedError {
    code: concatcp!(DATABASE_RELATED_ERRORS_PREFIX, "05"),
    message: "No values could be found for the requested operation",
};



