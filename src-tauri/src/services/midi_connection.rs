use crate::{
    constants::events_name::MIDI_NOTE, services::data_structs::midi_signal::MidiPayload, MidiState,
};
use arduino_comm::midi_connection::{connect, list_available_devices};
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
pub fn list_midi_devices() -> ServiceResult<Vec<String>> {
    list_available_devices().map_err(move |err| ServiceError::new_with_message(err))
}

#[tauri::command]
pub async fn start_listening_midi(
    window: Window,
    state: State<'_, MidiState>,
) -> ServiceResult<()> {
    if state.is_working() {
        return Err(ServiceError {
            code: ALREADY_CONNECTED_CODE.to_string(),
            message: ALREADY_CONNECTED.to_string(),
        });
    }
    let wg = WaitGroup::new();
    state.set_worker(&wg);
    let _conn = connect(move |wrapper| {
        println!(
            "Received input: {} - {} - {:?}",
            wrapper.state, wrapper.air_strength, wrapper.note
        );
        let payload = MidiPayload::from_midi_wrapper(wrapper);
        window
            .emit(MIDI_NOTE, payload)
            .expect("Could not send midi event!");
    });
    wg.wait().await;
    Ok(())
}
