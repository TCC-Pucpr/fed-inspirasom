use midi_reader_writer::midly_0_5::exports::Smf;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

use crate::midi_connection::midi_connection;
use midly::{Format, MidiMessage, Timing};
use nodi::timers::Ticker;
use nodi::{Connection, MidiEvent, Player, Sheet};
use std::fs;

use crate::Result;

#[derive(Debug, Clone)]
struct InvalidMidiFile;
#[derive(Debug, Clone)]
struct PlaybackError;
impl Display for PlaybackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred during game!")
    }
}
impl Display for InvalidMidiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The selected file is invalid!")
    }
}
impl Error for PlaybackError {}
impl Error for InvalidMidiFile {}

pub trait PlayBackCallback {
    fn on_note(&self, on: bool, key: u8, vel: u8) -> bool;
}

pub struct MidiFile {
    sheet: Sheet,
    timer: Ticker,
}

struct GamePlayer {
    callback: Box<dyn PlayBackCallback>,
    on_notes: HashSet<u8>,
}

impl GamePlayer {
    pub fn new(callback: Box<dyn PlayBackCallback>) -> Self {
        Self {
            callback,
            on_notes: HashSet::new(),
        }
    }
}

impl MidiFile {
    pub fn from_file(file_location: &str) -> Result<Self> {
        let file = fs::read(file_location)?;
        Self::from_bytes_vector(file)
    }

    pub fn from_bytes_vector(vector: Vec<u8>) -> Result<Self> {
        let smf = Smf::parse(&vector)?;
        let timer = match smf.header.timing {
            Timing::Metrical(n) => Ticker::new(n.as_int()),
            Timing::Timecode(_, _) => return Err(InvalidMidiFile.into()),
        };
        let sheet = match smf.header.format {
            Format::Parallel => Sheet::parallel(&smf.tracks),
            Format::SingleTrack | Format::Sequential => Sheet::sequential(&smf.tracks),
        };
        Ok(Self { sheet, timer })
    }

    pub fn play_music(&self) -> Result<()> {
        let midi_conn = midi_connection()?;
        let mut player = Player::new(self.timer, midi_conn);
        if !player.play(&self.sheet) {
            Err(Box::new(PlaybackError))
        } else {
            Ok(())
        }
    }

    pub fn read_sheet(&self, play_back_callback: Box<dyn PlayBackCallback>) -> Result<()> {
        let fake_conn = GamePlayer::new(play_back_callback);
        let mut player = Player::new(self.timer, fake_conn);
        if !player.play(&self.sheet) {
            Err(Box::new(PlaybackError))
        } else {
            Ok(())
        }
    }
}

impl Connection for GamePlayer {
    fn play(&mut self, event: MidiEvent) -> bool {
        match event.message {
            MidiMessage::NoteOff { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.remove(&k);
                self.callback.on_note(false, k, vel.into())
            }
            MidiMessage::NoteOn { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.insert(k);
                self.callback.on_note(true, k, vel.into())
            }
            _ => true,
        }
    }

    fn all_notes_off(&mut self) {
        for note in &self.on_notes {
            self.callback.on_note(false, *note, 0);
        }
        self.on_notes.clear();
    }
}
