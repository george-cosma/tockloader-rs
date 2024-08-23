// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use probe_rs::{Core, MemoryInterface};

use tbf_parser::{
    self,
    parse::{parse_tbf_header, parse_tbf_header_lengths},
    types::TbfHeader,
};

#[derive(Debug)]
pub struct AppAttributes {
    pub tbf_header: TbfHeader,
}

impl AppAttributes {
    pub(crate) fn new(header_data: TbfHeader) -> AppAttributes {
        AppAttributes {
            tbf_header: header_data,
        }
    }

    // TODO: Document this function
    pub(crate) fn read_apps_data(board_core: &mut Core, addr: u64) -> Vec<AppAttributes> {
        let mut appaddr: u64 = addr;
        let mut apps_counter = 0;
        let mut apps_details: Vec<AppAttributes> = vec![];

        loop {
            let mut appdata = vec![0u8; 8];

            // Do not ignore the result of the read operation
            board_core.read(appaddr, &mut appdata).unwrap();

            let tbf_version: u16;
            let header_size: u16;
            let total_size: u32;

            match parse_tbf_header_lengths(&appdata.try_into().unwrap()) {
                Ok(data) => {
                    tbf_version = data.0;
                    header_size = data.1;
                    total_size = data.2;
                }
                _ => break,
            };

            let mut header_data = vec![0u8; header_size as usize];

            // Do not ignore the result of the read operation
            board_core.read(appaddr, &mut header_data).unwrap();

            let header: TbfHeader = parse_tbf_header(&header_data, tbf_version)
                .unwrap_or_else(|e| panic!("Error found while getting tbf header data: {:?}", e));

            let details: AppAttributes = AppAttributes::new(header);

            apps_details.insert(apps_counter, details);
            apps_counter += 1;
            appaddr += total_size as u64;
        }
        apps_details
    }
}
