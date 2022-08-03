#![feature(format_args_nl)]
#![feature(print_internals)]

mod commands;
mod utils;

use std::{fs::File, io::Write};

use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(author)]
#[clap(version)]
#[clap(about, long_about = None)]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: commands::Commands,

}



fn main() {
    let cli = Cli::parse();

    commands::process_command(&cli.command);
}