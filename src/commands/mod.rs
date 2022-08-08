mod door;
mod init;
mod clean;
mod character;
mod status;
mod run;


use clap::Subcommand;
use colored::Colorize;

use crate::{utils::print::print_logo, infoln};

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new cl-rpg game in current directory
    Init(init::Init),

    /// Clean current game
    Clean,

    /// Alias for character list
    List(character::List),

    /// Alias for character new
    Nc(character::New),
    
    /// Alias for character wait
    Wait(character::Wait),

    /// Run away!
    Run,

    /// Show the current status
    Status,

    /// Manage characters
    Character(character::Character),

    /// Kick open the door and face your foe!
    Door(door::Door),

    /// Display logo
    Logo,
}

pub fn process_command(command: &mut Commands) {
    infoln!("{}", "Running clrpg...");
    match command {
        Commands::Init(init) => init::process_init(init),
        Commands::Door(door) => door::process_door(&door),
        Commands::List(list) => character::process_list(&list),
        Commands::Nc(new) => character::process_new(&new),
        Commands::Wait(wait) => character::process_wait(&wait),
        Commands::Clean => clean::process_clean(),
        Commands::Character(character) => character::process_character(&character),
        Commands::Logo => print_logo(),
        Commands::Status => status::process_status(),
        Commands::Run => run::process_run(),
    }
}