// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Action {
    ConnectToBoard {
        port: String,
    },
    AddScreen {
        screen_idx: usize,
    },
    #[allow(dead_code)]
    RemoveSreen {
        screend_idx: usize,
    },
    SelectApplication {
        screen_idx: usize,
        app_name: String,
    },
    SendMessage {
        content: Bytes,
    },
    #[allow(dead_code)]
    ResizeScreen {
        rows: usize,
        columns: usize,
    },
    Exit,
}
