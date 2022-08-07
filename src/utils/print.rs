
use clap::ArgEnum;
use colored::Colorize;
use std::cell::RefCell;
use std::io::stdout;
use std::time::Duration;
use std::thread::sleep;
use std::io::Write;

thread_local! { pub static VERBOSE: RefCell<bool> = RefCell::new(true); }

#[macro_export]
macro_rules! errln {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        crate::utils::print::VERBOSE.with(|b| {
            let inner = b.borrow_mut();
            if *inner {
                std::io::_print(std::format_args_nl!("{:>8} {}", "Error".red().bold(), std::format_args!($($arg)*)));
            }
        });
    }};
}

#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {{
        crate::utils::print::VERBOSE.with(|b| {
            let inner = b.borrow_mut();
            if *inner {
                std::io::_print(std::format_args_nl!("{:>8} {}", "Info".green().bold(), std::format_args!($($arg)*)));
            }
        });
    }};
}

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => {{
        crate::utils::print::VERBOSE.with(|b| {
            let inner = b.borrow_mut();
            if *inner {
                std::io::_print(std::format_args_nl!("{:>8} {}", "Warn".yellow().bold(), std::format_args!($($arg)*)));
            }
        });
    }};
}


pub fn print_logo() {
    
    include_str!("../../res/logo.txt").lines().for_each(|l| println!("{}", l));

}

#[derive(Debug, Clone, ArgEnum)]
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

