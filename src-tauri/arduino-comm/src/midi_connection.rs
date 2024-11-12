use crate::errors::{ArduinoCommResult, ArduinoCommunicationError};
use crate::midi_wrapper::MidiWrapper;
#[cfg(feature = "verbose")]
use crate::LOG_TAG;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
#[cfg(feature = "verbose")]
use paris::info;

const PORT_NAME: &str = "USB MidiKliK";
const CLIENT_NAME: &str = "InspiraSomMidiIn";
const INSPIRE_PORT_NAME: &str = "InspireMidiPort";

pub struct MidiConnection {
    pub port_name: String,
    pub(super) port: MidiInputPort,
    conn: MidiInput
}

pub struct ConnectionHolder {
    _con: MidiInputConnection<()>,
}

impl MidiConnection {
    pub fn start_connection<C: Fn(MidiWrapper) + Send + 'static>(
        self,
        callback: C
    ) -> ArduinoCommResult<ConnectionHolder> {
        let con = self.conn.connect(
            &self.port,
            INSPIRE_PORT_NAME,
            move |_, x, _| {
                if x.len() < 3 { return }
                let wrapper = MidiWrapper::new_from_bytes(
                    x[0],
                    x[1],
                    x[2]
                );
                callback(wrapper);
            },
            ()
        ).map_err(move |e| {
            ArduinoCommunicationError::PortListenError(e.to_string())
        })?;
        Ok(ConnectionHolder { _con: con })
    }
}

pub fn list_available_devices() -> ArduinoCommResult<Vec<String>> {
    let midi_in = MidiInput::new(CLIENT_NAME)?;
    let a = midi_in.ports().into_iter().map(move |x| {
        let name = midi_in.port_name(&x).unwrap();
        name
    }).collect();
    Ok(a)
}

pub fn connect_to_port_with_name(name: &str) -> ArduinoCommResult<MidiConnection> {
    let conn = MidiInput::new(CLIENT_NAME)?;
    for d in conn.ports() {
        let n = conn.port_name(&d).unwrap();
        #[cfg(feature = "verbose")]
        {
            info!("[{}] Found port {}", LOG_TAG, n);
        }
        if n.contains(name) {
            #[cfg(feature = "verbose")]
            {
                info!("[{}] Selected port {}", LOG_TAG, n);
            }
            return Ok(MidiConnection {
                port_name: n,
                port: d,
                conn,
            })
        }
    }
    Err(ArduinoCommunicationError::PortWithNameNotFound(name.to_string()))
}

pub fn connect_to_port() -> ArduinoCommResult<MidiConnection> {
    connect_to_port_with_name(PORT_NAME)
}