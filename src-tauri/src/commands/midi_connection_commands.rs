use crate::commands::commands_utils::midi_connection_utils::connect;
use crate::commands::payloads::service_error::{ServiceError, ServiceResult};
use crate::MidiState;
use arduino_comm::midi_connection::{connect_to_port, connect_to_port_with_name, list_available_devices};
use paris::{error, info, success};
use tauri::{AppHandle, State, Window};

#[tauri::command]
pub fn disconnect_midi(state: State<MidiState>) -> bool {
    state.drop_device_connection();
    true
}

#[tauri::command]
pub async fn list_midi_devices() -> ServiceResult<Vec<String>> {
    info!("Looking for available midi devices...");
    match list_available_devices() {
        Ok(v) => {
            success!("Found devices: {:?}", v);
            Ok(v)
        }
        Err(err) => {
            error!("Error while listing available midi devices: {}", err);
            Err(ServiceError::from(err))
        }
    }
}

#[tauri::command]
pub async fn connect_to_midi(
    port_name: &str,
    window: Window,
    state: State<'_, MidiState>,
    app_handle: AppHandle,
) -> ServiceResult<()> {
    let conn = connect_to_port_with_name(port_name)?;
    connect(&window, app_handle, state, conn)
}

#[tauri::command]
pub async fn start_listening_midi(
    window: Window,
    state: State<'_, MidiState>,
    app_handle: AppHandle,
) -> ServiceResult<()> {
    let conn = connect_to_port()?;
    connect(&window, app_handle, state, conn)
}
