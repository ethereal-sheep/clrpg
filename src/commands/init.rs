use crate::{errln, warnln, infoln};
use crate::utils::common::*;

use std::fs::{remove_dir_all};

use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct Init {
    /// Force the removal of an existing dungeon before creation.
    #[clap(short, long, action)]
    force: bool,
    
    /// Reset the dungeon (deletes history, characters)
    #[clap(short, long, action)]
    reset: bool,

    /// Seed the dungeon with the given seed (as u64)
    #[clap(short, long, value_parser, conflicts_with("reset"))]
    seed: Option<u64>,
}


fn create(init: &Init) -> Result<bool, String> {
    let mut destroyed = false;
    if check_root()? { // found existing
        if init.force || init.reset { // --force or --reset flag set 
            warnln!("Found existing {0}. Cleaning...", ROOT_FOLDER_NAME);
            remove_dir_all(ROOT_FOLDER_NAME).unwrap();
            warnln!("Deleted {}", ROOT_FOLDER_NAME);
            destroyed = true;
        } else { // return err
            return Err(format!("{} already exists", ROOT_FOLDER_NAME));
        }
    } 
    infoln!("Creating...");

    create_root()?;
    infoln!("Created {}", ROOT_FOLDER_NAME);
    
    create_char()?;
    infoln!("Created {}", CHAR_FOLDER_NAME);

    let seed = create_rand(init.seed)?;
    infoln!("Seeding dungeon with seed={}", seed);
    infoln!("Created {}", RAND_FILE_NAME);

    create_meta(Meta { seed })?;
    infoln!("Created {}", META_FILE_NAME);


    Ok(destroyed)
}


pub fn reset(init: &mut Init) -> Result<(), String> {
    require_root()?;
    let meta = require_meta()?;

    infoln!("Found current seed={}", meta.seed);
    init.seed = Some(meta.seed);

    match create(&init) {
        Ok(_) => (),
        Err(err) => {
            errln!("{}", err);
            return Err(format!("{}", "Failed to reset the dungeon."));
        }
    }

    Ok(())
}

pub fn process_init(init: &mut Init) {
    
    //print_logo();    
    
    if init.reset {
        infoln!("Resetting...");
        match reset(init) {
            Ok(_) => {
                infoln!("{}", "Reset succeeded.");
                println!("{}", "The dungeon seems to have reverted to its original state!");
            },
            Err(err) => {
                errln!("{}", "Reset failed.");
                println!("{}", err.red());
            }
        }
    } else {
        infoln!("Initializing...");
        match create(&init) {
            Ok(destroyed) => {
                infoln!("{}", "Init succeeded.");
                if destroyed {
                    println!("{}", "The dungeon crumbles as a new one takes its place!");
                } else {
                    println!("{}", "A dungeon has appeared!");
                }
            },
            Err(err) => {
                errln!("{}", err);
                errln!("{}", "Init failed.");
                println!("{}", "Failed to create the dungeon.".red());
            }
        }
    }

    

}