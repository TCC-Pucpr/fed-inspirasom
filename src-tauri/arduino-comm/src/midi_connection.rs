use crate::errors::{ArduinoCommResult, ArduinoCommunicationError};
use crate::midi_wrapper::MidiWrapper;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use paris::{info, success};

const PORT_NAME: &str = "USB MidiKliK";
const CLIENT_NAME: &str = "InspiraSomMidiIn";
const INSPIRE_PORT_NAME: &str = "InspireMidiPort";

pub fn list_available_devices() -> ArduinoCommResult<Vec<String>> {
    let midi_in =
        MidiInput::new(CLIENT_NAME).map_err(|_| ArduinoCommunicationError::MidiInputError)?;
    let ports = midi_in.ports();
    if ports.is_empty() {
        return Err(ArduinoCommunicationError::NoDevicesConnected);
    }
    let mut names = Vec::with_capacity(ports.len());
    for p in ports {
        if let Ok(n) = midi_in.port_name(&p) {
            names.push(n);
        }
    }
    Ok(names)
}

pub fn connect_to_port_with_name<F: Fn(MidiWrapper) + Send + 'static>(
    name: &str,
    callback: F,
) -> ArduinoCommResult<()> {
    let midi_in =
        MidiInput::new(CLIENT_NAME).map_err(|_| ArduinoCommunicationError::MidiInputError)?;
    let ports = midi_in.ports();
    for p in ports {
        if let Ok(n) = midi_in.port_name(&p) {
            if n == name {
                #[cfg(feature = "verbose")] 
                {
                    success!("Connected to port with name {name}")    
                }
                start_listening_port(&p, callback)?;
                return Ok(());
            }
        }
    }
    Err(ArduinoCommunicationError::PortWithNameNotFound(
        name.to_string(),
    ))
}

pub fn connect<F: Fn(MidiWrapper) + Send + 'static>(callback: F) -> ArduinoCommResult<()> {
    let midi_in =
        MidiInput::new(CLIENT_NAME).map_err(|_| ArduinoCommunicationError::MidiInputError)?;
    let ports = midi_in.ports();
    let input_port = match ports.len() {
        0 => return Err(ArduinoCommunicationError::NoDevicesConnected),
        1 => {
            let port = ports[0].to_owned();
            let port_name = midi_in
                .port_name(&port)
                .map_err(move |_| ArduinoCommunicationError::PortError)?;
            if port_name.starts_with(PORT_NAME) {
                Some(port)
            } else {
                None
            }
        }
        _ => {
            let mut port: Option<MidiInputPort> = None;
            #[cfg(feature = "verbose")]
            {
                info!("Many ports detected")
            }
            for p in ports {
                let port_name = midi_in
                    .port_name(&p)
                    .map_err(move |_| ArduinoCommunicationError::PortError)?;
                #[cfg(feature = "verbose")]
                {
                    info!("Port {}", port_name);
                }
                if port_name.starts_with(PORT_NAME) {
                    port = Some(p);
                    #[cfg(feature = "verbose")]
                    {
                        info!("Port {} found", PORT_NAME);
                    }
                    break;
                }
            }
            port
        }
    };
    if let Some(p) = input_port {
        let port_name = midi_in
            .port_name(&p)
            .map_err(move |_| ArduinoCommunicationError::PortError)?;
        #[cfg(feature = "verbose")]
        {
            info!("Selected Port: {}", port_name);
        }
        start_listening_port(&p, callback)?;
        Ok(())
    } else {
        Err(ArduinoCommunicationError::OcarinaNotFound)
    }
}

fn start_listening_port<F: Fn(MidiWrapper) + Send + 'static>(
    port: &MidiInputPort,
    callback: F,
) -> ArduinoCommResult<MidiInputConnection<()>> {
    let midi_in =
        MidiInput::new(CLIENT_NAME).map_err(|_| ArduinoCommunicationError::MidiInputError)?;
    let port_name = midi_in
        .port_name(port)
        .map_err(move |_| ArduinoCommunicationError::PortError)?;
    let res = midi_in
        .connect(
            port,
            INSPIRE_PORT_NAME,
            move |_, x: &[u8], _| {
                #[cfg(feature = "verbose")]
                {
                    info!("Payload {:?} received from Ocarina", x);
                }
                if x.len() > 2 {
                    callback(MidiWrapper::new_from_bytes(x[0], x[1], x[2]))
                }
            },
            (),
        )
        .map_err(|_| ArduinoCommunicationError::PortListenError(port_name))?;
    Ok(res)
}
