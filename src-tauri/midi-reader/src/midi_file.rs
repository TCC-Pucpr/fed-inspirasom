use midi_reader_writer::midly_0_5::exports::Smf;

use std::time::Duration;

use crate::errors::{MidiReaderError, MidiReaderResult};
use crate::game_player::GamePlayer;
use crate::midi_length_calc::calc_midi_sheet_length;
use crate::player_wrapper::PlayerWrapper;
#[cfg(test)]
use crate::test_callback::TestCallback;
use crate::timer::MidiPauseTimer;
use anyhow::anyhow;
use midly::Format;
use nodi::timers::Ticker;
#[cfg(test)]
use nodi::Player;
use nodi::Sheet;
use std::fs;
use utils::mutable_arc::MutableArc;

pub fn load_midi_bytes(file_location: &str) -> MidiReaderResult<Vec<u8>> {
    fs::read(file_location)
        .map_err(move |_| MidiReaderError::FileDoesNotExist(file_location.to_string()))
}

pub(crate) fn create_sheet_and_ticker(vec: Vec<u8>) -> MidiReaderResult<(Sheet, Ticker)> {
    let smf = Smf::parse(&vec).map_err(move |e| MidiReaderError::InvalidMidiFile(anyhow!(e)))?;
    let timer = Ticker::try_from(smf.header.timing)
        .map_err(|e| MidiReaderError::InvalidMidiFile(anyhow!(e)))?;
    let sheet = match smf.header.format {
        Format::Parallel => Sheet::parallel(&smf.tracks),
        Format::SingleTrack | Format::Sequential => Sheet::sequential(&smf.tracks),
    };
    Ok((sheet, timer))
}

pub trait MidiFilePlayer
where
    Self: Sized,
{
    fn is_still_playing(&self) -> bool;
    fn from_file(file_location: &str) -> MidiReaderResult<Self> {
        Self::from_bytes_vector(load_midi_bytes(file_location)?)
    }
    fn from_bytes_vector(vector: Vec<u8>) -> MidiReaderResult<Self> {
        let (sheet, timer) = create_sheet_and_ticker(vector)?;
        Ok(Self::from_sheet_and_ticker(sheet, timer))
    }
    fn from_sheet_and_ticker(sheet: Sheet, ticker: Ticker) -> Self;
    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> MidiReaderResult<()>;
    fn create_sheet_player<P: PlayBackCallback>(
        &mut self,
        play_back_callback: P,
    ) -> MidiReaderResult<PlayerWrapper<P>>;
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
    reading_state: MutableArc<ReadingState>,
    file_length: Duration,
    elapsed_time: MutableArc<Duration>,
}

impl MidiFile {
    fn create_timer<P: PlayBackCallback>(
        &self,
        reading_state: MutableArc<ReadingState>,
        pause_callback: MutableArc<P>,
    ) -> MidiPauseTimer<P> {
        MidiPauseTimer::new(
            self.ticker,
            reading_state,
            pause_callback,
            self.elapsed_time.clone(),
        )
    }
    fn update_reading_state(&self, reading_state: ReadingState) {
        self.reading_state.set_data(reading_state);
    }
    pub fn remaining_time(&self) -> Duration {
        if let Some(t) = self.elapsed_time.get_data() {
            *t
        } else {
            Duration::MAX
        }
    }
    pub fn current_state(&self) -> ReadingState {
        if let Some(s) = self.reading_state.get_data() {
            s.clone()
        } else {
            ReadingState::NotRunning
        }
    }

    #[cfg(test)]
    pub(crate) fn normal_play_file(file_location: &str) {
        let file = fs::read(file_location).unwrap();
        let Smf { header, tracks } = Smf::parse(&file).unwrap();
        let timer = Ticker::try_from(header.timing).unwrap();

        let p = TestCallback;

        let timer = MidiPauseTimer::new(
            timer,
            MutableArc::from(ReadingState::Playing),
            MutableArc::from(p),
            MutableArc::from(Duration::ZERO),
        );

        let sheet = match header.format {
            Format::SingleTrack | Format::Sequential => Sheet::sequential(&tracks),
            Format::Parallel => Sheet::parallel(&tracks),
        };

        let con = TestCallback;

        let mut player = Player::new(timer, con);

        player.play(&sheet);
    }
}

impl MidiFilePlayer for MidiFile {
    fn is_still_playing(&self) -> bool {
        if let Some(m) = self.reading_state.get_data() {
            match *m {
                ReadingState::NotRunning | ReadingState::Stoped => false,
                _ => true,
            }
        } else {
            false
        }
    }

    fn from_sheet_and_ticker(sheet: Sheet, ticker: Ticker) -> Self {
        Self {
            file_length: calc_midi_sheet_length(&sheet, ticker),
            sheet,
            ticker,
            reading_state: MutableArc::from(ReadingState::NotRunning),
            elapsed_time: MutableArc::from(Duration::ZERO),
        }
    }

    fn play_music<P: PlayBackCallback>(&mut self, play_back_callback: P) -> MidiReaderResult<()> {
        let _ = play_back_callback;
        unimplemented!()
    }

    fn create_sheet_player<P: PlayBackCallback>(
        &mut self,
        play_back_callback: P,
    ) -> MidiReaderResult<PlayerWrapper<P>> {
        if let Some(m) = self.reading_state.get_data() {
            if *m != ReadingState::NotRunning {
                return Err(MidiReaderError::AlreadyPlaying);
            }
        }
        let callback_arc = MutableArc::from(play_back_callback);
        self.update_reading_state(ReadingState::Playing);
        Ok(PlayerWrapper::new(
            self.create_timer(self.reading_state.clone(), callback_arc.clone()),
            GamePlayer::new(callback_arc.clone(), self.reading_state.clone()),
            self.reading_state.clone(),
            callback_arc.clone(),
            self.sheet.to_owned(),
        ))
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
}
