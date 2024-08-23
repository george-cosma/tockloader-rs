// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use probe_rs::probe::DebugProbeInfo;
use probe_rs::{Core, Permissions, Session};
use std::time::Duration;
use tokio_serial::{FlowControl, Parity, SerialPort, SerialPortType, SerialStream, StopBits};

pub struct ProbeSession {
    pub session: Option<Session>,
}

impl ProbeSession {
    pub fn new(probe_info: DebugProbeInfo, chip: &str) -> ProbeSession {
        let serial_nr = probe_info.clone().serial_number.unwrap();
        let mut probe = Some(probe_info.open().unwrap());
        let mut session = None;

        if let Some(probe) = probe.take() {
            session = Some(probe.attach(chip, Permissions::default()).unwrap());
        }
        let ports = tokio_serial::available_ports().unwrap();
        for port in ports {
            if let SerialPortType::UsbPort(inner) = port.port_type {
                if inner.serial_number.unwrap() == serial_nr {
                    // Open port and configure it with tokio_serial
                    let builder = tokio_serial::new(port.port_name, 115200);
                    match SerialStream::open(&builder) {
                        Ok(mut port) => {
                            port.set_parity(Parity::None).unwrap();
                            port.set_stop_bits(StopBits::One).unwrap();
                            port.set_flow_control(FlowControl::None).unwrap();
                            port.set_timeout(Duration::from_millis(500)).unwrap();
                            port.write_request_to_send(false).unwrap();
                            port.write_data_terminal_ready(false).unwrap();
                        }
                        Err(_e) => {
                            //TODO(Micu Ana): Add error handling
                        }
                    }
                }
            }
        }
        let probe_session = session.expect("Couldn't create a session");
        ProbeSession {
            session: Some(probe_session),
        }
    }

    pub fn get_core(&mut self, core_index: usize) -> Core {
        return self.session.as_mut().unwrap().core(core_index).unwrap();
    }
}
