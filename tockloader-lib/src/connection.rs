use std::time::Duration;

use probe_rs::{probe::DebugProbeInfo, Permissions, Session};
use tokio_serial::{FlowControl, Parity, SerialPort, SerialStream, StopBits};

use crate::errors::TockloaderError;

pub enum ConnectionInfo {
    SerialInfo(String, SerialTargetInfo),
    ProbeInfo(DebugProbeInfo, ProbeTargetInfo),
}

pub struct ProbeTargetInfo {
    pub chip: String,
    pub core: usize,
    pub start_address: u32,
}

impl ProbeTargetInfo {
    fn default(chip: String) -> Self {
        Self {
            chip,
            core: 0,
            start_address: 0x30000,
        }
    }
}

pub struct SerialTargetInfo {
    pub baud_rate: u32,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
    pub timeout: Duration,
    pub request_to_send: bool,
    pub data_terminal_ready: bool,
}

impl Default for SerialTargetInfo {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None,
            timeout: Duration::from_millis(500),
            request_to_send: false,
            data_terminal_ready: false,
        }
    }
}

pub enum Connection {
    ProbeRS(Session),
    Serial(SerialStream),
}

impl Connection {
    pub fn open(info: ConnectionInfo) -> Result<Connection, TockloaderError> {
        match info {
            ConnectionInfo::SerialInfo(path, target_info) => {
                let builder = tokio_serial::new(path, target_info.baud_rate);
                match SerialStream::open(&builder) {
                    Ok(mut port) => {
                        port.set_parity(target_info.parity)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_stop_bits(target_info.stop_bits)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_flow_control(target_info.flow_control)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.set_timeout(target_info.timeout)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.write_request_to_send(target_info.request_to_send)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        port.write_data_terminal_ready(target_info.data_terminal_ready)
                            .map_err(TockloaderError::SerialInitializationError)?;
                        Ok(Connection::Serial(port))
                    }
                    Err(e) => Err(TockloaderError::SerialInitializationError(e)),
                }
            }
            ConnectionInfo::ProbeInfo(probe_info, target_info) => {
                let probe = probe_info
                    .open()
                    .map_err(TockloaderError::ProbeRsInitializationError)?;

                match probe.attach(target_info.chip, Permissions::default()) {
                    Ok(session) => Ok(Connection::ProbeRS(session)),
                    Err(e) => Err(TockloaderError::ProbeRsCommunicationError(e)),
                }
            }
        }
    }
}
