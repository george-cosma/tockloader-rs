use std::time::Duration;

use probe_rs::{probe::DebugProbeInfo, Permissions, Session};
use tokio_serial::{FlowControl, Parity, SerialPort, SerialStream, StopBits};

use crate::errors::TockloaderError;

pub enum ConnectionInfo {
    SerialInfo(String),
    ProbeInfo(DebugProbeInfo),
}

impl From<String> for ConnectionInfo {
    fn from(value: String) -> Self {
        ConnectionInfo::SerialInfo(value)
    }
}

pub enum Connection {
    ProbeRS(Session),
    Serial(SerialStream),
}

impl Connection {
    pub fn open(info: ConnectionInfo, chip: Option<String>) -> Result<Connection, TockloaderError> {
        match info {
            ConnectionInfo::SerialInfo(serial_info) => {
                let builder = tokio_serial::new(serial_info, 115200);
                match SerialStream::open(&builder) {
                    Ok(mut port) => {
                        port.set_parity(Parity::None).unwrap();
                        port.set_stop_bits(StopBits::One).unwrap();
                        port.set_flow_control(FlowControl::None).unwrap();
                        port.set_timeout(Duration::from_millis(500)).unwrap();
                        port.write_request_to_send(false).unwrap();
                        port.write_data_terminal_ready(false).unwrap();
                        Ok(Connection::Serial(port))
                    }
                    Err(_) => {
                        //TODO(Micu Ana): Add error handling
                        Err(TockloaderError::NoPortAvailable)
                    }
                }
            }
            ConnectionInfo::ProbeInfo(probe_info) => {
                //TODO(Micu Ana): Add error handling
                let probe = probe_info.open().unwrap();
                match probe.attach(chip.unwrap(), Permissions::default()) {
                    Ok(session) => Ok(Connection::ProbeRS(session)),
                    Err(_) => {
                        //TODO(Micu Ana): Add error handling
                        Err(TockloaderError::NoPortAvailable)
                    }
                }
            }
        }
    }
}
