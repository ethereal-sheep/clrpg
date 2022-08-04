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


fn create_new(new: &New) -> Result<(), String> {
    
    if !check_root()? {
        return Err(
            format!(
                "Missing {}! Please run command: [ {} {} ]", 
                ROOT_FOLDER_NAME, 
                "clrpg".yellow(), 
                "init".black()
            )
        );
    }

    if !check_char()? {
        return Err(
            format!(
                "Missing {}! Please run command: [ {} {} {} ]", 
                CHAR_FOLDER_NAME, 
                "clrpg".yellow(), 
                "init".black(),
                "--force".black()
            )
        );
    }

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
    Ok(())
}

pub fn process_new(new: &New) {
    
    match create_new(new) {
        Ok(_) => {
            
        },
        Err(err) => errln!("{}", err)
    }
}

fn list_characters(list: &List) -> Result<(), String> {
    if !check_root()? {
        return Err(
            format!(
                "Missing {}! Please run command: [ {} {} ]", 
                ROOT_FOLDER_NAME, 
                "clrpg".yellow(), 
                "init".black()
            )
        );
    }

    if !check_char()? {
        return Err(
            format!(
                "Missing {}! Please run command: [ {} {} {} ]", 
                CHAR_FOLDER_NAME, 
                "clrpg".yellow(), 
                "init".black(),
                "--force".black()
            )
        );
    }


    let chars = load_characters();

    let style = Style::blank()
        .lines([(1, Style::markdown().get_horizontal().horizontal(Some('-')))]);


    let table = Table::new(chars)
        .with(Disable::Column(if list.all {4..} else {3..}))
        .with(style)
        .with(
            Margin::new(2, 0, 1, 1)
                .set_fill(' ', ' ', ' ', ' ')
        )
        .with(Modify::new(Rows::first()).with(str::to_uppercase))
        .with(
            Modify::new(Columns::single(1)
                .not(Rows::first()))
                .with(|s: &str| s.yellow().to_string())
        )

        .to_string();

    println!("{}", table);
    
    Ok(())

}

pub fn process_list(list: &List) {

    match list_characters(list) {
        Ok(_) => {
            
        },
        Err(err) => errln!("{}", err)
    }

}


pub fn process_character(character: &Character) {
        
    match &character.command {
        Subcommands::List(list) => process_list(&list),
        Subcommands::New(new) => process_new(&new)
    }

}