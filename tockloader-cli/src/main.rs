// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

mod cli;
mod display;
mod errors;

use cli::make_cli;
use display::{print_info, print_list};
use errors::TockloaderError;
use inquire::Select;
use tockloader_lib::{
    connection::{Connection, ConnectionInfo},
    info_probe, install_app_probe_rs, install_app_serial, list_debug_probes, list_probe,
    list_serial_ports,
    tabs::tab::Tab,
};

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
            match tock_process_console::run().await {
                Ok(()) => {}
                Err(_) => {
                    print!("cli bricked!")
                }
            };
        }
        Some(("list", sub_matches)) => {
            // TODO(george-cosma):inspect-err
            // TODO(Micu Ana): Add error handling
            let ans =
                Select::new("Which debug probe do you want to use?", list_debug_probes()).prompt();
            // Open connection
            let conn = Connection::open(
                tockloader_lib::connection::ConnectionInfo::ProbeInfo(ans.unwrap()),
                Some(sub_matches.get_one::<String>("chip").unwrap().to_string()),
            );

            let mut apps_details =
                list_probe(conn.unwrap(), sub_matches.get_one::<usize>("core").unwrap())
                    .await
                    .unwrap();
            print_list(&mut apps_details).await;
        }
        Some(("info", sub_matches)) => {
            // TODO(Micu Ana): Add error handling
            let ans =
                Select::new("Which debug probe do you want to use?", list_debug_probes()).prompt();
            // Open connection
            let conn = Connection::open(
                tockloader_lib::connection::ConnectionInfo::ProbeInfo(ans.unwrap()),
                Some(sub_matches.get_one::<String>("chip").unwrap().to_string()),
            );

            let mut attributes =
                info_probe(conn.unwrap(), sub_matches.get_one::<usize>("core").unwrap())
                    .await
                    .unwrap();

            print_info(&mut attributes.apps, &mut attributes.system).await;
        }
        Some(("install", sub_matches)) => {
            let tab_file =
                Tab::open(sub_matches.get_one::<String>("tab").unwrap().to_string()).unwrap();
            // If "--serial" flag is used, we choose the serial connection
            if *sub_matches.get_one::<bool>("serial").unwrap() {
                let serial_ports = list_serial_ports();
                // Let the user choose the port that will be used
                let mut port_names = Vec::new();
                for port in serial_ports {
                    port_names.push(port.port_name);
                }
                // TODO(Micu Ana): Add error handling
                let ans = Select::new("Which serial port do you want to use?", port_names)
                    .prompt()
                    .unwrap();
                // Open connection
                let conn = Connection::open(ConnectionInfo::from(ans), None);
                // Install app
                install_app_serial(
                    conn.unwrap(),
                    sub_matches.get_one::<String>("board").unwrap(),
                    tab_file,
                )
                .await
                .unwrap();
            } else {
                // TODO(Micu Ana): Add error handling
                let ans = Select::new("Which debug probe do you want to use?", list_debug_probes())
                    .prompt();
                // Open connection
                let conn = Connection::open(
                    tockloader_lib::connection::ConnectionInfo::ProbeInfo(ans.unwrap()),
                    Some(sub_matches.get_one::<String>("chip").unwrap().to_string()),
                );
                // Install app
                install_app_probe_rs(
                    conn.unwrap(),
                    sub_matches.get_one::<usize>("core").unwrap(),
                    tab_file,
                )
                .await
                .unwrap();
            }
        }
        _ => {
            println!("Could not run the provided subcommand.");
            _ = make_cli().print_help();
        }
    }
    // TODO(Micu Ana): Add error handling
    Ok(())
}
