use crate::{errln, infoln};
use super::utils::common::*;

use std::fs::{create_dir, remove_dir_all, File};
use std::path::Path;
use clap::{AppSettings, Parser, Subcommand, Args};
use colored::Colorize;

#[derive(Args)]
pub struct Init {
    /// Remove any existing dungeon before initialization.
    #[clap(short, long, action)]
    force: bool,

    // #[clap(short, long, value_parser)]
    // character_name: Option<String>,
}


fn create(init: &Init) -> Result<(), String> {

    if check_root(&init)? { // found existing and -f is true
        infoln!("Found existing {0}. Cleaning...", ROOT_FOLDER_NAME);
        remove_dir_all(ROOT_FOLDER_NAME).unwrap();
        infoln!("Deleted {}", ROOT_FOLDER_NAME);
    } else { // not found
        infoln!("{0} not found. Creating...", ROOT_FOLDER_NAME);
    }

    create_root()?;
    infoln!("Created {}", ROOT_FOLDER_NAME);
    
    create_character_list()?;
    infoln!("Created {}", CHAR_LIST_NAME);


    Ok(())
}

fn check_root(init: &Init) -> Result<bool, String> {
    let result = Path::new(ROOT_FOLDER_NAME).try_exists();

    match &result {
        Err(_) => Err(format!("Unable to determine existence of {}", ROOT_FOLDER_NAME)),
        Ok(t) if *t => {
            if init.force {
                Ok(true)
            } else {
                Err(format!("{} already exists", ROOT_FOLDER_NAME))
            }
        },
        Ok(_) => Ok(false)
        
    }
}

fn create_character_list() -> Result<(), String> {

    let result = File::create(CHAR_LIST_NAME);

    match &result {
        Err(_) => Err(format!("Unable to create {}", CHAR_LIST_NAME)),
        Ok(_) => {
            Ok(())
        }
    }
}

fn create_root() -> Result<(), String> {

    create_dir(ROOT_FOLDER_NAME).unwrap();
    Ok(())
}


pub fn process_init(init: &Init) {
    
    //print_logo();    
    infoln!("Initializing...");
    
    match create(&init) {
        Ok(_) => {
            
        },
        Err(err) => errln!("{}", err)
    }

}