#[macro_export]
macro_rules! errln {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        std::io::_print(std::format_args_nl!("{:>8} {}", "Error".red().bold(), std::format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {{
        std::io::_print(std::format_args_nl!("{:>8} {}", "Info".green().bold(), std::format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => {{
        std::io::_print(std::format_args_nl!("{:>8} {}", "Warning".yellow().bold(), std::format_args!($($arg)*)));
    }};
}


