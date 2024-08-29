use crate::{
    constants::events_name::MIDI_NOTE, services::data_structs::midi_payload::MidiPayload, MidiState,
};
use arduino_comm::midi_connection::{connect, list_available_devices};
use paris::{info, warn, Logger};
use tauri::{State, Window};
use waitgroup::WaitGroup;

use super::service_error::{ServiceError, ServiceResult};

const ALREADY_CONNECTED_CODE: &str = "0001";
const ALREADY_CONNECTED: &str = "Already listening to a midi device! Disconnect from it first";

#[tauri::command]
pub fn disconnect_midi(state: State<MidiState>) -> bool {
    state.release_worker();
    true
}

#[tauri::command]
pub async fn list_midi_devices() -> ServiceResult<Vec<String>> {
    let mut logger = Logger::new();
    logger.loading("Looking for available midi devices...");
    match list_available_devices() {
        Ok(v) => {
            let msg = format!("Found devices: {:?}", v);
            logger.done().info(msg);
            Ok(v)
        }
        Err(err) => {
            let msg = format!("Error while listing available midi devices: {}", err);
            logger.done().error(msg);
            Err(ServiceError::from(err))
        }
    }
}

#[tauri::command]
pub async fn start_listening_midi(
    window: Window,
    state: State<'_, MidiState>,
) -> ServiceResult<()> {
    if state.is_working() {
        warn!("There is already a device connected");
        return Err(ServiceError {
            code: ALREADY_CONNECTED_CODE.to_string(),
            message: ALREADY_CONNECTED.to_string(),
        });
    }
    let mut logger = Logger::new();
    logger.info("Starting connection to device and listening for inputs...");
    let wg = WaitGroup::new();
    state.set_worker(&wg);
    let conn = connect(move |wrapper| {
        info!(
            "Received input: {} - {} - {:?}",
            wrapper.state, wrapper.air_strength, wrapper.note
        );
        let payload = MidiPayload::from_midi_wrapper(wrapper);
        window
            .emit(MIDI_NOTE, payload)
            .expect("Could not send midi event!");
    });
    if let Err(err) = conn {
        let msg = format!("Error while connecting to midi device: {}", err);
        logger.error(msg);
        logger.warn("Worker released");
        state.release_worker();
    };
    wg.wait().await;
    Ok(())
}
