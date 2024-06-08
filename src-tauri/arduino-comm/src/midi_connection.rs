use midir::{ConnectError, MidiInput, MidiInputConnection};

use crate::midi_notes::NoteWrapper;

pub fn connect<F: Fn(NoteWrapper) + Send + 'static>(callback: F) -> Result<MidiInputConnection<()>, ConnectError<MidiInput>> {
    let midi_in = MidiInput::new("MidiConnection").expect("Error");
    let ports = midi_in.ports();
    let input_port = match ports.len() {
        0 => {
            panic!("Nao possui portas!")
        }
        1 => {
            let name = midi_in.port_name(&ports[0]).unwrap();
            println!("Uma unica porta encontrada: {}", name);
            Some(ports[0].to_owned())
        }
        _ => {
            println!("Varias portas encontradas!");
            for p in ports.iter() {
                println!("Porta: {}", midi_in.port_name(p).unwrap());
            }
            Some(ports[0].to_owned())
        }
    };
    midi_in.connect(
        &input_port.unwrap(),
        "Inspire", 
        move |_, x: &[u8], _| {
            println!("{:?} recebido", x);
            if x.len() > 2 {
                callback(NoteWrapper::new_from_bytes(x[1], x[2]))
            }
        },
        ()
    )
}