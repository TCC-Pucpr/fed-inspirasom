pub mod errors;
mod game_player;
pub mod midi_file;
pub use midi_length_calc::calculate_midi_length;
mod midi_length_calc;
pub mod player_wrapper;
#[cfg(test)]
mod test_callback;
mod timer;

#[cfg(feature = "verbose")]
pub(crate) const LOG_TAG: &str = "MDIFLRDR";

#[cfg(test)]
mod tests {
    #[test]
    fn midi_read_test() {}
}
