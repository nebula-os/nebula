extern crate clap;
extern crate semver;

use clap::{App, Arg, SubCommand};

pub mod commands;

const VERSION: &str = "0.0.1";

fn main() {
    use commands::Command;

    // Set the CLI up
    let matches = App::new("Nebula Package Keeper")
        .version(get_version_str())
        .author(get_author())
        .about("The package manager for Nebula OS")
        .subcommand(commands::add::AddCommand::clap_command())
        .get_matches();
}

pub fn get_version() -> semver::Version {
    VERSION.parse().unwrap()
}

pub fn get_version_str() -> &'static str {
    VERSION
}

pub fn get_author() -> &'static str {
    "Aleksey <ahalahan@gmail.com> Halahan"
}
