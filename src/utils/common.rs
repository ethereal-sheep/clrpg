use std::{path::Path, fs::{write, create_dir, remove_dir_all, read_to_string}, cell::RefCell, fmt::Debug};
use colored::Colorize;
use rand::{prelude::*};
use serde::{Deserialize, Serialize};

use tabled::Tabled;

use crate::{infoln, warnln};

pub const ROOT_FOLDER_NAME: &str = ".dungeon";
pub const CHAR_FOLDER_NAME: &str = ".dungeon/.characters";
pub const RAND_FILE_NAME: &str = ".dungeon/.rand";
pub const META_FILE_NAME: &str = ".dungeon/.meta";

/// Checks if the path exists
fn check_dir<T>(path: &T) -> Result<bool, String>
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

/// Checks if the path exists
fn write_to_dir<T, P>(value: &T, path: &P) -> Result<(), String>
where 
    T: Serialize, 
    P: AsRef<std::path::Path> + std::fmt::Display + ?Sized {
    let json = serde_json::to_string_pretty(&value).unwrap();
    match write(path, &json.as_bytes()) {
        Err(err) => Err(
            format!(
                "Unable to write to {}: {}", 
                path, err.to_string()
            )
        ),
        _ => Ok(())
    }
}


fn delete_dir<T>(path: &T) -> Result<(), String> 
where 
    T: AsRef<std::path::Path> + std::fmt::Display + ?Sized{
    match remove_dir_all(path) {
        Err(err) => Err(
            format!(
                "Unable to remove {}: {}", 
                path, err.to_string()
            )
        ),
        Ok(_) => {
            Ok(())
        }
        
    }
}


