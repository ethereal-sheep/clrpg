use crate::{errln, infoln};
use super::utils::common::*;

use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use clap::{AppSettings, Parser, Subcommand, Args};
use colored::Colorize;

#[derive(Args)]
pub struct Clean {
    /// Say yes
    #[clap(short, long, action)]
    yes: bool,
}


fn delete_root() -> Result<bool, String> {

    let result = Path::new(ROOT_FOLDER_NAME).try_exists();

    match &result {
        Err(_) => Err(format!("Unable to determine existence of {}", ROOT_FOLDER_NAME)),
        Ok(t) if *t => {
            remove_dir_all(ROOT_FOLDER_NAME).unwrap();
            Ok(true)
        },
        Ok(_) => Ok(false)
        
    }
}


pub fn process_clean(_clean: &Clean) {
    
    infoln!("Cleaning...");

    match delete_root() {
        Ok(t) if t => {
            infoln!("Deleted {}", ROOT_FOLDER_NAME);
        },
        Err(err) => errln!("{}", err),
        _ => ()
    }
    

}