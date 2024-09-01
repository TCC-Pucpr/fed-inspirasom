use std::ops::{Deref, Not};
use std::time::Duration;

use crate::{
    midi_file::{PlayBackCallback, ReadingState},
    ArcMutex,
};
use nodi::{
    timers::{sleep, Ticker},
    Timer,
};
#[cfg(feature = "verbose")]
use paris::info;

const GAME_PAUSE_CHECK_DELAY_MS: u32 = 4_000;

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
        if !duration.is_zero() {
            sleep(duration)
        }
    }

    fn current_state(&self) -> ReadingState {
        if let Ok(m) = self.reading_state.lock() {
            let rs = m.clone();
            drop(m);
            rs
        } else {
            ReadingState::Stoped
        }
    }

    fn on_pause(&self) {
        if let Ok(c) = self.pause_callback.deref().lock() {
            #[cfg(feature = "verbose")]
            {
                info!("Calling on pause");
            }
            c.on_pause();
        }
    }

    fn check_pause_and_sleep(&self, emitted_pause: &mut bool, dur: Duration) -> ReadingState {
        let cs = self.current_state();
        if cs == ReadingState::Paused {
            if emitted_pause.not() {
                #[cfg(feature = "verbose")]
                {
                    info!("Pause state reached!");
                }
                self.on_pause();
                *emitted_pause = true;
            }
            #[cfg(feature = "verbose")]
            {
                info!("Sleeping timer for {}", dur.as_micros());
            }
            self.count_sleep(dur)
        };
        cs
    }
}

impl<P: PlayBackCallback> Timer for MidiPauserTimer<P> {
    fn sleep_duration(&mut self, n_ticks: u32) -> Duration {
        self.ticker.sleep_duration(n_ticks)
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.ticker.change_tempo(tempo)
    }

    fn sleep(&mut self, n_ticks: u32) {
        let mut ms = self.ticker.sleep_duration(n_ticks);
        if ms.is_zero() {
            return;
        }
        let check_delay_duration = Duration::from_micros(self.check_delay.into());

        if ms < check_delay_duration {
            self.check_pause_and_sleep(&mut false, ms);
            return;
        }
        #[cfg(feature = "verbose")]
        {
            info!("Starting to sleep, the full duration of this will be {} ms, and will be sleeping for {} ms before checking state",
                ms.as_micros(),
                check_delay_duration.as_micros()
            );
            info!("Also, the amount of ticks is: {}", n_ticks);
        }
        while ms > check_delay_duration {
            let mut emitted_pause = false;
            loop {
                match self.check_pause_and_sleep(&mut emitted_pause, check_delay_duration) {
                    ReadingState::Playing => break,
                    ReadingState::Stoped | ReadingState::NotRunning => return,
                    ReadingState::Paused => {}
                }
            }
            #[cfg(feature = "verbose")]
            {
                info!(
                    "[Playing] Sleeping timer for {}",
                    check_delay_duration.as_micros()
                );
            }
            self.count_sleep(check_delay_duration);
            #[cfg(feature = "verbose")]
            {
                info!(
                    "Finished sleeping, remaining time is now {}",
                    ms.as_micros()
                );
            }
            let next_dur = ms.saturating_sub(check_delay_duration);
            if next_dur.is_zero() {
                self.count_sleep(ms);
            }
            ms = next_dur;
        }
    }
}
