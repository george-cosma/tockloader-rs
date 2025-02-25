use probe_rs::probe::DebugProbeInfo;

use crate::connection::{ProbeTargetInfo, SerialTargetInfo};

pub trait KnownBoard {
    fn serial_target_info() -> SerialTargetInfo;
    fn probe_target_info() -> ProbeTargetInfo;
}

pub struct NucleoF4;

impl KnownBoard for NucleoF4 {
    fn serial_target_info() -> SerialTargetInfo {
        SerialTargetInfo::default()
    }

    fn probe_target_info() -> ProbeTargetInfo {
        ProbeTargetInfo {
            chip: "STM32F429ZIT".to_string(),
            core: 0,
            start_address: 0x08040000,
        }
    }
}

pub struct MicrobitV2;

impl KnownBoard for MicrobitV2 {
    fn serial_target_info() -> SerialTargetInfo {
        SerialTargetInfo::default()
    }

    fn probe_target_info() -> ProbeTargetInfo {
        ProbeTargetInfo {
            chip: "nRF52833".to_string(),
            core: 0,
            start_address: 0x00040000,
        }
    }
}
