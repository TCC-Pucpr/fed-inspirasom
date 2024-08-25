use midi_reader_writer::midly_0_5::exports::Smf;
#[cfg(verbose)]
use paris::{info, warn, Logger};
use std::sync::{Arc, Mutex};

use std::time::Duration;

use crate::errors::{InvalidMidiFile, PlaybackError};
use crate::game_connection::GamePlayer;
use crate::midi_length_calc::calc_midi_sheet_length;
use crate::player_wrapper::PlayerWrapper;
use crate::timer::MidiPauserTimer;
use midly::{Format, Timing};
use nodi::timers::Ticker;
use nodi::Sheet;
use std::fs;

use crate::Result;

pub trait MidiFilePlayer
where
    Self: Sized,
{
    fn is_still_playing(&self) -> bool;
    fn from_file(file_location: &str) -> Result<Self>;
    fn from_bytes_vector(vector: Vec<u8>) -> Result<Self>;
    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()>;
    fn create_sheet_player<P: PlayBackCallback>(
        &mut self,
        play_back_callback: P,
    ) -> Result<PlayerWrapper<P>>;
    fn pause(&mut self);
    fn unpause(&mut self);
    fn stop(&mut self);
    fn file_length(&self) -> Duration;
}

pub trait PlayBackCallback {
    /// funcao a ser chamado quando um novo sinal de nota é recebido
    fn on_note(&self, on: bool, key: u8, vel: u8) -> bool;
    /// funcao a ser chamado quando encerra o play back sem a musica ter chego ao fim
    fn on_interrupted(&self);
    /// funcao a ser chamado quando a musica chega ao fim
    fn on_finished(&self);
    /// funcao a ser chamado quando o playback é pausado
    fn on_pause(&self);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadingState {
    Playing,
    Paused,
    Stoped,
    NotRunning,
}

pub struct MidiFile {
    sheet: Sheet,
    ticker: Ticker,
    reading_state: Arc<Mutex<ReadingState>>,
    file_length: Duration,
    elapsed_time: Arc<Mutex<Duration>>,
}

impl MidiFile {
    fn create_timer<P: PlayBackCallback>(
        &self,
        reading_state: Arc<Mutex<ReadingState>>,
        pause_callback: Arc<Mutex<P>>,
    ) -> MidiPauserTimer<P> {
        MidiPauserTimer::new(
            self.ticker,
            reading_state,
            pause_callback,
            Arc::clone(&self.elapsed_time),
        )
    }
    fn update_reading_state(&self, reading_state: ReadingState) {
        if let Ok(mut m) = self.reading_state.lock() {
            *m = reading_state;
        }
    }
    pub fn remaining_time(&self) -> Duration {
        if let Ok(t) = self.elapsed_time.lock() {
            *t
        } else {
            Duration::MAX
        }
    }
    pub fn current_state(&self) -> ReadingState {
        return if let Ok(s) = self.reading_state.lock().as_deref() {
            s.clone()
        } else {
            ReadingState::NotRunning
        };
    }
}

impl MidiFilePlayer for MidiFile {
    fn from_file(file_location: &str) -> Result<Self> {
        let file = fs::read(file_location)?;
        Self::from_bytes_vector(file)
    }

    fn from_bytes_vector(vector: Vec<u8>) -> Result<Self> {
        let smf = Smf::parse(&vector)?;
        let timer = match smf.header.timing {
            Timing::Metrical(n) => Ticker::new(n.as_int()),
            Timing::Timecode(_, _) => return Err(InvalidMidiFile.into()),
        };
        let sheet = match smf.header.format {
            Format::Parallel => Sheet::parallel(&smf.tracks),
            Format::SingleTrack | Format::Sequential => Sheet::sequential(&smf.tracks),
        };
        Ok(Self {
            file_length: calc_midi_sheet_length(&sheet, timer),
            sheet,
            ticker: timer,
            reading_state: Arc::new(Mutex::new(ReadingState::NotRunning)),
            elapsed_time: Arc::from(Mutex::new(Duration::ZERO)),
        })
    }

    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()> {
        let _ = play_back_callback;
        unimplemented!()
    }

    fn create_sheet_player<P: PlayBackCallback>(
        &mut self,
        play_back_callback: P,
    ) -> Result<PlayerWrapper<P>> {
        if let Ok(m) = self.reading_state.lock() {
            if *m != ReadingState::NotRunning {
                return Err(PlaybackError.into());
            }
        }
        let callback_arc = Arc::from(Mutex::new(play_back_callback));
        self.update_reading_state(ReadingState::Playing);
        let conn = GamePlayer::new(Arc::clone(&callback_arc), Arc::clone(&self.reading_state));
        let sheet = self.sheet.to_owned();
        let s = Arc::clone(&self.reading_state);
        let timer = self.create_timer(Arc::clone(&self.reading_state), Arc::clone(&callback_arc));
        Ok(PlayerWrapper::new(timer, conn, s, sheet))
    }

    fn pause(&mut self) {
        self.update_reading_state(ReadingState::Paused)
    }

    fn unpause(&mut self) {
        self.update_reading_state(ReadingState::Playing)
    }

    fn stop(&mut self) {
        self.update_reading_state(ReadingState::Stoped)
    }

    fn file_length(&self) -> Duration {
        self.file_length
    }

    fn is_still_playing(&self) -> bool {
        if let Ok(m) = self.reading_state.lock() {
            return match *m {
                ReadingState::NotRunning | ReadingState::Stoped => false,
                _ => true,
            };
        }
        false
    }
}
