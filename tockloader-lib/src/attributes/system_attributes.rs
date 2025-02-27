// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use byteorder::{ByteOrder, LittleEndian};
use probe_rs::{Core, MemoryInterface};
use tokio_serial::SerialStream;

use crate::bootloader_serial::{issue_command, Command, Response};
use crate::errors::TockloaderError;

use super::decode::{bytes_to_string, decode_attribute};

#[derive(Debug)]
pub struct SystemAttributes {
    pub board: Option<String>,
    pub arch: Option<String>,
    pub appaddr: Option<u64>,
    pub boothash: Option<String>,
    pub bootloader_version: Option<String>,
    pub sentinel: Option<String>,
    pub kernel_version: Option<u64>,
    pub app_mem_start: Option<u32>,
    pub app_mem_len: Option<u32>,
    pub kernel_bin_start: Option<u32>,
    pub kernel_bin_len: Option<u32>,
}

impl SystemAttributes {
    pub(crate) fn new() -> SystemAttributes {
        SystemAttributes {
            board: None,
            arch: None,
            appaddr: None,
            boothash: None,
            bootloader_version: None,
            sentinel: None,
            kernel_version: None,
            app_mem_start: None,
            app_mem_len: None,
            kernel_bin_start: None,
            kernel_bin_len: None,
        }
    }

    // TODO: explain what is happening here
    pub(crate) fn read_system_attributes_probe(
        board_core: &mut Core,
    ) -> Result<Self, TockloaderError> {
        let mut result = SystemAttributes::new();
        let address = 0x600;
        let mut buf = [0u8; 64 * 16];

        let _ = board_core.read(address, &mut buf);

        let mut data = buf.chunks(64);

        for index_data in 0..data.len() {
            let step = match data.next() {
                Some(data) => data,
                None => break,
            };

            let step_option = decode_attribute(step);

            if let Some(decoded_attributes) = step_option {
                match index_data {
                    0 => {
                        result.board = Some(decoded_attributes.value.to_string());
                    }
                    1 => {
                        result.arch = Some(decoded_attributes.value.to_string());
                    }
                    2 => {
                        result.appaddr = Some(
                            u64::from_str_radix(
                                decoded_attributes
                                    .value
                                    .to_string()
                                    .trim_start_matches("0x"),
                                16,
                            )
                            .map_err(|_| {
                                TockloaderError::MisconfiguredBoard(
                                    "Invalid start address.".to_owned(),
                                )
                            })?,
                        );
                    }
                    3 => {
                        result.boothash = Some(decoded_attributes.value.to_string());
                    }
                    _ => {}
                }
            } else {
                continue;
            }
        }

        let address = 0x40E;

        let mut buf = [0u8; 8];

        let _ = board_core.read_8(address, &mut buf);

        let string = String::from_utf8(buf.to_vec()).map_err(|_| {
            TockloaderError::MisconfiguredBoard(
                "Data may be corrupted. System attribure is not UTF-8.".to_owned(),
            )
        })?;

        let string = string.trim_matches(char::from(0));

        result.bootloader_version = Some(string.to_owned());

        let mut kernel_attr_binary = [0u8; 100];
        board_core
            .read(
                result.appaddr.ok_or(TockloaderError::MisconfiguredBoard(
                    "No start address found.".to_owned(),
                ))? - 100,
                &mut kernel_attr_binary,
            )
            .map_err(TockloaderError::ProbeRsReadError)?;

        let sentinel = bytes_to_string(&kernel_attr_binary[96..100]);
        let kernel_version = LittleEndian::read_uint(&kernel_attr_binary[95..96], 1);

        let app_memory_len = LittleEndian::read_u32(&kernel_attr_binary[84..92]);
        let app_memory_start = LittleEndian::read_u32(&kernel_attr_binary[80..84]);

        let kernel_binary_start = LittleEndian::read_u32(&kernel_attr_binary[68..72]);
        let kernel_binary_len = LittleEndian::read_u32(&kernel_attr_binary[72..76]);

        result.sentinel = Some(sentinel);
        result.kernel_version = Some(kernel_version);
        result.app_mem_start = Some(app_memory_start);
        result.app_mem_len = Some(app_memory_len);
        result.kernel_bin_start = Some(kernel_binary_start);
        result.kernel_bin_len = Some(kernel_binary_len);

        Ok(result)
    }

