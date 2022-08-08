use crate::{errln, infoln};
use crate::utils::common::*;

use colored::Colorize;

fn print_status() -> Result<(), String> {
    let meta = require_meta()?;
    
    let char_opt = match &meta.current {
        Some(character) => {
            Some(require_character(&character)?)
        },
        None => None,
    };

    infoln!("{}", "Status succeeded.");
    // infoln!("{:?}", &meta);
    // println!("Seed: {}", meta.seed);
    println!("{}", meta.status);

    println!();

    match char_opt {
        Some(character) => {
            println!("Current Adventurer");
            println!("{:>3}", character);
        },
        None => {
            println!("{}", "No waiting adventurer".red());
            println!(
                "   (use \"{} {}\" to wait an adventurer)", 
                "clrpg".yellow(), "character wait <NAME>".black()
            );
        },
    }


    Ok(())
}

pub fn process_status() {
    match print_status() {
        Ok(_) => {
        },
        Err(err) => {
            errln!("{}", "Status failed.");
            println!("{}", err.red());
        }
    }
}