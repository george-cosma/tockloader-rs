// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use self::{serial::SerialInterface, traits::*};
use crate::errors::{CLIError, TockloaderError};
use clap::ArgMatches;
use enum_dispatch::enum_dispatch;

pub mod serial;
pub mod traits;

#[enum_dispatch(BoardInterface)]
#[enum_dispatch(VirtualTerminal)]
pub enum Interface {
    Serial(SerialInterface),
}

// TODO(NegrilaRares): investigate if we need port
#[allow(dead_code)]
pub fn build_interface(args: &ArgMatches) -> Result<Interface, TockloaderError> {
    if args.get_flag("serial") as u8 > 1 {
        return Err(TockloaderError::CLIError(CLIError::MultipleInterfaces));
    }

    Ok(SerialInterface::new(args)?.into())
}
