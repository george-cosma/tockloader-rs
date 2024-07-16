mod cli;
mod errors;
mod interfaces;
use std::time::Duration;

use cli::make_cli;
use errors::TockloaderError;
use interfaces::{build_interface, traits::*};
use tokio::time::sleep;

use tock_process_console;


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
        Some(("listen", sub_matches)) => {
            let mut interface = build_interface(sub_matches)?;
            // interface.open()?;
            sleep(Duration::new(5, 0)).await;

            let _= match tock_process_console::run().await
            {
                Ok(()) => {},
                Err(_) => {print!("cli bricked!")},
            };
            // interface.run_terminal().await?;
        }
        // If only the "--debug" flag is set, then this branch is executed
        // Or, more likely at this stage, a subcommand hasn't been implemented yet.
        _ => {
            println!("Could not run the provided subcommand.");
            _ = make_cli().print_help();
        }
    }

    Ok(())
}
