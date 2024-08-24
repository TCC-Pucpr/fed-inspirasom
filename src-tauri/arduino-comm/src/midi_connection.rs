use midir::{MidiInput, MidiInputConnection, MidiInputPort};

use crate::{
    errors::{Errors, MidiConnectionResult, MidiDeviceConnectionError},
    midi_wrapper::MidiWrapper,
};

const PORT_NAME: &str = "USB MidiKliK";
const CLIENT_NAME: &str = "InspiraSomMidiIn";
const INSPIRE_PORT_NAME: &str = "InspireMidiPort";

pub fn list_available_devices() -> MidiConnectionResult<Vec<String>> {
    let midi_in = MidiInput::new(CLIENT_NAME)
        .map_err(|_| MidiDeviceConnectionError::from(Errors::Unknown))?;
    let ports = midi_in.ports();
    if ports.is_empty() {
        return Err(MidiDeviceConnectionError::from(Errors::NoPortsFound));
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
) -> MidiConnectionResult<MidiInputConnection<()>> {
    let midi_in = MidiInput::new(CLIENT_NAME)
        .map_err(|_| MidiDeviceConnectionError::from(Errors::Unknown))?;
    let ports = midi_in.ports();
    if ports.is_empty() {
        return Err(MidiDeviceConnectionError::from(Errors::NoPortsFound));
    }
    for p in ports {
        if let Ok(n) = midi_in.port_name(&p) {
            if n == name {
                return Ok(start_listening_port(&p, callback).expect("AAA"));
            }
        }
    }
    Err(MidiDeviceConnectionError::from(Errors::PortNotFound))
}

pub fn connect<F: Fn(MidiWrapper) + Send + 'static>(
    callback: F,
) -> MidiConnectionResult<MidiInputConnection<()>> {
    let midi_in = MidiInput::new(CLIENT_NAME).expect("AAA");
    let ports = midi_in.ports();
    let input_port = match ports.len() {
        0 => {
            return Err(MidiDeviceConnectionError::from(Errors::NoPortsFound));
        }
        1 => {
            let port = ports[0].to_owned();
            let port_name = midi_in.port_name(&port).expect("AAA");
            if port_name.starts_with(PORT_NAME) {
                Some(port)
            } else {
                None
            }
        }
        _ => {
            let mut port: Option<MidiInputPort> = None;
            println!("Varias portas encontradas!");
            for p in ports {
                let port_name = midi_in.port_name(&p).expect("AAA");
                println!("Porta: {}", port_name);
                if port_name.starts_with(PORT_NAME) {
                    port = Some(p);
                    break;
                }
            }
            port
        }
    };
    if let Some(p) = input_port {
        let port_name = if let Ok(p) = midi_in.port_name(&p) {
            p
        } else {
            return Err(MidiDeviceConnectionError::from(Errors::PortNotFound));
        };
        println!("Porta selecionada: {}", port_name);
        if let Ok(c) = start_listening_port(&p, callback) {
            Ok(c)
        } else {
            Err(MidiDeviceConnectionError::from(Errors::Unknown))
        }
    } else {
        Err(MidiDeviceConnectionError::from(Errors::PortNotFound))
    }
}

fn start_listening_port<F: Fn(MidiWrapper) + Send + 'static>(
    port: &MidiInputPort,
    callback: F,
) -> MidiConnectionResult<MidiInputConnection<()>> {
    let midi_in = MidiInput::new(CLIENT_NAME)
        .map_err(|_| MidiDeviceConnectionError::from(Errors::Unknown))?;
    let con = midi_in
        .connect(
            port,
            INSPIRE_PORT_NAME,
            move |_, x: &[u8], _| {
                println!("{:?} recebido", x);
                if x.len() > 2 {
                    callback(MidiWrapper::new_from_bytes(x[0], x[1], x[2]))
                }
            },
            (),
        )
        .map_err(|_| MidiDeviceConnectionError::from(Errors::PortNotFound))?;
    Ok(con)
}
