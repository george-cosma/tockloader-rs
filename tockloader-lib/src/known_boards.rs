use crate::board_settings::BoardSettings;
use crate::connection::{ProbeTargetInfo, SerialTargetInfo};

pub trait KnownBoard {
    fn serial_target_info(&self) -> SerialTargetInfo;
    fn probe_target_info(&self) -> ProbeTargetInfo;
    fn get_settings(&self) -> BoardSettings;
}

pub struct NucleoF4;

impl KnownBoard for NucleoF4 {
    fn serial_target_info(&self) -> SerialTargetInfo {
        SerialTargetInfo::default()
    }

    fn probe_target_info(&self) -> ProbeTargetInfo {
        ProbeTargetInfo {
            chip: "STM32F429ZIT".to_string(),
            core: 0,
        }
    }

    fn get_settings(&self) -> BoardSettings {
        BoardSettings {
            arch: Some("cortex-m4".to_string()),
            start_address: 0x08040000,
        }
    }
}

pub struct MicrobitV2;

impl KnownBoard for MicrobitV2 {
    fn serial_target_info(&self) -> SerialTargetInfo {
        SerialTargetInfo::default()
    }

    fn probe_target_info(&self) -> ProbeTargetInfo {
        ProbeTargetInfo {
            chip: "nRF52833".to_string(),
            core: 0,
        }
    }

    fn get_settings(&self) -> BoardSettings {
        BoardSettings {
            arch: Some("cortex-m4".to_string()),
            start_address: 0x00040000,
        }
    }
}
