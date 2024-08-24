use crate::Result;
use nodi::midir::{MidiOutput, MidiOutputConnection};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
struct NoPortsError;

impl Display for NoPortsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No ports available")
    }
}

impl Error for NoPortsError {}

#[allow(dead_code)]
pub fn midi_connection() -> Result<MidiOutputConnection> {
    let midi_out = MidiOutput::new("midi_out").unwrap();
    let port = midi_out.ports();
    let p = match port.len() {
        0 => {
            println!("Nenhuma porta");
            return Err(NoPortsError.into());
        }
        1 => port.first().unwrap(),
        _ => {
            println!("Varias portas");
            port.first().unwrap()
        }
    };
    let midi_conn = midi_out.connect(p, "out_conn")?;
    Ok(midi_conn)
}
