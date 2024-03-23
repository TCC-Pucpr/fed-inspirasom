use std::io::{Error, ErrorKind};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub trait SerialCommunicator {
    /// Imediatamente retorna a proxima mensagem que foi enviada na porta,
    /// ou [Err] caso não tiver uma mensagem, ou se ocorrer algum outro erro
    fn receive_message(&mut self) -> Result<String, Error>;
    fn send_message(&mut self, message: &str) -> bool;
    fn generate_message<'a>(&self, msg: &'a str) -> &'a[u8] {
        msg.as_bytes()
    }
    /// Bloqueia a thread atual até que uma nova mensagem seja recebida.
    /// Para não arriscar ficar travado para sempre, passe um tempo maximo para
    /// aguardar.
    /// A cada 100 milissegundos, uma nova tentativa para receber a mensagem é feita
    fn await_new_message(&mut self, duration: Duration) -> Result<String, Error> {
        let start_time = SystemTime::now();
        let retry_delay = Duration::from_millis(10);
        loop {
            let res = self.receive_message();
            match res {
                Ok(s) => {
                    return Ok(s)
                }
                Err(e) => {
                    if let Ok(s) = start_time.elapsed() {
                        if s > duration {
                            return Err(e)
                        }
                    }
                }
            }
            sleep(retry_delay);
        }
    }
}