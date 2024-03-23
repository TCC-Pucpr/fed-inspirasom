use std::io::{Error, ErrorKind, Read, Write};
use std::time::Duration;

use serialport::{SerialPort};
use regex::{Regex};

use crate::serial_communicator::SerialCommunicator;

const  BEGIN_CHAR: char = '{';
const  END_CHAR: char = '}';
const  DEFAULT_TIMEOUT: u64 = 1;
const DEFAULT_BAUD: usize = 115_200;
const DEFAULT_MESSAGE_CAPACITY: usize = 32;
const FETCH_MESSAGE_REGEX: &str = r"\{(.*?)\}";

pub struct ArduinoCommunicator {
    message_begin_char: char,
    message_end_char: char,
    port: Box<dyn SerialPort>,
    message_capacity: usize
}

impl ArduinoCommunicator {
    pub fn new(timeout: usize, baud: usize, port_name: String, message_capacity: usize) -> Self {
        let port =
            serialport::new(port_name, baud as u32)
                .timeout(Duration::from_millis(timeout as u64))
                .open()
                .expect("Failed to open serial port");
        ArduinoCommunicator {
            message_begin_char: BEGIN_CHAR,
            message_end_char: END_CHAR,
            port,
            message_capacity
        }
    }
}

impl Default for ArduinoCommunicator {
    fn default() -> Self {
        let baud = DEFAULT_BAUD;
        let port =
            serialport::new("COM6", baud as u32)
                .timeout(Duration::from_millis(DEFAULT_TIMEOUT))
                .open()
                .expect("Failed to open serial port");
        ArduinoCommunicator {
            message_begin_char: BEGIN_CHAR,
            message_end_char: END_CHAR,
            port,
            message_capacity: DEFAULT_MESSAGE_CAPACITY
        }
    }
}

impl SerialCommunicator for ArduinoCommunicator {
    fn receive_message(&mut self) -> Result<String, Error> {
        let mut serial_buf: Vec<u8> = vec![0; self.message_capacity];
        loop {
            let res = self.port.read(&mut serial_buf);
            match res {
                Ok(size) => {
                    if size > 0 {
                        let string = String::from_utf8_lossy(&serial_buf);
                        let regex = Regex::new(FETCH_MESSAGE_REGEX).expect("invalid regex");
                        let cap = regex.captures(&string).expect("Invalid");
                        return if let Some(s) = cap.get(1) {
                            Ok(String::from(s.as_str()))
                        } else {
                            Err(Error::new(ErrorKind::InvalidInput, "Message incorrectly sent!"))
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    fn send_message(&mut self, message: &str) -> bool {
        let message = format!("{}{}{}", self.message_begin_char, message, self.message_end_char);
        let msg = message.as_bytes();
        self.port.write(msg).is_ok()
    }

    fn generate_message<'a>(&self, msg: &'a str) -> &'a[u8] {
        return msg.as_bytes();
    }
}