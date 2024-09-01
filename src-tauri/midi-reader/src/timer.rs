use std::{ops::Deref, time::Duration};

use nodi::{
    timers::{sleep, Ticker},
    Timer,
};
#[cfg(feature = "verbose")]
use paris::info;

use crate::{
    midi_file::{PlayBackCallback, ReadingState},
    ArcMutex,
};

const GAME_PAUSE_CHECK_DELAY_MS: u32 = 15;

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
    fn sleep_duration(&mut self, n_ticks: u32) -> Duration {
        self.ticker.sleep_duration(n_ticks)
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.ticker.change_tempo(tempo)
    }

    fn sleep(&mut self, n_ticks: u32) {
        let mut ms = self.sleep_duration(n_ticks);
        let check_delay_duration = Duration::from_micros(self.check_delay.into());
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
                if let Ok(m) = self.reading_state.lock() {
                    match *m {
                        ReadingState::Paused => {
                            if !emitted_pause {
                                #[cfg(feature = "verbose")]
                                {
                                    info!("Pause state reached!");
                                }
                                if let Ok(c) = self.pause_callback.deref().lock() {
                                    #[cfg(feature = "verbose")]
                                    {
                                        info!("Calling on pause");
                                    }
                                    c.on_pause();
                                    emitted_pause = true;
                                }
                            }
                            drop(m);
                            #[cfg(feature = "verbose")]
                            {
                                info!("Sleeping timer for {}", check_delay_duration.as_micros());
                            }
                            self.count_sleep(check_delay_duration)
                        }
                        ReadingState::Playing => break,
                        _ => return,
                    };
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
            ms -= check_delay_duration;
            #[cfg(feature = "verbose")]
            {
                info!(
                    "Finished sleeping, remaining time is now {}",
                    ms.as_micros()
                );
            }
            if ms > check_delay_duration && ms > Duration::ZERO {
                sleep(ms);
            }
        }
    }
}
