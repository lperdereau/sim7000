use log::{debug, error, info};
use serialport::SerialPort;
use sim7000::tcp_client::TcpClient;
use sim7000::gnss_client::GnssClient;
use sim7000::{AtModem, Serial, SerialReadTimeout, SerialWrite};
use sim7000::commands::{At, AtCommand};
use simplelog::*;
use std::{collections::VecDeque, time::Duration};

#[derive(Debug)]
enum SerialOperation {
    Read(Vec<u8>),
    Write(Vec<u8>),
}

pub struct SIM7000Modem {
    port: Box<dyn SerialPort>,
    operations: VecDeque<SerialOperation>,
}

impl SIM7000Modem {
    pub fn build(baudrate: u32, port_path: &str) -> SerialBuilder {
        let port = Self::connect(baudrate, port_path.to_string());
        SerialBuilder {
            serial: SIM7000Modem {
                port,
                operations: VecDeque::new(),
            },
        }
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

pub struct SerialBuilder {
    serial: SIM7000Modem,
}

impl SerialBuilder {
    pub fn expect_read(mut self, bytes: &[u8]) -> SerialBuilder {
        self.serial
            .operations
            .push_back(SerialOperation::Read(Vec::from(bytes)));
        self
    }

    pub fn expect_write(mut self, bytes: &[u8]) -> SerialBuilder {
        self.serial
            .operations
            .push_back(SerialOperation::Write(Vec::from(bytes)));
        self
    }

    pub fn finalize(self) -> SIM7000Modem {
        self.serial
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
        if timeout_ms <= 200u32 {
            return Ok(None);
        }
        match self.operations.front_mut() {
            Some(SerialOperation::Read(bytes)) => {
                buf.copy_from_slice(&bytes[..buf.len()]);
                *bytes = Vec::from(&bytes[buf.len()..]);

                if bytes.len() == 0 {
                    self.operations.pop_front();
                }

                Ok(Some(()))
            }
            Some(SerialOperation::Write(bytes)) => panic!(
                "Expected Write of {:?}, read called instead",
                bytes.as_slice()
            ),
            None => Ok(None),
        }
    }

    fn read(
        &mut self,
        buf: &mut [u8],
        timeout_ms: u32,
    ) -> Result<Option<usize>, Self::SerialError> {
        todo!()
    }
}

impl AtModem for SIM7000Modem {
    fn read<C: sim7000::commands::AtRead>(
        &mut self,
        command: C,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        command.read( self, timeout_ms)
    }

    fn write<'a, C: sim7000::commands::AtWrite<'a>>(
        &mut self,
        command: C,
        param: C::Input,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        command.write(param, self, timeout_ms)
    }

    fn execute<C: sim7000::commands::AtExecute>(
        &mut self,
        command: C,
        timeout_ms: u32,
    ) -> Result<C::Output, sim7000::Error<Self::SerialError>> {
        command.execute(self, timeout_ms)
    }
}

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    // let mut serial = SIM7000Modem::build(115200, "/dev/ttyAMA0")
    //     .expect_write(b"AT\r\n")
    //     .expect_read(b"OK\r\n")
    //     .finalize();

    let mut serial = SIM7000Modem::build(115200, "/dev/ttyAMA0")
    .finalize();

    let mut gnss_client = GnssClient::default();
    gnss_client.gnss_start(&mut serial, Some(200u32));

}
