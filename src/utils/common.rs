use std::{path::Path, fs::{write, create_dir, remove_dir_all, read_to_string}, cell::RefCell};
use colored::Colorize;
use rand::{prelude::*};
use serde::{Deserialize, Serialize};

use tabled::Tabled;

pub const ROOT_FOLDER_NAME: &str = ".dungeon";
pub const CHAR_FOLDER_NAME: &str = ".dungeon/.characters";
pub const RAND_FILE_NAME: &str = ".dungeon/.rand";
pub const META_FILE_NAME: &str = ".dungeon/.meta";

/// Checks if the path exists
pub fn check_dir<T>(path: &T) -> Result<bool, String>
where 
    T: AsRef<std::ffi::OsStr> + std::fmt::Display + ?Sized {
    match Path::new(path).try_exists() {
        Err(err) => Err(
            format!(
                "Unable to determine existence of {}: {}", 
                path, err.to_string()
            )
        ),
        Ok(t) => Ok(t)
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub seed: u64,
    
}

/// Checks if meta file exists
pub fn _check_meta() -> Result<bool, String> {
    check_dir(META_FILE_NAME)
}

pub fn create_meta(meta: Meta) -> Result<(), String> {
    write_meta(&meta)
}

pub fn write_meta(meta: &Meta) -> Result<(), String> {
    let json = serde_json::to_string_pretty(meta).unwrap();

    match write(META_FILE_NAME, &json.as_bytes()) {
        Err(err) => Err(
            format!(
                "Unable to write to {}: {}", 
                META_FILE_NAME, err.to_string()
            )
        ),
        _ => Ok(())
    }
}

pub fn require_meta() -> Result<Meta, String> {
    match read_to_string(META_FILE_NAME) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(meta) => {
                    Ok(meta)
                },
                Err(err) => {
                    crate::errln!(
                        "{} is corrupted: {}", 
                        META_FILE_NAME, err.to_string()
                    );
                    return Err(
                        format!(
                            "The dungeon is corrupted! {}{} {} {}{}", 
                            "(use \"".white(),
                            "clrpg".yellow(), 
                            "init".black(),
                            "--force".black(),
                            "\" to create a new dungeon)".white()
                        )
                    );

                },
            }
        },
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    crate::errln!(
                        "Required {} is missing!", 
                        META_FILE_NAME
                    );

                    return Err(
                        format!(
                            "The dungeon is corrupted! {}{} {} {}{}", 
                            "(use \"".white(),
                            "clrpg".yellow(), 
                            "init".black(),
                            "--force".black(),
                            "\" to create a new dungeon)".white()
                        )
                    );
                },
                _ => {

                    crate::errln!(
                        "Unable to read from {}: {}", 
                        META_FILE_NAME, err.to_string()
                    );

                    return Err(
                        format!(
                            "{}", 
                            "Unexpected error occured."
                        )
                    );
                }
            }
        },
    }
}



thread_local!(static ACTIVE: RefCell<bool> = RefCell::new(false));

pub type Prng = rand_pcg::Pcg64Mcg;
pub struct RandomState {
    pub rng: Prng,
}

pub fn create_rand(seed: Option<u64>) -> Result<u64, String> {
    if let Some(s) = seed {
        write_rand(&Prng::seed_from_u64(s))?;
        Ok(s)
    } else {
        let s: u64 = thread_rng().gen();
        write_rand(&Prng::seed_from_u64(s))?;
        Ok(s)
    }
}

