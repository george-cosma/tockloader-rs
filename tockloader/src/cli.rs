use clap::{arg, crate_version, value_parser, Command};

/// Create the [command](clap::Command) object which will handle all of the command line arguments.
pub fn make_cli() -> Command {
    Command::new("tockloader")
        .about("This is a sample description.")
        .version(crate_version!())
        .subcommand_required(true)
        .subcommands(get_subcommands())
        .args([
            arg!(--debug "Print additional debugging information").action(clap::ArgAction::SetTrue)
        ])
    // Note: arg_require_else_help will trigger the help command if no argument/subcommand is given.
    // This means that the --debug flag will not trigger the help menu, even if alone it does nothing.
}

/// Generate all of the [subcommands](clap::Command) used by the program.
fn get_subcommands() -> Vec<Command> {
    vec![Command::new("listen")
        .about("Open a terminal to receive UART data")
        .args(get_app_args())
        .args(get_channel_args())
        .arg_required_else_help(false)]
}

/// Generate all of the [arguments](clap::Arg) that are required by subcommands which work with apps.
fn get_app_args() -> Vec<clap::Arg> {
    vec![
        arg!(-a --"app-address" <ADDRESS> "Address where apps are located"),
        arg!(--force "Allow apps on boards that are not listed as compatible")
            .action(clap::ArgAction::SetTrue),
        arg!(--"bundle-apps" "Concatenate apps and flash all together, re-flashing apps as needed")
            .action(clap::ArgAction::SetTrue),
    ]
    // Note: the .action(clap::ArgAction::SetTrue) doesn't seem to be necessary, though in clap documentation it is used.
}

/// Generate all of the [arguments](clap::Arg) that are required by subcommands which work
/// with channels and computer-board communication.
fn get_channel_args() -> Vec<clap::Arg> {
    vec![
        arg!(-p --port <PORT> "The serial port or device name to use"),
        arg!(--serial "Use the serial bootloader to flash").action(clap::ArgAction::SetTrue),
        // -----
        arg!(--"flash-file" "Operate on a binary flash file instead of a proper board")
            .action(clap::ArgAction::SetTrue),
        arg!(--board <BOARD> "Explicitly specify the board that is being targeted"),
        arg!(--arch <ARCH> "Explicitly specify the architecture of the board that is being targeted"),
        arg!(--"page-size" <SIZE> "Explicitly specify how many bytes in a flash page")
            .default_value("0"),
        arg!(--"baud-rate" <RATE> "If using serial, set the target baud rate")
            .value_parser(value_parser!(u32))
            .default_value("115200"),
        arg!(--"no-bootloader-entry" "Tell Tockloader to assume the bootloader is already active")
            .action(clap::ArgAction::SetTrue),
    ]
}