fn require_file_strict<T, P>(path: &P) -> Result<T, String>
where 
    T: for<'a> Deserialize<'a>, 
    P: AsRef<std::path::Path> + std::fmt::Display + ?Sized {
    match read_to_string(path) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(v) => {
                    Ok(v)
                },
                Err(err) => {
                    crate::errln!(
                        "{} is corrupted: {}", 
                        path, err.to_string()
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
                        path
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
                        path, err.to_string()
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


fn require_file<T, P>(path: &P) -> Result<T, String>
where 
    T: for<'a> Deserialize<'a>, 
    P: AsRef<std::path::Path> + std::fmt::Display + ?Sized {
    match read_to_string(path) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(v) => {
                    Ok(v)
                },
                Err(err) => {
                    crate::errln!(
                        "{} is corrupted: {}", 
                        path, err.to_string()
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
                        Err(err) => return Err(err)
                    }

                },
            }
        },
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    crate::errln!(
                        "Required {} is missing!", 
                        path
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
                        Err(err) => return Err(err)
                    }
                },
                _ => {

                    crate::errln!(
                        "Unable to read from {}: {}", 
                        path, err.to_string()
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


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaStatus {
    HelpWanted,
    OutsideTheDungeon,
    InTheDungeon,
    InCombat
}

impl std::fmt::Display for MetaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MetaStatus::HelpWanted => {
                write!(f, "{}\n{}", 
                    "* Help Wanted! *".yellow(),
                    "Looking for capable adventurers to subdue the dungeon."
                )
            },
            MetaStatus::OutsideTheDungeon => {
                write!(f, "{}\n{}", 
                    "Outside the dungeon".bold().green(),
                    "Preparing for the dive..."
                )
            },
            MetaStatus::InTheDungeon => {
                write!(f, "{}", "In the dungeon".bold().yellow())
            },
            MetaStatus::InCombat => {
                write!(f, "{}", "IN COMBAT".bold().red())
            }, 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub seed: u64,
    pub status: MetaStatus,
    pub current: Option<String>,
}

impl Meta {
    pub fn new(seed: u64) -> Self {
        Self { 
            seed, 
            status: MetaStatus::OutsideTheDungeon,
            current: None
        }
    }
}

/// Checks if meta file exists
pub fn _check_meta() -> Result<bool, String> {
    check_dir(META_FILE_NAME)
}

pub fn create_meta(meta: Meta) -> Result<(), String> {
    write_meta(&meta)
}

pub fn write_meta(meta: &Meta) -> Result<(), String> {
    write_to_dir(&meta, META_FILE_NAME)
}

pub fn require_meta() -> Result<Meta, String> {
    require_file_strict(META_FILE_NAME)
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

fn write_rand(rng: &Prng) -> Result<(), String> {
    write_to_dir(&rng, RAND_FILE_NAME)
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

        Ok(RandomState {
            rng: require_file(RAND_FILE_NAME)?,
        })

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

    pub fn generate_name(&mut self) -> Result<String, String> {

        let mut i:u32 = 0;
        const MAX_RETRIES: u32 = 100;
        while i < MAX_RETRIES {
            let adjectives = include_str!("../../res/adjectives.txt").lines();
            let animals = include_str!("../../res/animals.txt").lines();
    
            let adjective = adjectives.choose_stable(&mut self.rng).unwrap();
            let animal = animals.choose_stable(&mut self.rng).unwrap();
    
            let name = format!("{}{}", adjective, animal);
            match check_character(&name) {
                Ok(t) if !t => {
                    infoln!("Retried name generation {} times", i);
                    return Ok(name);
                },
                _ => i += 1
            }
        }

        crate::errln!("Max retries for name generation exceeded!");
        Err("Unexpected error occured.".to_string())
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
            Err(err) => return Err(err)
        }
    }
    Ok(())
}

/// Checks if character file exists
pub fn check_character(name: &str) -> Result<bool, String> {
    check_dir(&format!("{}/{}", CHAR_FOLDER_NAME, name))
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CharacterStatus {
    Healthy,
}

impl std::fmt::Display for CharacterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let mut len = 0;
        if let Some(w) = f.width() {
            len = w;
        } 

        match *self {
            CharacterStatus::Healthy => write!(f, "{}{:indent$}", "HEALTHY".green(), "",indent=len.saturating_sub(7)),
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthStat {
    max: i32,
    curr: i32,
    temp: i32,
}

impl HealthStat {
    fn new(max: i32) -> Self {
        HealthStat { max, curr: max, temp: 0 }
    }
}

impl std::fmt::Display for HealthStat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        let mut padding = String::new();
        if let Some(width) = f.width() {
            padding = String::with_capacity(width);
            for _ in 0..width {
                padding.push(' ');
            }
        }         
        
        const STEP: i32 = 5;
        let mut red = String::new();
        let mut blue = String::new();
        let mut black = String::new();

        let mut curr = 0;
        while curr < self.curr {
            red.push('♥');
            curr += STEP;
        }
        while curr < self.curr + self.temp {
            blue.push('♥');
            curr += STEP;
        }
        
        while curr < self.max {
            black.push('♡');
            curr += STEP;
        }

        
        write!(f, 
            "{padding}-----|  {}{}{}", 
            red.red(), blue.cyan(), black.black()
        )?;

        if self.temp > 0 {
            write!(f, "\n{padding}-----|  {:<3} /{} {}", 
                (self.curr + self.temp).to_string().cyan().bold(), self.max,
                format!("({}+{})", self.curr, self.temp).black()
            )
        } else {  
            write!(f, 
                "\n{padding}-----|  {:<3} /{}", 
                self.curr, self.max
            )
        }


        
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegStat {
    max: i32,
    temp: Vec<(i32, i32)>,
}

impl RegStat {
    fn new(max: i32) -> Self {
        RegStat { max, temp: vec![] }
    }
}

impl std::fmt::Display for RegStat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        let mut padding = String::new();
        if let Some(width) = f.width() {
            padding = String::with_capacity(width);
            for _ in 0..width {
                padding.push(' ');
            }
        }         
        
        if self.temp.len() == 0 {
            write!(f, "{padding}{:<3}", self.max)
        } else {
            let mut curr = self.max as i32;
            let mut mods = curr.to_string();
            for (modify, _) in &self.temp {
                mods = format!("{}{:+}", mods, modify);
                curr += modify;
            }
            if curr > self.max as i32 {
                write!(f, "{padding}{:<3} {}", 
                    curr.to_string().green(), 
                    format!("({})", mods).black()
                )
            } else if curr < self.max as i32 {
                write!(f, "{padding}{:<3} {}", 
                    curr.to_string().red(), 
                    format!("({})", mods).black()
                )
            } else {
                write!(f, "{padding}{:<3} {}", 
                    curr, 
                    format!("({})", mods).black()
                )
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterStats {
    health: HealthStat, 
    power: RegStat,
    block: RegStat,
    magic: RegStat,
    faith: RegStat,
    speed: RegStat
}


impl CharacterStats {
    pub fn from_rng(rng: &mut RandomState) -> CharacterStats {
        let c = CharacterStats {
            health: HealthStat::new(100),
            power: RegStat::new(rng.rng.gen_range(1..20)),
            block: RegStat::new(rng.rng.gen_range(1..20)),
            magic: RegStat::new(rng.rng.gen_range(1..20)),
            faith: RegStat::new(rng.rng.gen_range(1..20)),
            speed: RegStat::new(rng.rng.gen_range(1..20))
        };

        // c.power.temp.push((2, 1));

        // c.magic.temp.push((-2, 1));

        // c.faith.temp.push((1, 1));
        // c.faith.temp.push((-1, 1));

        
        // c.speed.temp.push((1, 1));
        // c.speed.temp.push((-1, 1));
        // c.speed.temp.push((2, 1));
        // c.speed.temp.push((-3, 1));
        // c.speed.temp.push((-4, 1));

        c
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, Tabled)]
pub struct CharacterObj {
    id: String,
    name: String,
    status: CharacterStatus,
    created: chrono::NaiveDateTime,

    #[tabled(skip)]
    stats: CharacterStats,
}


impl std::fmt::Display for CharacterObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        let mut len = 0;
        let mut padding = String::new();
        if let Some(width) = f.width() {
            len = width;
            padding = String::with_capacity(width);
            for _ in 0..width {
                padding.push(' ');
            }
        }

        write!(f, "{padding}{}   {} ({}) \n", self.get_life_string(), self.name.bold(), self.id)?;
                // write!(f, 
        //     "{padding}Health: {} ({})\n",
        //     health_to_hearts(self.health), self.health
        // )?;
        // write!(f, "{:>indent$}", self.stats, indent=padding.len())
        
        write!(f, "{:indent$}\n", self.stats.health, indent=len)?;
        write!(f, "{padding}POWER:  {}\n", self.stats.power)?;
        write!(f, "{padding}BLOCK:  {}\n", self.stats.block)?;
        write!(f, "{padding}MAGIC:  {}\n", self.stats.magic)?;
        write!(f, "{padding}FAITH:  {}\n", self.stats.faith)?;
        write!(f, "{padding}SPEED:  {}\n", self.stats.speed)
    }
}

impl CharacterObj {
    pub fn get_life_string(&self) -> String {
        let life = if self.is_alive() {
            format!("{}", "ALIVE".green().bold())
        } else {
            format!(" {}", "DEAD".red().bold())
        };

        life
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_alive(&self) -> bool {
        self.stats.health.curr > 0
    }
}



pub fn create_character(id: String, name: String, rng: &mut RandomState) -> Result<(), String> {
    write_character(
        &CharacterObj{ 
            id,
            name,
            status: CharacterStatus::Healthy,
            created: chrono::Local::now().naive_local(),
            stats: CharacterStats::from_rng(rng)
        }
    )
}

pub fn write_character(char: &CharacterObj) -> Result<(), String> {
    let path = &format!("{}/{}", CHAR_FOLDER_NAME, char.name);
    write_to_dir(&char, path)
}


pub fn require_character(character: &str) -> Result<CharacterObj, String> {
    let path = &format!("{}/{}", CHAR_FOLDER_NAME, character);
    require_file(&path)
}

pub fn load_character<P>(path: &P) -> Result<CharacterObj, String> 
where 
    P: AsRef<Path> + Debug{

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
                // report failures but do not fail
                match load_character(&entry.path()) {
                    Ok(obj) => characters.push(obj),
                    Err(err) => warnln!("{}", err.to_string()),
                }
            }
        }
    }
    characters
}


pub fn delete_root() -> Result<(), String> {
    delete_dir(ROOT_FOLDER_NAME)
}

pub fn _delete_char() -> Result<(), String> {
    delete_dir(CHAR_FOLDER_NAME)
}


