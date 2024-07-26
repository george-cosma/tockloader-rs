// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

mod board_settings;
mod cli;
mod errors;
mod interfaces;

use board_settings::BoardSettings;
use clap::ArgMatches;
use cli::make_cli;
use errors::TockloaderError;

use glob::glob;
use inquire::Select;
use interfaces::{build_interface, traits::*};
use probe_rs::probe::list::Lister;
use probe_rs::{MemoryInterface, Permissions};
use tbf_parser::parse::*;
use tbf_parser::types::*;
use tock_process_console;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), TockloaderError> {
    let result = run().await;
    if let Err(e) = &result {
        eprintln!("\n{}", e);
    }
    result
}

async fn run() -> Result<(), TockloaderError> {
    let matches = make_cli().get_matches();

    if matches.get_flag("debug") {
        println!("Debug mode enabled");
    }

    match matches.subcommand() {
        Some(("listen", _sub_matches)) => {
            let _ = match tock_process_console::run().await {
                Ok(()) => {}
                Err(_) => {
                    print!("cli bricked!")
                }
            };
        }
        Some(("list", sub_matches)) => {
            list_probes(sub_matches).await?;
        }
        Some(("install", sub_matches)) => {}
        // If only the "--debug" flag is set, then this branch is executed
        // Or, more likely at this stage, a subcommand hasn't been implemented yet.
        _ => {
            println!("Could not run the provided subcommand.");
            _ = make_cli().print_help();
        }
    }

    Ok(())
}

async fn list_probes(sub_matches: &ArgMatches) -> Result<(), TockloaderError> {
    let lister = Lister::new();
    let probes = lister.list_all();
    println!("Probes: {:?}\n", probes);

    let ans = Select::new("Which probe do you want to use?", probes).prompt();

    match ans {
        Ok(choice) => {
            let probe = choice.open().unwrap();

            let chip = sub_matches.get_one::<String>("chip").unwrap();
            let board = sub_matches.get_one::<String>("board").unwrap();

            let board_settings = BoardSettings::new(board.clone(), chip.clone());

            let mut session = probe
                .attach(board_settings.chip, Permissions::default())
                .unwrap();

            println!("Session target: {:?}\n", session.target());
            println!("Session interfaces: {:?}\n", session.architecture());
            println!("Session core: {:?}\n", session.list_cores());

            let core_index = sub_matches.get_one::<usize>("core").unwrap();

            let mut core = session.core(*core_index).unwrap();

            let mut address = board_settings.start_address;

            // Jump through the linked list of apps
            loop {
                // Read a block of 200 8-bit words
                let mut buff = vec![0u8; 200];
                match core.read(address, &mut buff) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error reading memory: {:?}", e);
                        break;
                    }
                }

                let (ver, header_len, whole_len) =
                    match parse_tbf_header_lengths(&buff[0..8].try_into().unwrap()) {
                        Ok((ver, header_len, whole_len)) if header_len != 0 => {
                            println!("Version: {:?}\n", ver);
                            println!("Header length: {:?}\n", header_len);
                            println!("Whole length: {:?}\n", whole_len);
                            (ver, header_len, whole_len)
                        }
                        _ => break, // No more apps
                    };

                let header = parse_tbf_header(&buff[0..header_len as usize], ver);
                match header {
                    Ok(header) => {
                        println!("Enabled: {:?}\n", header.enabled());
                        println!(
                            "Minimum App Ram Size: {:?}\n",
                            header.get_minimum_app_ram_size()
                        );
                        println!(
                            "Init function offset: {:?}\n",
                            header.get_init_function_offset()
                        );
                        println!("Protected size: {:?}\n", header.get_protected_size());
                        println!(
                            "Package name: {:?}\n",
                            header.get_package_name().unwrap_or_default()
                        );
                        println!(
                            "Kernel version: {:?}\n",
                            header.get_kernel_version().unwrap_or_default()
                        );
                    }
                    // TODO(MicuAna): refactor when reworking errors
                    Err(TbfParseError::ChecksumMismatch(
                        provided_checksum,
                        calculated_checksum,
                    )) => {
                        println!(
                            "Checksum mismatch: provided = {}, calculated = {}",
                            provided_checksum, calculated_checksum
                        );
                        break;
                    }
                    Err(e) => {
                        println!("Failed to parse TBF header: {:?}", e);
                        break;
                    }
                }
                address += whole_len as u64;
            }
        }
        Err(err) => println!("{}", err),
    }

    Ok(())
}
