use midi_reader_writer::midly_0_5::exports::Smf;
#[cfg(feature = "verbose")]
use paris::warn;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::sync::{Arc, Weak};
use std::thread::sleep;
use std::time::Duration;

use crate::midi_connection::midi_connection;
use midly::{Format, MidiMessage, Timing};
use nodi::timers::Ticker;
use nodi::{Connection, MidiEvent, Player, Sheet, Timer};
use std::fs;

use crate::Result;

const GAME_PAUSE_CHECK_DELAY_MS: u32 = 33;

#[derive(Debug, Clone)]
struct InvalidMidiFile;
#[derive(Debug, Clone)]
struct PlaybackError;
#[derive(Debug, Clone)]
struct AlreadyPlaying;
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
impl Display for AlreadyPlaying {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The selected file is invalid!")
    }
}
impl Error for PlaybackError {}
impl Error for InvalidMidiFile {}
impl Error for AlreadyPlaying {}

#[derive(Debug, PartialEq)]
pub enum ReadingState {
    Playing,
    Paused,
    Stoped,
    NotRunning,
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

pub struct MidiFile {
    sheet: Sheet,
    ticker: Ticker,
    reading_state: Arc<ReadingState>,
}

#[derive(Debug)]
pub struct MidiPauserTimer<'a, P: PlayBackCallback> {
    check_delay: u32,
    ticker: &'a mut Ticker,
    reading_state: Weak<ReadingState>,
    pause_callback: Weak<P>,
    micro_per_tick: Duration,
}

struct GamePlayer<P: PlayBackCallback> {
    callback: Weak<P>,
    on_notes: HashSet<u8>,
    reading_state: Weak<ReadingState>,
}

pub trait MidiFilePlayer
where
    Self: Sized,
{
    fn from_file(file_location: &str) -> Result<Self>;
    fn from_bytes_vector(vector: Vec<u8>) -> Result<Self>;
    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()>;
    fn read_sheet<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()>;
    fn pause(&mut self);
    fn unpause(&mut self);
    fn stop(&mut self);
}

impl<'a, P: PlayBackCallback> MidiPauserTimer<'a, P> {
    pub fn new(
        ticker: &'a mut Ticker,
        reading_state: Weak<ReadingState>,
        pause_callback: Weak<P>,
    ) -> Self {
        Self {
            check_delay: GAME_PAUSE_CHECK_DELAY_MS,
            ticker,
            reading_state,
            micro_per_tick: Duration::ZERO,
            pause_callback,
        }
    }
}

impl<P: PlayBackCallback> GamePlayer<P> {
    pub fn new(callback: Weak<P>, reading_state: Weak<ReadingState>) -> Self {
        Self {
            callback,
            on_notes: HashSet::new(),
            reading_state,
        }
    }
}

impl MidiFile {
    fn timer<P: PlayBackCallback>(
        &mut self,
        reading_state: Weak<ReadingState>,
        pause_callback: Weak<P>,
    ) -> MidiPauserTimer<P> {
        MidiPauserTimer::new(&mut self.ticker, reading_state, pause_callback)
    }
    fn update_reading_state(&mut self, reading_state: ReadingState) {
        *Arc::get_mut(&mut self.reading_state).unwrap() = reading_state;
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
            sheet,
            ticker: timer,
            reading_state: Arc::new(ReadingState::NotRunning),
        })
    }

    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()> {
        let midi_conn = midi_connection()?;
        self.update_reading_state(ReadingState::Playing);
        let callback_arc = Arc::from(play_back_callback);
        let sheet = self.sheet.to_owned();
        let mut player = Player::new(
            self.timer(
                Arc::downgrade(&self.reading_state),
                Arc::downgrade(&callback_arc),
            ),
            midi_conn,
        );
        let play_result = player.play(&sheet);
        self.update_reading_state(ReadingState::NotRunning);
        if !play_result {
            callback_arc.on_interrupted();
            Err(PlaybackError.into())
        } else {
            callback_arc.on_finished();
            Ok(())
        }
    }

    fn read_sheet<P: PlayBackCallback>(&mut self, play_back_callback: P) -> Result<()> {
        if *self.reading_state != ReadingState::NotRunning {
            return Err(PlaybackError.into());
        }
        let callback_arc = Arc::from(play_back_callback);
        self.update_reading_state(ReadingState::Playing);
        let conn = GamePlayer::new(
            Arc::downgrade(&callback_arc),
            Arc::downgrade(&self.reading_state),
        );
        let sheet = self.sheet.to_owned();
        let mut player = Player::new(
            self.timer(
                Arc::downgrade(&self.reading_state),
                Arc::downgrade(&callback_arc),
            ),
            conn,
        );
        let play_result = player.play(&sheet);
        self.update_reading_state(ReadingState::NotRunning);
        if !play_result {
            callback_arc.on_interrupted();
            Err(PlaybackError.into())
        } else {
            callback_arc.on_finished();
            Ok(())
        }
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
}

impl<P: PlayBackCallback> Connection for GamePlayer<P> {
    fn play(&mut self, event: MidiEvent) -> bool {
        if let Some(s) = self.reading_state.upgrade() {
            match *s {
                ReadingState::Stoped => {
                    return false;
                }
                _ => {}
            }
        } else {
            #[cfg(feature = "verbose")]
            {
                warn!("Weak reference to reading state returned None, cancelling playback...")
            }
            return false;
        };
        match event.message {
            MidiMessage::NoteOff { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.remove(&k);
                if let Some(c) = self.callback.upgrade() {
                    c.on_note(false, k, vel.into())
                } else {
                    false
                }
            }
            MidiMessage::NoteOn { key, vel } => {
                let k: u8 = key.into();
                self.on_notes.insert(k);
                if let Some(c) = self.callback.upgrade() {
                    c.on_note(true, k, vel.into())
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    fn all_notes_off(&mut self) {
        if let Some(c) = self.callback.upgrade() {
            for note in &self.on_notes {
                c.on_note(false, *note, 0);
            }
        }
        self.on_notes.clear();
    }
}

impl<'a, P: PlayBackCallback> Timer for MidiPauserTimer<'a, P> {
    fn sleep_duration(&mut self, n_ticks: u32) -> std::time::Duration {
        let d = self.ticker.sleep_duration(n_ticks).to_owned();
        self.micro_per_tick = d;
        self.micro_per_tick
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.ticker.change_tempo(tempo)
    }

    fn sleep(&mut self, n_ticks: u32) {
        let mut ms = self.micro_per_tick * n_ticks;
        let check_delay_duration = Duration::from_micros(self.check_delay.into());
        let strong = if let Some(s) = self.reading_state.upgrade() {
            s
        } else {
            return;
        };
        while ms > check_delay_duration {
            let mut emitted_pause = false;
            loop {
                match *strong {
                    ReadingState::Paused => {
                        if !emitted_pause {
                            if let Some(c) = self.pause_callback.upgrade() {
                                c.on_pause();
                            }
                        }
                        emitted_pause = true;
                        sleep(check_delay_duration)
                    }
                    ReadingState::Playing => break,
                    _ => return,
                };
            }
            sleep(check_delay_duration);
            ms -= check_delay_duration;
            if ms > check_delay_duration && ms > Duration::ZERO {
                sleep(ms);
            }
        }
    }
}
