use log::{info, debug, error};
use std::{time::Duration, collections::VecDeque};
use sim7000::{AtModem, SerialReadTimeout, Serial, SerialWrite};
use sim7000::tcp_client::TcpClient;
use serialport::SerialPort;

enum SerialOperation {
    Read(Vec<u8>),
    Write(Vec<u8>),
}

struct SIM7000Modem {
    port: Box<dyn SerialPort>,
    operations: VecDeque<SerialOperation>,
}

impl SIM7000Modem {
    pub fn new(baudrate: u32, port_path: &str) -> Self {
        let port = Self::connect(baudrate, port_path.to_string());
        SIM7000Modem { port, operations: VecDeque::new() }
    }

    fn connect(baudrate: u32, port_path: String) -> Box<dyn SerialPort> {
        let port = serialport::new(port_path.to_string(), baudrate)
            .timeout(Duration::from_millis(10))
            .open();

        match port {
            Ok(port) => {
                info!("SHIELD: Connected to {}", port_path);
                port
            }
            Err(e) => {
                panic!("SHIELD: Failed to connect to {}: {}", port_path, e);
            }
        }
    }
}

impl Serial for SIM7000Modem {
    type SerialError = std::io::Error;
}

impl SerialWrite for SIM7000Modem {
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::SerialError> {
        loop {
            match self.operations.front_mut() {
                Some(SerialOperation::Read(bytes)) => panic!(
                    "Expected Read of {:?}, write called instead with {:?}",
                    bytes.as_slice(),
                    buf
                ),
                Some(SerialOperation::Write(bytes)) => {
                    let result = self.port.write(bytes);
                    match result {
                        Ok(_) => {
                            self.operations.pop_front();
                            ()
                        }
                        Err(e) => {
                            panic!("{:?}", e)
                        }
                    }
                }
                None => panic!("Expected no more operations, write called instead"),
            }
        }
    }
}

impl SerialReadTimeout for SIM7000Modem {
    fn read_exact(
        &mut self,
        buf: &mut [u8],
        timeout_ms: u32,
    ) -> Result<Option<()>, Self::SerialError> {
        todo!()
    }

    fn read(&mut self, buf: &mut [u8], timeout_ms: u32)
        -> Result<Option<usize>, Self::SerialError> {
        todo!()
    }
}

impl AtModem for SIM7000Modem {
    fn read<C: sim7000::commands::AtRead>(
        &mut self,
        command: C,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        todo!()
    }

    fn write<'a, C: sim7000::commands::AtWrite<'a>>(
        &mut self,
        command: C,
        param: C::Input,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        todo!()
    }

    fn execute<C: sim7000::commands::AtExecute>(
        &mut self,
        command: C,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        todo!()
    }
}

fn main() {
    let mut modem = SIM7000Modem::new(115200, "/dev/ttyUSB0");
    let mut tcp_client = TcpClient::default();
    tcp_client.connect(&mut modem, "google.com", 80, None);
}