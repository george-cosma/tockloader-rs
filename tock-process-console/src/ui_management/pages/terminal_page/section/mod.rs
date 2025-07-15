// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

pub mod usage;
pub trait SectionActivation {
    fn activate(&mut self);
    fn deactivate(&mut self);
}
