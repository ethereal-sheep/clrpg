use crate::{errln, warnln, infoln};
use crate::utils::common::*;

use std::fs::{remove_dir_all};

use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct Init {
    /// Remove any existing dungeon before initialization.
    #[clap(short, long, action)]
    force: bool,

    #[clap(short, long, value_parser)]
    seed: Option<u64>,
    // #[clap(short, long, value_parser)]
    // character_name: Option<String>,
}


fn create(init: &Init) -> Result<(), String> {

    if check_root()? { // found existing
        if init.force { // --force flag set
            warnln!("Found existing {0}. Cleaning...", ROOT_FOLDER_NAME);
            remove_dir_all(ROOT_FOLDER_NAME).unwrap();
            warnln!("Deleted {}", ROOT_FOLDER_NAME);
        } else { // return err
            return Err(format!("{} already exists", ROOT_FOLDER_NAME));
        }
    } 
    infoln!("Creating...");

    create_root()?;
    infoln!("Created {}", ROOT_FOLDER_NAME);
    
    create_char()?;
    infoln!("Created {}", CHAR_FOLDER_NAME);

    create_rand(init.seed)?;
    infoln!("Created {}", RAND_FILE_NAME);

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