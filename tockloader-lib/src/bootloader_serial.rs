// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

// The "X" commands are for external flash

use crate::errors;
use bytes::BytesMut;
use errors::TockloaderError;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{SerialPort, SerialStream};

// Tell the bootloader to reset its buffer to handle a new command
pub const SYNC_MESSAGE: [u8; 3] = [0x00, 0xFC, 0x05];

// "This was chosen as it is infrequent in .bin files" - immesys
pub const ESCAPE_CHAR: u8 = 0xFC;

#[allow(dead_code)]
pub enum Command {
    // Commands from this tool to the bootloader
    Ping = 0x01,
    Info = 0x03,
    ID = 0x04,
    Reset = 0x05,
    ErasePage = 0x06,
    WritePage = 0x07,
    XEBlock = 0x08,
    XWPage = 0x09,
    Crcx = 0x10,
    ReadRange = 0x11,
    XRRange = 0x12,
    SetAttribute = 0x13,
    GetAttribute = 0x14,
    CRCInternalFlash = 0x15,
    Crcef = 0x16,
    XEPage = 0x17,
    XFinit = 0x18,
    ClkOut = 0x19,
    WUser = 0x20,
    ChangeBaudRate = 0x21,
    Exit = 0x22,
    SetStartAddress = 0x23,
}

#[derive(Clone, Debug)]
pub enum Response {
    // Responses from the bootloader
    Overflow = 0x10,
    Pong = 0x11,
    BadAddr = 0x12,
    IntError = 0x13,
    BadArgs = 0x14,
    OK = 0x15,
    Unknown = 0x16,
    XFTimeout = 0x17,
    Xfepe = 0x18,
    Crcrx = 0x19,
    ReadRange = 0x20,
    XRRange = 0x21,
    GetAttribute = 0x22,
    CRCInternalFlash = 0x23,
    Crcxf = 0x24,
    Info = 0x25,
    ChangeBaudFail = 0x26,
    BadResp,
}

impl From<u8> for Response {
    fn from(value: u8) -> Self {
        match value {
            0x10 => Response::Overflow,
            0x11 => Response::Pong,
            0x12 => Response::BadAddr,
            0x13 => Response::IntError,
            0x14 => Response::BadArgs,
            0x15 => Response::OK,
            0x16 => Response::Unknown,
            0x17 => Response::XFTimeout,
            0x18 => Response::Xfepe,
            0x19 => Response::Crcrx,
            0x20 => Response::ReadRange,
            0x21 => Response::XRRange,
            0x22 => Response::GetAttribute,
            0x23 => Response::CRCInternalFlash,
            0x24 => Response::Crcxf,
            0x25 => Response::Info,
            0x26 => Response::ChangeBaudFail,

            // This error handling is temmporary
            //TODO(Micu Ana): Add error handling
            _ => Response::BadResp,
        }
    }
}

#[allow(dead_code)]
pub async fn toggle_bootloader_entry_dtr_rts(
    port: &mut SerialStream,
) -> Result<(), TockloaderError> {
    port.write_data_terminal_ready(true)
        .map_err(TockloaderError::SerialInitializationError)?;
    port.write_request_to_send(true)
        .map_err(TockloaderError::SerialInitializationError)?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    port.write_data_terminal_ready(false)
        .map_err(TockloaderError::SerialInitializationError)?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    port.write_request_to_send(false)
        .map_err(TockloaderError::SerialInitializationError)?;

    Ok(())
}

#[allow(dead_code)]
pub async fn ping_bootloader_and_wait_for_response(
    port: &mut SerialStream,
) -> Result<Response, TockloaderError> {
    let ping_pkt = [ESCAPE_CHAR, Command::Ping as u8];

    let mut ret = BytesMut::with_capacity(200);

    for _ in 0..30 {
        let mut bytes_written = 0;
        while bytes_written != ping_pkt.len() {
            bytes_written += port.write_buf(&mut &ping_pkt[bytes_written..]).await?;
        }
        let mut read_bytes = 0;
        while read_bytes < 2 {
            read_bytes += port.read_buf(&mut ret).await?;
        }
        if ret[1] == Response::Pong as u8 {
            return Ok(Response::from(ret[1]));
        }
    }
    Ok(Response::from(ret[1]))
}

#[allow(dead_code)]
pub async fn issue_command(
    port: &mut SerialStream,
    command: Command,
    mut message: Vec<u8>,
    sync: bool,
    response_len: usize,
    response_code: Response,
) -> Result<(Response, Vec<u8>), TockloaderError> {
    // Setup a command to send to the bootloader and handle the response
    // Generate the message to send to the bootloader
    let mut i = 0;
    while i < message.len() {
        if message[i] == ESCAPE_CHAR {
            // Escaped by replacing all 0xFC with two consecutive 0xFC - tock bootloader readme
            message.insert(i + 1, ESCAPE_CHAR);
            // Skip the inserted character
            i += 2;
        } else {
            i += 1;
        }
    }
    message.push(ESCAPE_CHAR);
    message.push(command as u8);

    // If there should be a sync/reset message, prepend the outgoing message with it
    if sync {
        message.insert(0, SYNC_MESSAGE[0]);
        message.insert(1, SYNC_MESSAGE[1]);
        message.insert(2, SYNC_MESSAGE[2]);
    }

    // Write the command message
    let mut bytes_written = 0;
    while bytes_written != message.len() {
        bytes_written += port.write_buf(&mut &message[bytes_written..]).await?;
    }

    // Response has a two byte header, then response_len bytes
    let bytes_to_read = 2 + response_len;
    let mut ret = BytesMut::with_capacity(2);

    // We are waiting for 2 bytes to be read
    let mut read_bytes = 0;
    while read_bytes < 2 {
        read_bytes += port.read_buf(&mut ret).await?;
    }

    if ret[0] != ESCAPE_CHAR {
        return Err(TockloaderError::BootloaderError(ret[0]));
    }

    if ret[1] != response_code.clone() as u8 {
        return Err(TockloaderError::BootloaderError(ret[1]));
    }

    let mut new_data: Vec<u8> = Vec::new();
    let mut value = 2;

    if response_len != 0 {
        while bytes_to_read > value {
            value += port.read_buf(&mut new_data).await?;
        }

        // De-escape and add array of read in the bytes
        for i in 0..(new_data.len() - 1) {
            if new_data[i] == ESCAPE_CHAR && new_data[i + 1] == ESCAPE_CHAR {
                new_data.remove(i + 1);
            }
        }

        ret.extend_from_slice(&new_data);
    }

    Ok((Response::from(ret[1]), ret[2..].to_vec()))
}
