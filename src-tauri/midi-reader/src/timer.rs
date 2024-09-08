use std::time::Duration;

use crate::midi_file::{PlayBackCallback, ReadingState};
use nodi::{
    timers::{sleep, Ticker},
    Timer,
};
#[cfg(feature = "verbose")]
use paris::info;
use utils::mutable_arc::MutableArc;

const GAME_PAUSE_CHECK_DELAY_MS: u32 = 4_000;

#[derive(Debug)]
pub struct MidiPauseTimer<P: PlayBackCallback> {
    check_delay: u32,
    ticker: Ticker,
    reading_state: MutableArc<ReadingState>,
    pause_callback: MutableArc<P>,
    elapsed_time: MutableArc<Duration>,
}

impl<P: PlayBackCallback> MidiPauseTimer<P> {
    pub fn new(
        ticker: Ticker,
        reading_state: MutableArc<ReadingState>,
        pause_callback: MutableArc<P>,
        elapsed_time: MutableArc<Duration>,
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
        if !duration.is_zero() {
            if let Some(mut t) = self.elapsed_time.get_data() {
                *t += duration;
            }
            sleep(duration)
        }
    }

    fn current_state(&self) -> ReadingState {
        if let Some(m) = self.reading_state.get_data() {
            let rs = m.clone();
            drop(m);
            rs
        } else {
            ReadingState::Stoped
        }
    }

    fn on_pause(&self) {
        if let Some(c) = self.pause_callback.get_data() {
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
            if *emitted_pause == false {
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

impl<P: PlayBackCallback> Timer for MidiPauseTimer<P> {
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
            loop {
                match self.check_pause_and_sleep(&mut false, ms) {
                    ReadingState::Playing | ReadingState::Stoped | ReadingState::NotRunning => {
                        return
                    }
                    ReadingState::Paused => {}
                }
            }
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
