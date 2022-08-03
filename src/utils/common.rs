use std::{path::Path, fs::{write, create_dir, remove_dir_all, read_to_string}, cell::RefCell};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const ROOT_FOLDER_NAME: &str = ".dungeon";
pub const CHAR_FOLDER_NAME: &str = ".dungeon/.characters";
pub const RAND_FILE_NAME: &str = ".dungeon/.rand";


thread_local!(static ACTIVE: RefCell<bool> = RefCell::new(false));

pub type Rng = rand_pcg::Pcg64Mcg;
pub struct RandomState {
    pub rng: Rng,
}

pub fn create_rand(seed: Option<u64>) -> Result<(), String> {
    if let Some(s) = seed {
        write_rand(&Rng::seed_from_u64(s))
    } else {
        write_rand(&Rng::from_entropy())
    }
}

pub fn write_rand(rng: &Rng) -> Result<(), String> {


    let json = match serde_json::to_string(rng) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(s) => s
    };

    match write(RAND_FILE_NAME, &json.as_bytes()) {
        Err(_) => Err(format!("Unable to write to {}", RAND_FILE_NAME)),
        _ => Ok(())
    }
}


impl RandomState {
    pub fn single_use() -> Result<Self, String> {

        ACTIVE.with(|b| {
            let mut inner = b.borrow_mut();
            if !*inner {
                *inner = true;
            } else {
                panic!("Cannot instantiate more than one RandomState!")
            }
        });

        match read_to_string(RAND_FILE_NAME) {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(rng) => {
                        Ok(RandomState {
                            rng: rng,
                        })
                    },
                    Err(err) => Err(err.to_string()),
                }
            },
            Err(err) => Err(err.to_string()),
        }
        


    }

}


impl Drop for RandomState {
    fn drop(&mut self) {
        // serialize rng      
        
        if let Err(err) = write_rand(&self.rng) {
            panic!("{}", err);
        }

        ACTIVE.with(|b| {
            let mut inner = b.borrow_mut();
            *inner = false;
        });
    }
}


/// Checks if the path exists
pub fn check_dir<T>(path: &T) -> Result<bool, String>
where 
    T: AsRef<std::ffi::OsStr> + std::fmt::Display + ?Sized {
    match Path::new(path).try_exists() {
        Err(_) => Err(format!("Unable to determine existence of {}", path)),
        Ok(t) => Ok(t)
    }
}

/// Checks if root folder exists
pub fn check_root() -> Result<bool, String> {
    check_dir(ROOT_FOLDER_NAME)
}

/// Checks if character folder exists
pub fn check_char() -> Result<bool, String> {
    check_dir(CHAR_FOLDER_NAME)
}

/// Checks if character file exists
pub fn check_character(name: &str) -> Result<bool, String> {
    check_dir(&format!("{}/{}", CHAR_FOLDER_NAME, name))
}

pub fn create_root() -> Result<(), String> {

    match create_dir(ROOT_FOLDER_NAME) {
        Err(_) => Err(format!("Unable to create {}", ROOT_FOLDER_NAME)),
        Ok(_) => {
            Ok(())
        }
    }
}

pub fn create_char() -> Result<(), String> {

    match create_dir(CHAR_FOLDER_NAME) {
        Err(_) => Err(format!("Unable to create {}", CHAR_FOLDER_NAME)),
        Ok(_) => {
            Ok(())
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterObj {
    name: String
}

pub fn create_character(name: &str) -> Result<(), String> {
    write_character(&CharacterObj{ name: name.to_string() })
}

pub fn write_character(char: &CharacterObj) -> Result<(), String> {
    let json = match serde_json::to_string(char) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(s) => s
    };

    let path = &format!("{}/{}", CHAR_FOLDER_NAME, char.name);

    match write(path, &json.as_bytes()) {
        Err(_) => Err(format!("Unable to write to {}", path)),
        _ => Ok(())
    }
}

pub fn delete_root() -> Result<(), String> {

    match remove_dir_all(ROOT_FOLDER_NAME) {
        Err(_) => Err(format!("Unable to remove {}", CHAR_FOLDER_NAME)),
        Ok(_) => {
            Ok(())
        }
        
    }
}

pub fn _delete_char() -> Result<(), String> {

    match remove_dir_all(CHAR_FOLDER_NAME) {
        Err(_) => Err(format!("Unable to remove {}", CHAR_FOLDER_NAME)),
        Ok(_) => {
            Ok(())
        }
        
    }
}


