// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

pub mod attributes;
mod errors;
pub mod probe_session;

use attributes::app_attributes::AppAttributes;
use attributes::general_attributes::GeneralAttributes;
use attributes::system_attributes::SystemAttributes;

use probe_rs::probe::DebugProbeInfo;
use probe_session::ProbeSession;

pub async fn list_probe(
    choice: DebugProbeInfo,
    chip: &str,
    core_index: &usize,
) -> Vec<AppAttributes> {
    let mut probe_session = ProbeSession::new(choice, chip);
    let mut core = probe_session.get_core(*core_index);

    let system_attributes = SystemAttributes::read_system_attributes(&mut core);

    AppAttributes::read_apps_data(&mut core, system_attributes.appaddr.unwrap())
}

pub async fn info_probe(
    choice: DebugProbeInfo,
    chip: &str,
    core_index: &usize,
) -> GeneralAttributes {
    let mut probe_session = ProbeSession::new(choice, chip);

    let mut core = probe_session.get_core(*core_index);

    let system_attributes = SystemAttributes::read_system_attributes(&mut core);

    let apps_details = AppAttributes::read_apps_data(&mut core, system_attributes.appaddr.unwrap());

    GeneralAttributes::new(system_attributes, apps_details)
}
