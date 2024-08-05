pub mod midi_connection;
pub mod midi_wrapper;
mod note;

#[cfg(test)]
mod tests {
    use std::io::stdin;
    use midir::{Ignore, MidiInput};

    use crate::midi_connection::{connect};
    
    #[ignore]
    #[test]
    fn check_if_listens() {
        connect(move |note| {
            println!("{} - {:?}", note.air_strength, note.note)
        }).expect("Could not connect to midi device!");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Could not read line");
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
