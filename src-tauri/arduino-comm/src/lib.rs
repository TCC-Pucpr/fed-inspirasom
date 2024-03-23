mod serial_communicator;
mod arduino_communicator;

#[cfg(test)]
mod arduino_tests {
    use crate::arduino_communicator::ArduinoCommunicator;
    use crate::serial_communicator::{print_ports, SerialCommunicator};
    
    #[ignore]
    #[test]
    fn print_available_ports() {
        print_ports();
    }
    
    #[ignore]
    #[test]
    fn connect_board_test() {
        ArduinoCommunicator::default();
    }
    
    #[test]
    fn send_and_receive() {
        let test_message = "Teste";
        let mut comm = ArduinoCommunicator::default();
        comm.send_message(test_message);
        let response = comm.receive_message().expect("Message receive error!");
        assert_eq!(response, test_message);
        let second_test_message = "pao de batata";
        comm.send_message(second_test_message);
        let second_response = comm.receive_message().expect("Second message error");
        assert_eq!(second_response, second_test_message);
    }
}
