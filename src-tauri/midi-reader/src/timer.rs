use std::{ops::Deref, time::Duration};

use nodi::{
    timers::{sleep, Ticker},
    Timer,
};

use crate::{
    midi_file::{PlayBackCallback, ReadingState},
    ArcMutex,
};

const GAME_PAUSE_CHECK_DELAY_MS: u32 = 70;

#[derive(Debug)]
pub struct MidiPauserTimer<P: PlayBackCallback> {
    check_delay: u32,
    ticker: Ticker,
    reading_state: ArcMutex<ReadingState>,
    pause_callback: ArcMutex<P>,
    elapsed_time: ArcMutex<Duration>,
}

impl<P: PlayBackCallback> MidiPauserTimer<P> {
    pub fn new(
        ticker: Ticker,
        reading_state: ArcMutex<ReadingState>,
        pause_callback: ArcMutex<P>,
        elapsed_time: ArcMutex<Duration>,
    ) -> Self {
        Self {
            check_delay: GAME_PAUSE_CHECK_DELAY_MS,
            ticker,
            reading_state,
            pause_callback,
            elapsed_time,
        }
    }

    fn count_sleep(&self, duration: Duration) {
        if let Ok(mut t) = self.elapsed_time.lock() {
            *t += duration;
        }
        sleep(duration)
    }
}

impl<P: PlayBackCallback> Timer for MidiPauserTimer<P> {
    fn sleep_duration(&mut self, n_ticks: u32) -> std::time::Duration {
        self.ticker.sleep_duration(n_ticks)
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.ticker.change_tempo(tempo)
    }

    fn sleep(&mut self, n_ticks: u32) {
        let mut ms = self.sleep_duration(n_ticks);
        let check_delay_duration = Duration::from_micros(self.check_delay.into());
        while ms > check_delay_duration {
            let mut emitted_pause = false;
            loop {
                if let Ok(m) = self.reading_state.lock() {
                    match *m {
                        ReadingState::Paused => {
                            if !emitted_pause {
                                if let Ok(c) = self.pause_callback.deref().lock() {
                                    c.on_pause();
                                }
                            }
                            emitted_pause = true;
                            drop(m);
                            self.count_sleep(check_delay_duration)
                        }
                        ReadingState::Playing => break,
                        _ => return,
                    };
                }
            }
            self.count_sleep(check_delay_duration);
            ms -= check_delay_duration;
            if ms > check_delay_duration && ms > Duration::ZERO {
                sleep(ms);
            }
        }
    }
}
