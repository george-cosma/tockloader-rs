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
                        port.set_parity(Parity::None)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_stop_bits(StopBits::One)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_flow_control(FlowControl::None)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_timeout(Duration::from_millis(500))
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.write_request_to_send(false)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.write_data_terminal_ready(false)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        Ok(Connection::Serial(port))
                    }
                    Err(e) => Err(TockloaderError::SerialInitializationError(e)),
                }
            }
            ConnectionInfo::ProbeInfo(probe_info) => {
                let probe = probe_info
                    .open()
                    .map_err(TockloaderError::ProbeRsInitializationError)?;
                match probe.attach(chip.unwrap(), Permissions::default()) {
                    Ok(session) => Ok(Connection::ProbeRS(session)),
                    Err(e) => Err(TockloaderError::ProbeRsCommunicationError(e)),
                }
            }
        }
    }
}
