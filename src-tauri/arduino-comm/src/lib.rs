pub mod midi_connection;
pub mod midi_notes;

#[cfg(test)]
mod tests {
    use std::io::stdin;
    use midir::{Ignore, MidiInput};

    use crate::midi_connection::{connect};
    
    #[ignore]
    #[test]
    fn check_if_listens() {
        connect(move |note| {
            println!("{} - {:?}", note.velocity, note.note)
        }).expect("TODO: panic message");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("TODO: panic message");
    }
    
    #[ignore]
    #[test]
    fn test_ports() {
        let mut input = MidiInput::new("Midi").expect("Erro entrada midi");
        input.ignore(Ignore::None);
        
        println!("Portas disponiveis:");
        for p in input.ports().iter() {
            println!("{}", input.port_name(p).expect("Erro porta midi"));
        }
    }
}
