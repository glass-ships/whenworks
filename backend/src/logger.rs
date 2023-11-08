use crate::args_parser::ARGS;

#[macro_export]
macro_rules! log {
    (DEBUG, $($fmt:tt)*) => {
        crate::logger::logger(crate::logger::Level::Debug, format!($($fmt)*))
    };
    (WARN, $($fmt:tt)*) => {
        crate::logger::logger(crate::logger::Level::Warn, format!($($fmt)*))
    };
    (ERROR, $($fmt:tt)*) => {
        crate::logger::logger(crate::logger::Level::Error, format!($($fmt)*))
    };
    (FATAL, $($fmt:tt)*) => {
        crate::logger::fatal(format!($($fmt)*))
    };
    ($($fmt:tt)*) => {
        crate::logger::logger(crate::logger::Level::Info, format!($($fmt)*))
    };
}

#[derive(PartialEq)]
pub enum Level {
    Debug, // cyan
    Info,  // default, white
    Warn,  // yellow
    Error, // red
    Fatal, // red, bold
}

pub fn fatal(msg: String) {
    logger(Level::Fatal, msg);
    std::process::exit(1)
}

pub fn logger(lvl: Level, msg: String) {
    if lvl == Level::Debug && unsafe{!ARGS.debug} { return; }

    match lvl {
        Level::Info  => println!("{}", msg),
        Level::Warn  => println!("\x1b[33m[WARN]:\x1b[0m {}", msg),
        Level::Error => println!("\x1b[31m[ERROR]:\x1b[0m {}", msg),
        Level::Fatal => println!("\x1b[31;1m[FATAL]:\x1b[0m {}", msg),
        Level::Debug => println!("\x1b[36m[DEBUG]:\x1b[0m {}", msg),
    }
}
