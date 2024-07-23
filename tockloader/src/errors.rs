// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use core::fmt;

#[derive(Debug)]
pub enum TockloaderError {
    TokioSeriallError(tokio_serial::Error),
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    NoPortAvailable,
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    CLIError(CLIError),
    IOError(std::io::Error),
    JoinError(tokio::task::JoinError),
}

#[derive(Debug)]
pub enum CLIError {
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    MultipleInterfaces,
}

impl fmt::Display for TockloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TockloaderError::TokioSeriallError(inner) => {
                inner.fmt(f)
            }
            TockloaderError::NoPortAvailable => {
                f.write_str("Tockloader has failed to find any open ports. If your device is plugged in, you can manually specify it using the '--port <path>' argument.")
            },
            TockloaderError::CLIError(inner) => {
                inner.fmt(f)
            }
            TockloaderError::IOError(inner) => {
                inner.fmt(f)
            },
            TockloaderError::JoinError(inner) => {
                inner.fmt(f)
            },
        }
    }
}

impl fmt::Display for CLIError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLIError::MultipleInterfaces => {
                //f.write_str("")
                todo!("TODO (github-username-here): Do we need still need this error?")
            }
        }
    }
}

impl From<std::io::Error> for TockloaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<tokio_serial::Error> for TockloaderError {
    fn from(value: tokio_serial::Error) -> Self {
        Self::TokioSeriallError(value)
    }
}

impl From<tokio::task::JoinError> for TockloaderError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::JoinError(value)
    }
}

impl std::error::Error for TockloaderError {}
impl std::error::Error for CLIError {}
