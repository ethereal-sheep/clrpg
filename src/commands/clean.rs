use crate::{errln, infoln, warnln};
use crate::utils::common::*;

use colored::Colorize;

pub fn process_clean() {
    

    match check_root() {
        Ok(exist) if !exist => {
            warnln!("Nothing to clean up");
            return;
        },
        _ => (), // can silently ignore error
    }
    
    infoln!("Cleaning...");
    match delete_root() {
        Ok(_)  => {
            infoln!("Deleted {}", ROOT_FOLDER_NAME);
        },
        Err(err) =>  {
            errln!("{}", err);
            errln!("Clean failed.");
            return;
        }
    }
    
    infoln!("Clean succeeded.");
    println!("{}", "The dungeon has mysteriously vanished.");
}