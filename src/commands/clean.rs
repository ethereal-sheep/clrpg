use crate::{errln, infoln};
use crate::utils::common::*;

use colored::Colorize;

pub fn process_clean() {
    
    infoln!("Cleaning...");

    match check_root() {
        Ok(exist) if !exist => return,
        _ => (), // can silently ignore error
    }
    
    match delete_root() {
        Ok(_)  => {
            infoln!("Deleted {}", ROOT_FOLDER_NAME);
        },
        Err(err) => errln!("{}", err)
    }
    
    println!("{}", "The dungeon has mysteriously vanished.");
}