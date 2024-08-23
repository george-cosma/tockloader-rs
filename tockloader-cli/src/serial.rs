// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use inquire::{InquireError, Select};
use probe_rs::probe::{list::Lister, DebugProbeInfo};

pub fn select_probe() -> Result<DebugProbeInfo, InquireError> {
    let lister = Lister::new();
    let probes = lister.list_all();
    let ans = Select::new("Which probe do you want to use?", probes).prompt();
    ans
}
