// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use std::fs::OpenOptions;
use std::io::Write;
use tokio::sync::mpsc::UnboundedSender;

pub struct BoardMessage {
    pub pid: u8,
    pub is_app: bool,
    pub payload: Vec<u8>,
}

pub struct Decoder {
    buffer: Vec<u8>,
    decoded_sender: UnboundedSender<BoardMessage>,
}

impl Default for BoardMessage {
    fn default() -> Self {
        Self {
            pid: 0,
            is_app: false,
            payload: Vec::new(),
        }
    }
}

impl Decoder {
    pub fn new(decoded_sender: UnboundedSender<BoardMessage>) -> Self {
        Decoder {
            buffer: Vec::new(),
            decoded_sender,
        }
    }

    pub fn send_encoded_message(&mut self, mut buffer: Vec<u8>) {
        while buffer.contains(&0xFF) {
            let tail_index: usize = buffer.iter().position(|byte: &u8| byte.eq(&0xFF)).unwrap();

            self.buffer.append(&mut buffer[0..tail_index].to_vec());
            self.decode_buffer();
            self.buffer.clear();

            buffer = buffer[tail_index + 1..buffer.len()].to_vec();
        }

        self.buffer.append(&mut buffer);
    }

    pub fn decode_buffer(&self) {
        let is_app = self.buffer[0];
        let pid = self.buffer[1];

        let _ = self.decoded_sender.send(BoardMessage {
            pid,
            is_app: if is_app == 0 { false } else { true },
            payload: self.buffer.clone(),
        });
    }
}
