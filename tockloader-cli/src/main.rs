// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

mod cli;
mod display;

use anyhow::{Context, Result};
use clap::ArgMatches;
use cli::make_cli;
use display::{print_info, print_list};
use inquire::Select;
use tockloader_lib::{
    connection::{Connection, ConnectionInfo, SerialTargetInfo},
    info, install_app, list, list_debug_probes, list_serial_ports,
    tabs::tab::Tab,
};

fn get_serial_target_info(user_options: &ArgMatches) -> SerialTargetInfo {
    let mut result = SerialTargetInfo::default();

    // if let Some(vla) = user_optionst
    todo!()
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = make_cli().get_matches();

    if matches.get_flag("debug") {
        println!("Debug mode enabled.");
    }

    // match matches.subcommand() {
    //     Some(("listen", _sub_matches)) => {
    //         tock_process_console::run()
    //             .await
    //             .context("Failed to run console.")?;
    //     }
    //     Some(("list", sub_matches)) => {
    //         if *sub_matches.get_one::<bool>("serial").unwrap_or(&false) {
    //             let serial_ports = list_serial_ports().context("Failed to list serial ports.")?;
    //             let port_names: Vec<_> = serial_ports.iter().map(|p| p.port_name.clone()).collect();
    //             let ans = Select::new("Which serial port do you want to use?", port_names)
    //                 .prompt()
    //                 .context("No device is connected.")?;
    //             let conn = Connection::open(ConnectionInfo::from(ans), None)
    //                 .context("Failed to open serial connection.")?;
    //             let mut apps_details = list(conn, None).await.context("Failed to list apps.")?;
    //             print_list(&mut apps_details).await;
    //         } else {
    //             let ans = Select::new("Which debug probe do you want to use?", list_debug_probes())
    //                 .prompt()
    //                 .context("No debug probe is connected.")?;
    //             let conn = Connection::open(
    //                 ConnectionInfo::ProbeInfo(ans),
    //                 Some(
    //                     sub_matches
    //                         .get_one::<String>("chip")
    //                         .context("No chip has been provided.")?
    //                         .to_string(),
    //                 ),
    //             )
    //             .context("Failed to open probe connection.")?;
    //             let mut apps_details = list(conn, sub_matches.get_one::<usize>("core"))
    //                 .await
    //                 .context("Failed to list apps.")?;
    //             print_list(&mut apps_details).await;
    //         }
    //     }
    //     Some(("info", sub_matches)) => {
    //         if *sub_matches.get_one::<bool>("serial").unwrap_or(&false) {
    //             let serial_ports = list_serial_ports().context("Failed to list serial ports.")?;
    //             // Let the user choose the port that will be used
    //             let port_names: Vec<_> = serial_ports.iter().map(|p| p.port_name.clone()).collect();
    //             let ans = Select::new("Which serial port do you want to use?", port_names)
    //                 .prompt()
    //                 .context("No device is connected.")?;
    //             // Open connection
    //             let conn = Connection::open(ConnectionInfo::from(ans), None)
    //                 .context("Failed to open serial connection.")?;
    //             let mut attributes = info(conn, None)
    //                 .await
    //                 .context("Failed to get data from the board.")?;
    //             print_info(&mut attributes.apps, &mut attributes.system).await;
    //         } else {
    //             // TODO(Micu Ana): Add error handling
    //             let ans = Select::new("Which debug probe do you want to use?", list_debug_probes())
    //                 .prompt()
    //                 .context("No debug probe is connected.")?;
    //             // Open connection
    //             let conn = Connection::open(
    //                 ConnectionInfo::ProbeInfo(ans),
    //                 Some(
    //                     sub_matches
    //                         .get_one::<String>("chip")
    //                         .context("No chip has been provided.")?
    //                         .to_string(),
    //                 ),
    //             )
    //             .context("Failed to open probe connection.")?;

    //             let mut attributes = info(conn, sub_matches.get_one::<usize>("core"))
    //                 .await
    //                 .context("Failed to get data from the board.")?;

    //             print_info(&mut attributes.apps, &mut attributes.system).await;
    //         }
    //     }
    //     Some(("install", sub_matches)) => {
    //         let tab_file = Tab::open(sub_matches.get_one::<String>("tab").unwrap().to_string())
    //             .context("Failed to use provided tab file.")?;
    //         // If "--serial" flag is used, we choose the serial connection
    //         if sub_matches.get_one::<bool>("serial").is_some() {
    //             let serial_ports = list_serial_ports().context("Failed to list serial ports.")?;
    //             // Let the user choose the port that will be used
    //             let port_names: Vec<_> = serial_ports.iter().map(|p| p.port_name.clone()).collect();
    //             let ans = Select::new("Which serial port do you want to use?", port_names)
    //                 .prompt()
    //                 .context("No device is connected.")?;
    //             // Open connection
    //             let conn = Connection::open(ConnectionInfo::from(ans), None)
    //                 .context("Failed to open serial connection.")?;
    //             // Install app
    //             install_app(conn, None, tab_file)
    //                 .await
    //                 .context("Failed to install app.")?;
    //         } else {
    //             let ans = Select::new("Which debug probe do you want to use?", list_debug_probes())
    //                 .prompt()
    //                 .context("No debug probe is connected.")?;
    //             let conn = Connection::open(
    //                 ConnectionInfo::ProbeInfo(ans),
    //                 Some(
    //                     sub_matches
    //                         .get_one::<String>("chip")
    //                         .context("No chip has been provided.")?
    //                         .to_string(),
    //                 ),
    //             )
    //             .context("Failed to open probe connection.")?;
    //             // Install app
    //             install_app(conn, sub_matches.get_one::<usize>("core"), tab_file)
    //                 .await
    //                 .context("Failed to install app.")?;
    //         }
    //     }
    //     _ => {
    //         println!("Could not run the provided subcommand.");
    //         _ = make_cli().print_help();
    //     }
    // }
    Ok(())
}
