use crate::{errln, infoln};
use crate::utils::common::*;

use colored::Colorize;

fn run_away() -> Result<String, String> {
    require_root()?;

    
    let mut meta = require_meta()?;

    let character = match meta.current {
        Some(s) => s,
        None => return Err(
            format!(
                "{}", "No one is in the dungeon right now!"
            )
        ),
    };
    let curr_status = meta.status;
    meta.current = None;
    meta.status = MetaStatus::HelpWanted;
    write_meta(&meta)?;


    let ret = match curr_status {
        MetaStatus::HelpWanted => return Err("Unexpected error occured".to_string()),
        MetaStatus::OutsideTheDungeon => format!(
            "{} decides to go to the tavern for a drink instead!", 
            character.bold()
        ),
        MetaStatus::InTheDungeon => format!(
            "{} exits the dungeon!", 
            character.bold()
        ),
        MetaStatus::InCombat => format!(
            "{} escapes from battle and runs out the dungeon!", 
            character.bold()
        ),
    };

    Ok(
        ret
    )
}

pub fn process_run() {
    
    match run_away() {
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