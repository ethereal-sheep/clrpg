use crate::{errln, infoln};
use crate::utils::common::*;

use std::fs::{remove_dir_all};
use std::path::Path;
use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct Clean {
    /// Say yes
    #[clap(short, long, action)]
    yes: bool,
}



pub fn process_clean(_clean: &Clean) {
    
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
    

}