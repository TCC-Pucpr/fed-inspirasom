use crate::app_states::midi_device_state::MidiState;
use crate::app_states::monitoring_state::MonitoringState;
use crate::commands::payloads::midi_payload::MidiPayload;
use crate::commands::ServiceResult;
use crate::constants::events_name::{MIDI_DEVICE_CONNECTION_STATE, MIDI_NOTE};
use crate::constants::limits::MIDI_DEVICE_CONNECTION_CHECKER_TIMEOUT;
use arduino_comm::midi_connection::{list_available_devices, MidiConnection};
use arduino_comm::note::Note;
use paris::{info, warn};
use std::time::Duration;
use log::error;
use tauri::{AppHandle, Manager, State, Window};
use crate::app_states::midi_output_state::MidiOutputState;

pub fn connect(
    window: &Window,
    app_handle: AppHandle,
    midi_state: State<'_, MidiState>,
    midi_output_state: State<'_, MidiOutputState>,
    conn: MidiConnection
) -> ServiceResult<()> {
    info!("Connecting to device {}", conn.port_name);
    midi_state.set_device_connection(conn)?;
    info!("Successfully connected, listening for inputs...");
    let window_label = window.label().to_owned();
    device_connection_listener(
        midi_state.connected_port_name()?,
        window_label.clone(),
        app_handle.clone()
    );
    window.emit(MIDI_DEVICE_CONNECTION_STATE, true)?;
    if let Err(e) = midi_output_state.create_output_connection() {
        error!("Could not connect to output port: {}", e);
    }
    midi_state.start_listening_to_device(move |wrapper| {
        let monitoring_state = app_handle.state::<MonitoringState>();
        let output_state = app_handle.state::<MidiOutputState>();
        let input_msg = format!(
            "{} - {} - {:?}",
            wrapper.state, wrapper.air_strength, wrapper.note
        );
        info!("Received input: {}", input_msg);
        let is_on = wrapper.state == Note::STATE_ON;
        if let Err(e) = output_state.send_output_note(
            wrapper.note, 
            is_on, 
            wrapper.air_strength
        ) {
            error!("Could not send output note: {}", e);
        } else { 
            info!("Successfully sent output: {}", wrapper.state);
        }
        if let Err(_) = monitoring_state.receive_breath_data(wrapper.air_strength, is_on) {
            warn!("Error while monitoring breath data {}", input_msg);
        };
        let payload = MidiPayload::from_midi_wrapper(wrapper);
        let _ = app_handle.emit_to(&window_label, MIDI_NOTE, payload);
    }).map_err(|e| {
        let _ = window.emit(MIDI_DEVICE_CONNECTION_STATE, false);
        if let Err(e) = midi_output_state.drop_output_connection() {
            error!("Could not drop output connection: {}", e);
        }
        e
    })?;
    Ok(())
}

pub fn device_connection_listener(
    connected_port_name: String,
    window_label: String,
    app_handle: AppHandle,
) {
    info!("Starting device health checker");
    tokio::spawn(async move { 
        let midi_state = app_handle.state::<MidiState>();
        let duration = Duration::from_millis(MIDI_DEVICE_CONNECTION_CHECKER_TIMEOUT);
        let output_state = app_handle.state::<MidiOutputState>();
        loop {
            tokio::time::sleep(duration).await;
            if !midi_state.has_connection() {
                info!("Disconnecting device health checker");
                break
            }
            let port_available = if let Ok(devices) = list_available_devices() {
                devices.into_iter().any(|d| d.contains(&connected_port_name))
            } else { 
                false
            };
            if !port_available {
                let _ = app_handle.emit_to(
                    &window_label, 
                    MIDI_DEVICE_CONNECTION_STATE, 
                    false
                );
                midi_state.drop_device_connection();
                if let Err(e) = output_state.drop_output_connection() {
                    error!("Failed to drop output: {}", e);
                }
                warn!("Midi device lost connection");
                break
            } else { 
                info!("Device is still connected")
            }
        }
    });
}