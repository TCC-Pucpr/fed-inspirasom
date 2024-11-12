pub mod errors;
pub mod midi_connection;
pub mod midi_wrapper;
pub mod note;

#[cfg(feature = "verbose")]
pub(crate) const LOG_TAG: &str = "ARD";