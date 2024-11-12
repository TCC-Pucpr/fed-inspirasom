use std::time::Duration;

use crate::midi_file::{create_sheet_and_ticker, load_midi_bytes};
use nodi::{timers::Ticker, Connection, Player, Sheet, Timer};

pub fn calculate_midi_length(file: &str) -> Duration {
    let (sheet, timer) = create_sheet_and_ticker(load_midi_bytes(file).unwrap()).unwrap();
    calc_midi_sheet_length(&sheet, timer)
}

pub(crate) fn calc_midi_sheet_length(sheet: &Sheet, ticker: Ticker) -> Duration {
    let mut dur = Duration::ZERO;
    let timer = MidiLengthCalculator::new(&mut dur, ticker);
    let conn = FakeConn;
    let mut player = Player::new(timer, conn);
    player.play(sheet);
    dur
}

struct MidiLengthCalculator<'a> {
    length: &'a mut Duration,
    ticker: Ticker,
}

struct FakeConn;

impl Connection for FakeConn {
    fn play(&mut self, event: nodi::MidiEvent) -> bool {
        let _ = event;
        true
    }
}

impl<'a> MidiLengthCalculator<'a> {
    pub fn new(length: &'a mut Duration, ticker: Ticker) -> Self {
        Self { length, ticker }
    }
}

impl<'a> Timer for MidiLengthCalculator<'a> {
    fn sleep_duration(&mut self, n_ticks: u32) -> Duration {
        self.ticker.sleep_duration(n_ticks)
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.ticker.change_tempo(tempo)
    }

    fn sleep(&mut self, n_ticks: u32) {
        let dur = self.sleep_duration(n_ticks);
        *self.length += dur;
    }
}
