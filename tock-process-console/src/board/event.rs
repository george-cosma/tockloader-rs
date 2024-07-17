// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

#[derive(Debug, Clone)]
pub struct NewMessageEvent {
    pub app: String,
    pub pid: u8,
    pub is_app: bool,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Event {
    NewMessage(NewMessageEvent),
    LostConnection(String),
}
