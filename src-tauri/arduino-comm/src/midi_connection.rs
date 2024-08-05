use midir::{ConnectError, MidiInput, MidiInputConnection, MidiInputPort};

use crate::midi_wrapper::MidiWrapper;

const PORT_NAME: &str = "USB MidiKliK";

pub fn connect<F: Fn(MidiWrapper) + Send + 'static>(callback: F) -> Result<MidiInputConnection<()>, ConnectError<MidiInput>> {
    let midi_in = MidiInput::new("MidiConnection").expect("Error");
    let ports = midi_in.ports();
    let input_port = match ports.len() {
        0 => {
            panic!("Nao possui portas!")
        }
        1 => {
            let port = ports[0].to_owned();
            let port_name = midi_in.port_name(&port).unwrap();
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
                let port_name = midi_in.port_name(&p).unwrap();
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
        let port_name = midi_in.port_name(&p).unwrap();
        println!("Porta selecionada: {}", port_name);
        midi_in.connect(
            &p,
            "Inspire", 
            move |_, x: &[u8], _| {
                println!("{:?} recebido", x);
                if x.len() > 2 {
                    callback(MidiWrapper::new_from_bytes(x[0], x[1], x[2]))
                }
            },
            ()
        )
    } else {
        panic!("Porta não encontrada!");
    }
}