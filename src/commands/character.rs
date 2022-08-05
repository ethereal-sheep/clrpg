use crate::{errln, infoln};
use crate::utils::common::*;

use clap::{Subcommand, Args};
use colored::Colorize;
use tabled::object::{Columns, Rows, Object};
use tabled::{Table, Style, Modify, Margin, Disable};


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


#[derive(Subcommand)]
enum Subcommands {
    /// Create new character.
    New(New),

    /// List all characters.
    List(List)
}

#[derive(Args)]
pub struct Character {
    #[clap(subcommand)]
    command: Subcommands,
}


fn create_new(new: &New) -> Result<String, String> {
    
    require_root()?;
    require_char()?;

    let mut state = match RandomState::single_use() {
        Ok(s) => s,
        Err(err) => return Err(
            format!("Error generating random state: {}", err.to_string())
        )
    };
    
    let name = match &new.name {
        Some(s) => s.clone(),
        None => state.generate_name()
    };

    let id = state.generate_id();

    if check_character(&name)? {
        return Err(
            format!(
                "Character {} already exist!", 
                name.yellow().bold()
            )
        );
    }

    create_character(id.clone(), name.clone())?;
    infoln!("Created {}", name.yellow().bold());
    Ok(name)
}

pub fn process_new(new: &New) {
    
    match create_new(new) {
        Ok(s) => println!("The adventurer {} walks into the tavern.", s.bold()),
        Err(err) => println!("{}", err.red())
    }
}

fn list_characters(list: &List) -> Result<String, String> {
    
    require_root()?;
    require_char()?;


    let chars = load_characters();

    let style = Style::blank()
        .lines([(1, Style::markdown().get_horizontal().horizontal(Some('-')))]);


    let table = Table::new(chars)
        .with(Disable::Column(if list.all {4..} else {3..}))
        .with(style)
        .with(
            Margin::new(0, 0, 1, 1)
                .set_fill(' ', ' ', ' ', ' ')
        )
        .with(Modify::new(Rows::first()).with(str::to_uppercase))
        .with(
            Modify::new(Columns::single(1)
                .not(Rows::first()))
                .with(|s: &str| s.yellow().to_string())
        )

        .to_string();
    
    Ok(table)

}

pub fn process_list(list: &List) {

    match list_characters(list) {
        Ok(s) => println!("{}", s),
        Err(err) => errln!("{}", err)
    }

}


pub fn process_character(character: &Character) {
        
    match &character.command {
        Subcommands::List(list) => process_list(&list),
        Subcommands::New(new) => process_new(&new)
    }

}