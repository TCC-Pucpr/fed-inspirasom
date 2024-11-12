use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
#[cfg(feature = "verbose")]
use paris::info;
use crate::errors::{MidiOutputErrors, MidiOutputResult};
use crate::errors::MidiOutputErrors::{ConnectToPort, CouldNotSendByte, NoAvailablePorts};
use crate::LOG_TAG;

const PORT_NAME: &str = "midi-output";

pub struct MidiOutputConnectionHolder {
    conn: MidiOutputConnection
}

impl MidiOutputConnectionHolder {
    pub fn new_with_first_port() -> MidiOutputResult<MidiOutputConnectionHolder> {
        let output = MidiOutput::new(PORT_NAME)?;
        let ports = output.ports();
        if ports.is_empty() {
            #[cfg(feature = "verbose")]
            {
                info!("{} No output ports available", LOG_TAG)
            }
            return Err(NoAvailablePorts)
        }
        let port = &ports[0];
        let name = output.port_name(&port)
            .map_err(move |_e| MidiOutputErrors::Connecting)?;
        let conn = Self::connect(output, port, &name)?;
        #[cfg(feature = "verbose")]
        {
            info!("{} Connected to output port {}", LOG_TAG, name)
        }
        Ok(MidiOutputConnectionHolder { conn })
    }
    pub fn new_with_port_name(port_name: String) -> MidiOutputResult<MidiOutputConnectionHolder> {
        let output = MidiOutput::new(PORT_NAME)?;
        let ports = output.ports();
        if ports.is_empty() {
            return Err(NoAvailablePorts)
        }
        for p in ports {
            if let Ok(pn) = output.port_name(&p) {
                if pn.contains(&port_name) {
                    let conn = Self::connect(output, &p, &port_name)?;
                    #[cfg(feature = "verbose")]
                    {
                        info!("{} Connected to output port {}", LOG_TAG, port_name)
                    }
                    return Ok(MidiOutputConnectionHolder { conn })
                }
            }
        }
        Err(MidiOutputErrors::PortNotFound(port_name))
    }
    
    fn connect(
        output: MidiOutput,
        port: &MidiOutputPort,
        port_name: &str
    ) -> MidiOutputResult<MidiOutputConnection> {
        let conn = output.connect(
            port,
            port_name
        ).map_err(move |_e| ConnectToPort(port_name.to_string()))?;
        Ok(conn)
    }
    
    pub fn send_note(&mut self, on: bool, byte: u8, velocity: u8) -> MidiOutputResult<()> {
        const NOTE_ON: u8 = 0x90;
        const NOTE_OFF: u8 = 0x80;
        let note_on_off = if on {
            NOTE_ON
        } else { 
            NOTE_OFF
        };
        self.conn.send(&[note_on_off, byte, velocity]).map_err(
            move |_e| CouldNotSendByte(byte)
        )
    }
}