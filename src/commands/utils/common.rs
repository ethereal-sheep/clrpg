use colored::Colorize;
use std::io::stdout;
use std::time::Duration;
use std::thread::sleep;
use std::io::Write;

pub const ROOT_FOLDER_NAME: &str = ".dungeon";
pub const CHAR_LIST_NAME: &str = ".dungeon/characters.txt";


pub fn print_logo() {
    
    println!();
    include_str!("../../../logo.txt").lines().for_each(|l| println!("{}", l));
    println!();

}

pub enum NarrateSpeed {
    Slow,
    Fast,
    Norm
}

pub fn narrate(s: &str, speed: NarrateSpeed, wrapping_len: usize) {

    let sleep_time = match speed {
        NarrateSpeed::Slow => 80,
        NarrateSpeed::Norm => 40,
        NarrateSpeed::Fast => 20,
    };

    print!("\n{:>8} ", "Narrate".cyan().bold());

    s.split(' ')    
    .fold(0, |mut count, s| {

        if count + s.len() > wrapping_len {
            count = 0;
            print!("\n{:>8} ", "");
        }

        print!("{} ", s);
        let _ = stdout().flush();
        sleep(Duration::from_millis(sleep_time));
        count + s.len()

    });
    print!("\n\n");
}