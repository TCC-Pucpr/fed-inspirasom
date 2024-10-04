use crate::commands::payloads::service_error::{ServiceError, ServiceResult};
use crate::{
    commands::payloads::midi_payload::MidiPayload, constants::events_name::MIDI_NOTE, MidiState,
};
use arduino_comm::errors::ArduinoCommResult;
use arduino_comm::midi_connection::{connect, connect_to_port_with_name, list_available_devices, MidiConnection};
use arduino_comm::midi_wrapper::MidiWrapper;
use paris::{info, warn, Logger};
use tauri::{State, Window};
use waitgroup::WaitGroup;

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
pub async fn connect_to_midi(
    port_name: &str,
    window: Window,
    state: State<'_, MidiState>,
) -> ServiceResult<()> {
    let p = move || {
        connect_to_port_with_name(port_name, move |wrapper| {
            note_received(wrapper, &window);
        })
    };
    listen_to_port(state, p).await
}

#[tauri::command]
pub async fn start_listening_midi(
    window: Window,
    state: State<'_, MidiState>,
) -> ServiceResult<()> {
    let p = move || {
        connect(move |wrapper| {
            note_received(wrapper, &window);
        })
    };
    listen_to_port(state, p).await
}

fn note_received(
    wrapper: MidiWrapper,
    window: &Window
) {
    info!(
        "Received input: {} - {} - {:?}",
        wrapper.state, wrapper.air_strength, wrapper.note
    );
    let payload = MidiPayload::from_midi_wrapper(wrapper);
    window
        .emit(MIDI_NOTE, payload)
        .expect("Could not send midi event!");
}

async fn listen_to_port(
    state: State<'_, MidiState>,
    midi_connection: impl FnOnce() -> ArduinoCommResult<MidiConnection>
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
    let conn = midi_connection();
    if let Err(err) = conn {
        let msg = format!("Error while connecting to midi device: {}", err);
        logger.error(msg);
        logger.warn("Worker released");
        state.release_worker();
    };
    wg.wait().await;
    Ok(())
}