    // TODO: explain what is happening here
    pub(crate) async fn read_system_attributes_serial(
        port: &mut SerialStream,
    ) -> Result<Self, TockloaderError> {
        let mut result = SystemAttributes::new();

        let mut pkt = (0x600_u32).to_le_bytes().to_vec();
        let length = (1024_u16).to_le_bytes().to_vec();
        for i in length {
            pkt.push(i);
        }

        let (_, buf) = issue_command(
            port,
            Command::ReadRange,
            pkt,
            true,
            64 * 16,
            Response::ReadRange,
        )
        .await?;

        let mut data = buf.chunks(64);

        for index_data in 0..data.len() {
            let step = match data.next() {
                Some(data) => data,
                None => break,
            };

            let step_option = decode_attribute(step);

            if let Some(decoded_attributes) = step_option {
                match index_data {
                    0 => {
                        result.board = Some(decoded_attributes.value.to_string());
                    }
                    1 => {
                        result.arch = Some(decoded_attributes.value.to_string());
                    }
                    2 => {
                        result.appaddr = Some(
                            u64::from_str_radix(
                                decoded_attributes
                                    .value
                                    .to_string()
                                    .trim_start_matches("0x"),
                                16,
                            )
                            .map_err(|_| {
                                TockloaderError::MisconfiguredBoard(
                                    "Invalid start address.".to_owned(),
                                )
                            })?,
                        );
                    }
                    3 => {
                        result.boothash = Some(decoded_attributes.value.to_string());
                    }
                    _ => {}
                }
            } else {
                continue;
            }
        }

        let mut pkt = (0x40E_u32).to_le_bytes().to_vec();
        let length = (8_u16).to_le_bytes().to_vec();
        for i in length {
            pkt.push(i);
        }

        let (_, buf) =
            issue_command(port, Command::ReadRange, pkt, true, 8, Response::ReadRange).await?;

        let string = String::from_utf8(buf).map_err(|_| {
            TockloaderError::MisconfiguredBoard(
                "Data may be corrupted. System attribure is not UTF-8.".to_owned(),
            )
        })?;

        let string = string.trim_matches(char::from(0));

        result.bootloader_version = Some(string.to_owned());

        let mut pkt = ((result.appaddr.ok_or(TockloaderError::MisconfiguredBoard(
            "No start address found.".to_owned(),
        ))? - 100) as u32)
            .to_le_bytes()
            .to_vec();
        let length = (100_u16).to_le_bytes().to_vec();
        for i in length {
            pkt.push(i);
        }

        let (_, kernel_attr_binary) = issue_command(
            port,
            Command::ReadRange,
            pkt,
            true,
            100,
            Response::ReadRange,
        )
        .await?;

        let sentinel = bytes_to_string(&kernel_attr_binary[96..100]);
        let kernel_version = LittleEndian::read_uint(&kernel_attr_binary[95..96], 1);

        let app_memory_len = LittleEndian::read_u32(&kernel_attr_binary[84..92]);
        let app_memory_start = LittleEndian::read_u32(&kernel_attr_binary[80..84]);

        let kernel_binary_start = LittleEndian::read_u32(&kernel_attr_binary[68..72]);
        let kernel_binary_len = LittleEndian::read_u32(&kernel_attr_binary[72..76]);

        result.sentinel = Some(sentinel);
        result.kernel_version = Some(kernel_version);
        result.app_mem_start = Some(app_memory_start);
        result.app_mem_len = Some(app_memory_len);
        result.kernel_bin_start = Some(kernel_binary_start);
        result.kernel_bin_len = Some(kernel_binary_len);

        Ok(result)
    }
}
