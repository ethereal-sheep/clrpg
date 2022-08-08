use crate::utils::common::*;
use crate::{errln, infoln, warnln};

use clap::{Args, Subcommand};
use colored::Colorize;
use tabled::object::{Columns, Object, Rows};
use tabled::{Disable, Margin, Modify, Style, Table};

#[derive(Args)]
pub struct New {
    /// New character name.
    #[clap(short, long, value_parser)]
    name: Option<String>,
}

#[derive(Args)]
pub struct List {
    /// New character name.
    #[clap(short, long, action)]
    all: bool,
}

#[derive(Args)]
pub struct Wait {
    /// New character name.
    #[clap(value_parser)]
    name: String,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Create new character.
    New(New),

    /// List all characters.
    List(List),

    /// Wait a character
    Wait(Wait),

}

#[derive(Args)]
pub struct Character {
    #[clap(subcommand)]
    command: Subcommands,
}

fn create_new(new: &New) -> Result<String, String> {
    require_root()?;
    require_char()?;

    let mut state = RandomState::single_use()?;

    let name = match &new.name {
        Some(s) => s.clone(),
        None => state.generate_name()?,
    };

    let id = state.generate_id();

    if check_character(&name)? {
        return Err(format!("Character {} already exist!", name.yellow().bold()));
    }

    create_character(id.clone(), name.clone(), &mut state)?;
    infoln!("Created {}", name.yellow().bold());
    Ok(name)
}

pub fn process_new(new: &New) {
    infoln!("Creating new character...");
    match create_new(new) {
        Ok(s) => {
            infoln!("{}", "Create character succeeded.");
            println!("The adventurer {} walks into the tavern.", s.bold());
        }
        Err(err) => {
            errln!("{}", "Create character failed.");
            println!("{}", err.red());
        }
    }
}

fn list_characters(list: &List) -> Result<String, String> {
    require_root()?;
    require_char()?;

    let chars = load_characters();

    let style =
        Style::blank().lines([(1, Style::markdown().get_horizontal().horizontal(Some('-')))]);

    let table = Table::new(chars)
        .with(Disable::Column(if list.all { 4.. } else { 3.. }))
        .with(style)
        .with(Margin::new(0, 0, 1, 1).set_fill(' ', ' ', ' ', ' '))
        .with(Modify::new(Rows::first()).with(str::to_uppercase))
        .with(
            Modify::new(Columns::single(1).not(Rows::first()))
                .with(|s: &str| s.yellow().to_string()),
        )
        .to_string();

    Ok(table)
}

pub fn process_list(list: &List) {
    infoln!("Listing characters...");
    match list_characters(list) {
        Ok(s) => {
            infoln!("{}", "List characters succeeded.");
            println!("{}", s);
        }
        Err(err) => {
            errln!("{}", err);
            errln!("{}", "List characters failed.");
        }
    }
}

pub fn wait_character(wait: &Wait) -> Result<String, String> {

    require_root()?;
    let char_obj = match require_character(&wait.name) {
        Ok(c) => c,
        Err(_) => {
            return Err(
                format!(
                    "The adventurer {} is not in the tavern.", 
                    wait.name.bold()
                )
            )
        },
    };

    if !char_obj.is_alive() {
        errln!("{} is dead", char_obj.get_name());
        return Err(
            format!(
                "{} is dead!", 
                char_obj.get_name().bold()
            )
        )
    }


    let mut meta = require_meta()?;

    if let Some(character) = meta.current {
        if character == wait.name {
            errln!("{} is already waiting", character);
            return Ok(
                format!(
                    "{} is already waiting at the mouth of the dungeon.", 
                    character.bold()
                )
            )
        } else if meta.status != MetaStatus::OutsideTheDungeon {
            errln!("{} is already inside the dungeon", character);
            return Err(
                format!(
                    "{} is already attempting the dungeon.", 
                    character.bold()
                )
            )
        }  else {
            warnln!("{} is already waiting", character);
            warnln!("Replacing {} with {}", character, wait.name);

        }
    }

    meta.current = Some(wait.name.clone());
    meta.status = MetaStatus::OutsideTheDungeon;
    write_meta(&meta)?;

    Ok(
        format!(
            "{} stands outside the dungeon, awaiting a glorious adventure!", 
            wait.name.bold()
        )
    )
}

pub fn process_wait(wait: &Wait) {
    infoln!("Waiting character");


    
    match wait_character(wait) {
        Ok(s) => {
            infoln!("{}", "Waiting character succeeded.");
            println!("{}", s);
        }
        Err(err) => {
            errln!("{}", "Waiting character failed.");
            println!("{}", err.red());
        }
    }
}

pub fn process_character(character: &Character) {
    match &character.command {
        Subcommands::List(list) => process_list(list),
        Subcommands::New(new) => process_new(new),
        Subcommands::Wait(wait) => process_wait(wait),
    }
}
