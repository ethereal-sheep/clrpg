#![feature(format_args_nl)]
#![feature(print_internals)]

mod commands;
mod utils;

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
    
    /// Verbose debug information
    #[clap(short, long, action)]
    verbose: bool

}



fn main() {
    let mut cli = Cli::parse();

    // use crate::utils::print::VERBOSE;
    // VERBOSE.with(|b| *b.borrow_mut() = cli.verbose);

    commands::process_command(&mut cli.command);
}