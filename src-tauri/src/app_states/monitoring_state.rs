use crate::commands::OnNotePrecision;
use anyhow::{anyhow, Error};
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

const BUFFER_SIZE: usize = 3;
const DEFAULT_ERROR_MSG: &str = "Unexpected error while updating monitor data";

type MonitorResult<T> = Result<T, Error>;

#[derive(Default)]
pub struct MonitoringState {
    pub(crate) data: Mutex<Option<MonitoringData>>
}

pub(crate) struct MonitoringData {
    pub highest_breath_time: Duration,
    pub total_breath_time: Duration,
    pub total_great_scores: u32,
    pub total_ok_scores: u32,
    pub total_close_scores: u32,
    pub total_misses: u32,
    pub total_early_misses: u32,
    pub average_strength: i32,
    pub breath_strength_buffer: VecDeque<u8>,
    pub current_breath_strengths: Vec<u8>,
    pub breath_start_time: Duration,
}

impl MonitoringState {
    fn lock(&self, lock: impl Fn(&mut MonitoringData) -> MonitorResult<()>) -> MonitorResult<()> {
        if let Ok(mut data) = self.data.lock() {
            let d = if let Some(d) = data.deref_mut() {
                d
            } else {
                *data = Some(MonitoringData::default());
                let Some(a) = data.deref_mut() else {
                    return Err(anyhow!(DEFAULT_ERROR_MSG));
                };
                a
            };
            lock(d)
        } else {
            Err(anyhow!(DEFAULT_ERROR_MSG))
        }
    }
    pub fn receive_breath_data(&self, strength_byte: u8, state: bool) -> MonitorResult<()> {
        self.lock(move |data| {
            data.receive_breath_data(strength_byte, state);
            Ok(())
        })
    }
    
    pub fn receive_score(&self, precision: OnNotePrecision) -> MonitorResult<()> {
        self.lock(move |data| {
            data.receive_note(precision);
            Ok(())
        })
    }
    
    pub fn reset(&self) {
        self.data.lock().unwrap().take();
    }
}

impl Default for MonitoringData {
    fn default() -> Self {
        MonitoringData {
            highest_breath_time: Default::default(),
            total_breath_time: Default::default(),
            total_great_scores: Default::default(),
            total_close_scores: Default::default(),
            total_ok_scores: Default::default(),
            total_misses: Default::default(),
            total_early_misses: Default::default(),
            average_strength: Default::default(),
            breath_strength_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            current_breath_strengths: Default::default(),
            breath_start_time: Default::default(),
        }
    }
}

impl MonitoringData {
    #[inline]
    fn duration_from_epoch() -> Duration {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }
    pub(super) fn receive_note(&mut self, precision: OnNotePrecision) {
        match precision {
            OnNotePrecision::Middle => {
                self.total_great_scores += 1
            }
            OnNotePrecision::Left => {
                self.total_ok_scores += 1
            }
            OnNotePrecision::Right => {
                self.total_close_scores += 1
            }
            OnNotePrecision::Miss => {
                self.total_early_misses += 1
            }
            OnNotePrecision::EarlyMiss => {
                self.total_early_misses += 1
            }
        }
    }
    /// recebe a forca do ultimo sinal emitido.
    /// manda 0 para sinalizar que parou
    pub(crate) fn receive_breath_data(&mut self, strength_byte: u8, state: bool) {
        if strength_byte == 0 || !state {
            if self.breath_strength_buffer.len() == 0 {
                return;
            }
            let breath_time = Self::duration_from_epoch();
            let breath_time = breath_time - self.breath_start_time;
            if self.highest_breath_time < breath_time {
                self.highest_breath_time = breath_time;
            }
            self.total_breath_time += breath_time;
            self.breath_start_time = Duration::ZERO;
            let mut current_sum: usize = 0;
            for byte in self.current_breath_strengths.iter() {
                current_sum += *byte as usize;
            }
            self.breath_strength_buffer.clear();
            self.current_breath_strengths.clear();
            let avg = current_sum / self.current_breath_strengths.len();
            self.average_strength = if self.average_strength > 0 {
                (self.average_strength + avg as i32) / 2
            } else {
                avg as i32
            };
            return;
        }
        let buffer_len = self.breath_strength_buffer.len();
        if buffer_len == 0 {
            self.breath_start_time = Self::duration_from_epoch();
            self.current_breath_strengths.push(strength_byte);
        } else {
            if !self.breath_strength_buffer.contains(&strength_byte) {
                self.current_breath_strengths.push(strength_byte);
            }
            if buffer_len >= BUFFER_SIZE {
                self.breath_strength_buffer.pop_front();
            }
        }
        self.breath_strength_buffer.push_back(strength_byte);
    }
}