// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

pub mod input_box;
mod output_box;
// pub mod output_box;
pub use input_box::InputBox;

mod component;
pub use component::{Component, ComponentRender};
