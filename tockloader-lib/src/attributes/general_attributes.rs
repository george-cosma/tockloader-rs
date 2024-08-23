// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use super::{app_attributes::AppAttributes, system_attributes::SystemAttributes};

#[derive(Debug)]
pub struct GeneralAttributes {
    pub system: SystemAttributes,
    pub apps: Vec<AppAttributes>,
}

impl GeneralAttributes {
    pub(crate) fn new(
        system_attributes: SystemAttributes,
        apps_attributes: Vec<AppAttributes>,
    ) -> GeneralAttributes {
        GeneralAttributes {
            system: system_attributes,
            apps: apps_attributes,
        }
    }
}
