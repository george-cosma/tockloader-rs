// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::errors::TockloaderError;
use crate::tabs::metadata::Metadata;
use std::fs::File;
use std::io::Read;
use tar::Archive;

struct TbfFile {
    pub filename: String,
    pub data: Vec<u8>,
}

pub struct Tab {
    metadata: Metadata,
    tbf_files: Vec<TbfFile>,
}

impl Tab {
    pub fn open(path: String) -> Result<Self, TockloaderError> {
        let mut metadata = None;
        let mut tbf_files = Vec::new();
        let file = File::open(path).map_err(TockloaderError::UnusableTab)?;
        let mut archive = Archive::new(file);
        for file in archive.entries().map_err(TockloaderError::UnusableTab)? {
            let mut file = file.map_err(TockloaderError::UnusableTab)?;
            let path = file.path().map_err(TockloaderError::UnusableTab)?;
            let file_name = match path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name.to_owned(),
                None => continue,
            };
            if file_name == "metadata.toml" {
                let mut buf = String::new();
                file.read_to_string(&mut buf)
                    .map_err(TockloaderError::UnusableTab)?;
                metadata = Some(Metadata::new(buf)?);
            } else if file_name.ends_with(".tbf") {
                let mut data = Vec::new();

                file.read_to_end(&mut data)
                    .map_err(TockloaderError::UnusableTab)?;
                tbf_files.push(TbfFile {
                    filename: file_name.to_string(),
                    data,
                });
            }
        }

        match metadata {
            Some(metadata) => Ok(Tab {
                metadata,
                tbf_files,
            }),
            None => Err(TockloaderError::NoMetadata),
        }
    }

    pub fn is_compatible_with_kernel_verison(&self, _kernel_version: u32) -> bool {
        // Kernel version seems to not be working properly on the microbit bootloader. It is always
        // "1" despite the actual version.
        // return self.metadata.minimum_tock_kernel_version.major <= kernel_version;
        true
    }

    pub fn is_compatible_with_board(&self, board: &String) -> bool {
        if let Some(boards) = &self.metadata.only_for_boards {
            boards.contains(board)
        } else {
            true
        }
    }

    pub fn extract_binary(&self, arch: &str) -> Result<Vec<u8>, TockloaderError> {
        for file in &self.tbf_files {
            if file.filename.starts_with(arch) {
                return Ok(file.data.clone());
            }
        }

        Err(TockloaderError::NoBinaryError(arch.to_owned()))
    }
}
