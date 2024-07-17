// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::errors::TockloaderError;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}

#[async_trait]
#[enum_dispatch]
pub trait VirtualTerminal {
    async fn run_terminal(&mut self) -> Result<(), TockloaderError>;
}
