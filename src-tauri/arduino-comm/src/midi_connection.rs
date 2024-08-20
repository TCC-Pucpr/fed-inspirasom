use std::error::Error;

use midir::{ConnectError, MidiInput, MidiInputConnection, MidiInputPort};

use crate::midi_wrapper::MidiWrapper;

const PORT_NAME: &str = "USB MidiKliK";
const CLIENT_NAME: &str = "InspiraSomMidiIn";
const INSPIRE_PORT_NAME: &str = "InspireMidiPort";

pub fn list_available_devices() -> Result<Vec<String>, String> {
    let midi_in =
        MidiInput::new(CLIENT_NAME).map_err(|_| String::from("Unable to list devices"))?;
    let ports = midi_in.ports();
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
) -> Result<MidiInputConnection<()>, ConnectError<String>> {
    let midi_in = MidiInput::new(CLIENT_NAME).map_err(|_| {
        ConnectError::new(
            midir::ConnectErrorKind::Other("Could not connect to port"),
            name.to_string(),
        )
    })?;
    let ports = midi_in.ports();
    for p in ports {
        if let Ok(n) = midi_in.port_name(&p) {
            if n == name {
                return Ok(start_listening_port(&p, callback).expect("AAA"));
            }
        }
    }
    Err(ConnectError::new(
        midir::ConnectErrorKind::InvalidPort,
        name.to_string(),
    ))
}

pub fn connect<F: Fn(MidiWrapper) + Send + 'static>(
    callback: F,
) -> Result<MidiInputConnection<()>, ConnectError<MidiInput>> {
    let midi_in = MidiInput::new(CLIENT_NAME).expect("AAA");
    let ports = midi_in.ports();
    let input_port = match ports.len() {
        0 => {
            panic!("Nao possui portas!")
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
        let port_name = midi_in.port_name(&p).expect("AAA");
        println!("Porta selecionada: {}", port_name);
        Ok(start_listening_port(&p, callback).expect("AAA"))
    } else {
        Err(ConnectError::new(
            midir::ConnectErrorKind::InvalidPort,
            midi_in,
        ))
    }
}

#[inline]
fn start_listening_port<F: Fn(MidiWrapper) + Send + 'static>(
    port: &MidiInputPort,
    callback: F,
) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let midi_in = MidiInput::new(CLIENT_NAME)?;
    midi_in
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
        .map_err(|_| Box::from("Could not connect to midi port"))
}
