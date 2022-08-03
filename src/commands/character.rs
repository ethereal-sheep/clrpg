use crate::{errln, infoln};
use crate::utils::common::*;

use std::fs::{create_dir, remove_dir_all, File, OpenOptions};
use std::io::{Write, ErrorKind};
use std::path::Path;
use clap::{AppSettings, Parser, Subcommand, Args};
use colored::Colorize;
use rand::Rng;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};


#[derive(Args)]
pub struct New {
    /// New character name.
    #[clap(short, long, value_parser)]
    Name: Option<String>,
}

#[derive(Args)]
pub struct List {
    /// New character name.
    #[clap(short, long, action)]
    All: bool,
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




// fn create(init: &Init) -> Result<(), String> {

//     if check_root(&init)? { // found existing and -f is true
//         infoln!("Found existing {0}. Cleaning...", ROOT_FOLDER_NAME);
//         remove_dir_all(ROOT_FOLDER_NAME).unwrap();
//         infoln!("Deleted {}", ROOT_FOLDER_NAME);
//     } else { // not found
//         infoln!("{0} not found. Creating...", ROOT_FOLDER_NAME);
//     }

//     create_root()?;
//     infoln!("Created {}", ROOT_FOLDER_NAME);
    
//     create_character_list()?;
//     infoln!("Created {}", CHAR_LIST_NAME);


//     Ok(())
// }

// fn check_root(init: &Init) -> Result<bool, String> {
//     let result = Path::new(ROOT_FOLDER_NAME).try_exists();

//     match &result {
//         Err(_) => Err(format!("Unable to determine existence of {}", ROOT_FOLDER_NAME)),
//         Ok(t) if *t => {
//             if init.force {
//                 Ok(true)
//             } else {
//                 Err(format!("{} already exists", ROOT_FOLDER_NAME))
//             }
//         },
//         Ok(_) => Ok(false)
        
//     }
// }

// fn create_character_list() -> Result<(), String> {

//     let result = File::create(CHAR_LIST_NAME);

//     match &result {
//         Err(_) => Err(format!("Unable to create {}", CHAR_LIST_NAME)),
//         Ok(_) => {
//             Ok(())
//         }
//     }
// }

// fn create_root() -> Result<(), String> {

//     create_dir(ROOT_FOLDER_NAME).unwrap();
//     Ok(())
// }

fn load_characters() {

}


fn generate_name() -> Result<String, String> {

    let mut state = match RandomState::single_use() {
        Ok(s) => s,
        Err(err) => return Err(
            format!("Error generating random state: {}", err.to_string())
        )
    };

    loop {
        
        let adjectives = include_str!("../../res/adjectives.txt").lines();
        let animals = include_str!("../../res/animals.txt").lines();

        let adjective = adjectives.choose_stable(&mut state.rng).unwrap();
        let animal = animals.choose_stable(&mut state.rng).unwrap();

        let name = format!("{}{}", adjective, animal);
        match check_character(&name) {
            Ok(t) if !t => return Ok(format!("{}{}", adjective, animal)),
            _ => ()
        }
    }


}

fn create_new(name: &str) -> Result<(), String> {
    
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

    if check_character(name)? {
        return Err(
            format!(
                "Character {} already exist!", 
                name.yellow().bold()
            )
        );
    }
    

    create_character(name)?;
    infoln!("Created {}", name.yellow().bold());
    Ok(())


    
    
}


fn process_new(new: &New) {
    
    let name = match generate_name() {
        Ok(s) => s,
        Err(err) => {
            errln!("{}", err);
            return;
        }    
    };

    let name = new.Name
        .as_ref()
        .map_or_else(|| &name, |name| name );

    match create_new(name) {
        Ok(_) => {
            
        },
        Err(err) => errln!("{}", err)
    }
}


pub fn process_character(character: &Character) {
        
    match &character.command {
        Subcommands::List(list) => (),
        Subcommands::New(new) => process_new(&new)
    }

}