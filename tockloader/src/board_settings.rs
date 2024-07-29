// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

pub struct BoardSettings {
    #[allow(dead_code)]
    pub board: String,
    pub chip: String,
    pub start_address: u64,
}

impl BoardSettings {
    pub fn new(board: String, chip: String) -> Self {
        match board.as_str() {
            "microbit_v2" => BoardSettings {
                board,
                chip,
                start_address: 0x0004_0000,
            },
            // TODO(MicuAna): inform the user we assumed we have the default settings if board is not found
            _ => BoardSettings {
                board,
                chip,
                start_address: 0x0003_0000,
            },
        }
    }
}
