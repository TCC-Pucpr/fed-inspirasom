use crate::{
    constants::events_name::MIDI_NOTE, services::data_structs::midi_payload::MidiPayload, MidiState,
};
use arduino_comm::{
    errors::{Errors, MidiDeviceConnectionError},
    midi_connection::{connect, connect_to_port_with_name, list_available_devices},
    midi_wrapper::MidiWrapper,
};
use tauri::{State, Window};
use waitgroup::WaitGroup;

use super::service_error::{ServiceError, ServiceResult};

const ALREADY_CONNECTED_CODE: &str = "0001";
const ALREADY_CONNECTED: &str = "Already listening to a midi device! Disconnect from it first";
const NOT_FOUND_CODE: &str = "0002";
const NOT_FOUND_MESSAGE: &str = "Port with name not found";
const NO_PORTS_FOUND_CODE: &str = "0003";
const NO_PORTS_FOUND_MESSAGE: &str = "No midi devices connected";

#[tauri::command]
pub fn disconnect_midi(state: State<MidiState>) -> bool {
    if !state.is_working() {
        false
    } else {
        state.release_worker();
        true
    }
}

#[tauri::command]
pub fn list_midi_devices() -> ServiceResult<Vec<String>> {
    list_available_devices().map_err(map_error)
}

#[tauri::command]
pub async fn start_listening_midi(
    port_name: String,
    window: Window,
    state: State<'_, MidiState>,
) -> ServiceResult<()> {
    check_if_connected(&state)?;
    let wg = WaitGroup::new();
    state.set_worker(&wg);
    let _conn = connect_to_port_with_name(&port_name, move |wrapper| {
        let _ = midi_callback(wrapper, &window);
    })
    .map_err(map_error)?;
    wg.wait().await;
    Ok(())
}

#[tauri::command]
pub async fn quick_midi_connect(window: Window, state: State<'_, MidiState>) -> ServiceResult<()> {
    check_if_connected(&state)?;
    let wg = WaitGroup::new();
    state.set_worker(&wg);
    let _con = connect(move |wrapper| {
        if let Err(e) = midi_callback(wrapper, &window) {
            println!("Error received: {}", e);
        }
    })
    .map_err(map_error)?;
    wg.wait().await;
    Ok(())
}

fn check_if_connected(state: &State<'_, MidiState>) -> ServiceResult<()> {
    if state.is_working() {
        Err(map_error(MidiDeviceConnectionError::from(
            Errors::AlreadyConnected,
        )))
    } else {
        Ok(())
    }
}

fn midi_callback(wrapper: MidiWrapper, window: &Window) -> ServiceResult<()> {
    println!(
        "Received input: {} - {} - {:?}",
        wrapper.state, wrapper.air_strength, wrapper.note
    );
    let payload = MidiPayload::from_midi_wrapper(wrapper);
    window
        .emit(MIDI_NOTE, payload)
        .map_err(|err| ServiceError::new_with_message(err.to_string()))
}

fn map_error(error: MidiDeviceConnectionError) -> ServiceError {
    match error.error {
        arduino_comm::errors::Errors::NoPortsFound => ServiceError {
            code: NO_PORTS_FOUND_CODE.to_string(),
            message: NO_PORTS_FOUND_MESSAGE.to_string(),
        },
        arduino_comm::errors::Errors::PortNotFound => ServiceError {
            code: NOT_FOUND_CODE.to_string(),
            message: NOT_FOUND_MESSAGE.to_string(),
        },
        arduino_comm::errors::Errors::AlreadyConnected => ServiceError {
            code: ALREADY_CONNECTED_CODE.to_string(),
            message: ALREADY_CONNECTED.to_string(),
        },
        arduino_comm::errors::Errors::Unknown => ServiceError::generic(),
    }
}
