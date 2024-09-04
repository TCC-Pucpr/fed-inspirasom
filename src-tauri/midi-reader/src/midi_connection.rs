use crate::errors::{MidiReaderError, MidiReaderResult};
use anyhow::anyhow;
use nodi::midir::{MidiOutput, MidiOutputConnection};
#[cfg(feature = "verbose")]
use paris::info;

#[allow(dead_code)]
pub fn midi_connection() -> MidiReaderResult<MidiOutputConnection> {
    let midi_out = MidiOutput::new("midi_out")
        .map_err(move |e| MidiReaderError::MidiOutputError(anyhow!(e)))?;
    let port = midi_out.ports();
    let p = match port.len() {
        0 => {
            #[cfg(feature = "verbose")]
            {
                info!("No ports were found")
            }
            return Err(MidiReaderError::NoPortsFound);
        }
        1 => port.first().unwrap(),
        _ => {
            #[cfg(feature = "verbose")]
            {
                info!("Many ports were found, selecting the first one...")
            }
            port.first().unwrap()
        }
    };
    let midi_conn = midi_out
        .connect(p, "out_conn")
        .map_err(move |e| MidiReaderError::MidiOutputError(anyhow!(e.to_string())))?;
    Ok(midi_conn)
}