pub fn write_rand(rng: &Prng) -> Result<(), String> {
    let json = serde_json::to_string_pretty(rng).unwrap();
    match write(RAND_FILE_NAME, &json.as_bytes()) {
        Err(err) => Err(
            format!(
                "Unable to write to {}: {}", 
                RAND_FILE_NAME, err.to_string()
            )
        ),
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


    pub fn generate_id(&mut self) -> String {
        // just randomly select 8 bytes of our alphabet and return
        // can be decoded to find a seed that produces the same result
        // but who cares 🤷
        const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
        const ID_SIZE: usize = 8;

        let mut ret = String::with_capacity(ID_SIZE);
        for _ in 0..ID_SIZE {
            let u = ALPHABET.as_bytes().choose(&mut self.rng).unwrap();
            ret.push(*u as char);
        }
        ret
    }

    pub fn generate_name(&mut self) -> String {

        loop {
            let adjectives = include_str!("../../res/adjectives.txt").lines();
            let animals = include_str!("../../res/animals.txt").lines();
    
            let adjective = adjectives.choose_stable(&mut self.rng).unwrap();
            let animal = animals.choose_stable(&mut self.rng).unwrap();
    
            let name = format!("{}{}", adjective, animal);
            match check_character(&name) {
                Ok(t) if !t => return format!("{}{}", adjective, animal),
                _ => ()
            }
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



pub fn create_root() -> Result<(), String> {

    match create_dir(ROOT_FOLDER_NAME) {
        Err(err) => Err(
            format!(
                "Unable to create {}: {}", 
                ROOT_FOLDER_NAME, err.to_string()
            )
        ),
        Ok(_) => {
            Ok(())
        }
    }
}

/// Checks if root folder exists
pub fn check_root() -> Result<bool, String> {
    check_dir(ROOT_FOLDER_NAME)
}

pub fn require_root() -> Result<(), String> {
    if !check_root()? {
        crate::errln!(
            "Missing {}! Please run command: [ {} {} ]", 
            ROOT_FOLDER_NAME, 
            "clrpg".yellow(), 
            "init".black()
        );

        return Err(
            format!(
                "There is no dungeon! {}{} {}{}", 
                "(use \"".white(),
                "clrpg".yellow(), 
                "init".black(),
                "\" to create the dungeon)".white()
            )
        );
    }
    Ok(())
}


pub fn create_char() -> Result<(), String> {

    match create_dir(CHAR_FOLDER_NAME) {
        Err(err) => Err(
            format!(
                "Unable to create {}: {}", 
                CHAR_FOLDER_NAME, err.to_string()
            )
        ),
        Ok(_) => {
            Ok(())
        }
    }
}

/// Checks if character folder exists
pub fn check_char() -> Result<bool, String> {
    check_dir(CHAR_FOLDER_NAME)
}

pub fn require_char() -> Result<(), String> {
    if !check_char()? {

        crate::errln!(
            "Required file {} is missing!", 
            CHAR_FOLDER_NAME
        );

        // check if meta is valid
        match require_meta() {
            Ok(_) => {
                return Err(
                    format!(
                        "The dungeon is corrupted!\n   {}{} {}{}\n   {}{} {} {}{}", 
                        "(use \"".white(),
                        "clrpg".yellow(), 
                        "reset".black(),
                        "\" to create a new dungeon with the current seed); or".white(),
                        "(use \"".white(),
                        "clrpg".yellow(), 
                        "init".black(),
                        "--force".black(),
                        "\" to create a new dungeon)".white()
                    )
                );
            },
            Err(err) => {
                crate::errln!("{}", err);
                return Err(
                    format!(
                        "The dungeon is corrupted! {}{} {} {}{}",
                        "(use \"".white(),
                        "clrpg".yellow(), 
                        "init".black(),
                        "--force".black(),
                        "\" to create a new dungeon)".white()
                    )
                );
            }
        }
    }
    Ok(())
}

/// Checks if character file exists
pub fn check_character(name: &str) -> Result<bool, String> {
    check_dir(&format!("{}/{}", CHAR_FOLDER_NAME, name))
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CharacterStatus {
    Alive,
    Dead
}

impl std::fmt::Display for CharacterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CharacterStatus::Alive => write!(f, "{}", "ALIVE".green()),
            CharacterStatus::Dead => write!(f, "{}", "DEAD".red()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Tabled)]
pub struct CharacterObj {
    id: String,
    name: String,
    status: CharacterStatus,
    created: chrono::NaiveDateTime,
}

pub fn create_character(id: String, name: String) -> Result<(), String> {
    write_character(
        &CharacterObj{ 
            id,
            name,
            status: CharacterStatus::Alive,
            created: chrono::Local::now().naive_local(),
        }
    )
}

pub fn write_character(char: &CharacterObj) -> Result<(), String> {
    let json = serde_json::to_string_pretty(char).unwrap();
    let path = &format!("{}/{}", CHAR_FOLDER_NAME, char.name);

    match write(path, &json.as_bytes()) {
        Err(_) => Err(format!("Unable to write to {}", path)),
        _ => Ok(())
    }
}


pub fn load_character(path: &Path) -> Result<CharacterObj, String> {

    match read_to_string(path) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(meta) => {
                    Ok(meta)
                },
                Err(err) => Err(
                    format!(
                        "{:?} is corrupted: {}", 
                        path, err.to_string()
                    )
                ),
            }
        },
        Err(err) => Err(
            format!(
                "Unable to read from {:?}: {}", 
                path, err.to_string()
            )
        ),
    }
}

pub fn load_characters() -> Vec<CharacterObj> {
    let mut characters: Vec<CharacterObj> = vec![];

    if let Ok(entries) = std::fs::read_dir(CHAR_FOLDER_NAME) {
        for res in entries {
            if let Ok(entry) = res {
                // silently ignore failures
                if let Ok(obj) = load_character(&entry.path()) {
                    characters.push(obj);
                }
            }
        }
    }
    characters
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


