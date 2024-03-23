pub trait SerialCommunicator {
    fn receive_message(&mut self) -> Result<String, std::io::Error>;
    fn send_message(&mut self, message: &str) -> bool;
    fn generate_message<'a>(&self, msg: &'a str) -> &'a[u8] {
        return msg.as_bytes();
    }
}

pub fn print_ports() {
    let ports = serialport::available_ports().expect("Unable to read ports");
    println!("{:?}", ports);
}