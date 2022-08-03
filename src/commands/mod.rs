mod door;
mod init;
mod clean;
mod character;


use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new cl-rpg game in current directory
    Init(init::Init),

    /// Kick open the door and face you foe!
    Clean(clean::Clean),

    /// Show the current status
    Status,

    Character(character::Character),

    /// Kick open the door and face you foe!
    Door(door::Door),
}

pub fn process_command(command: &Commands) {
    match command {
        Commands::Init(init) => init::process_init(&init),
        Commands::Door(door) => door::process_door(&door),
        Commands::Clean(clean) => clean::process_clean(&clean),
        Commands::Character(character) => character::process_character(&character),
        Commands::Status => ()
    }
}