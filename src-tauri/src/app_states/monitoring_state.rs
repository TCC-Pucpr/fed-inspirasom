use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

const BUFFER_SIZE: usize = 3;

pub struct MonitoringState {
    data: Mutex<MonitoringData>
}

pub(crate) struct MonitoringData {
    highest_breath_time: Duration,
    total_breath_time: Duration,
    total_great_scores: u64,
    total_ok_scores: u64,
    total_close_scores: u64,
    average_strength: f64,
    breath_strength_change_count: u64,
    breath_strength_buffer: VecDeque<u8>,
    current_breath_strengths: Vec<u8>,
    breath_start_time: Duration,
}

impl Default for MonitoringData {
    fn default() -> Self {
        MonitoringData {
            highest_breath_time: Default::default(),
            total_breath_time: Default::default(),
            total_great_scores: Default::default(),
            total_close_scores: Default::default(),
            total_ok_scores: Default::default(),
            breath_strength_change_count: Default::default(),
            average_strength: Default::default(),
            breath_strength_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            current_breath_strengths: Default::default(),
            breath_start_time: Default::default(),
        }
    }
}

impl MonitoringState {}

impl MonitoringData {

    #[inline]
    fn duration_from_epoch() -> Duration {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }

    /// recebe a forca do ultimo sinal emitido.
    /// manda 0 para sinalizar que parou
    pub fn receive_breath_data(&mut self, strength_byte: u8) {
        if strength_byte == 0 {
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
            let avg = current_sum as f64 / self.current_breath_strengths.len() as f64;
            self.average_strength = (self.average_strength + avg) / 2f64;
            return;
        }
        let buffer_len = self.breath_strength_buffer.len();
        let contains: bool;
        if buffer_len == 0 {
            self.breath_strength_change_count += 1;
            self.breath_start_time = Self::duration_from_epoch();
            contains = false
        } else {
            contains = self.breath_strength_buffer.contains(&strength_byte);
            if buffer_len >= BUFFER_SIZE {
                self.breath_strength_buffer.pop_front();
            }
        }
        self.breath_strength_buffer.push_back(strength_byte);
        if !contains {
            self.current_breath_strengths.push(strength_byte);
        }
    }
    
    
}