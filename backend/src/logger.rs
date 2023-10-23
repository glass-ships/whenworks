use crate::args_parser::ARGS;

#[macro_export]
macro_rules! log {
    ($lvl:expr, $($fmt:tt)*) => {
        logger($lvl, format!($($fmt)*))
    };
}

pub const DEBUG: Level = Level::Debug;
pub const INFO: Level = Level::Info;
pub const WARN: Level = Level::Warn;
pub const ERROR: Level = Level::Error;
pub const FATAL: Level = Level::Fatal;

#[derive(PartialEq)]
pub enum Level {
    Debug, // cyan
    Info,  // default, white
    Warn,  // yellow
    Error, // red
    Fatal, // red, bold
}

pub fn logger<T: std::fmt::Display>(lvl: Level, msg: T) {
    unsafe {
        if ARGS.quiet && lvl != Level::Fatal { return; }
        if lvl == Level::Debug && !ARGS.debug { return; }
    }

    match lvl {
        Level::Info  => println!("{}", msg),
        Level::Warn  => println!("\x1b[33m[WARN]:\x1b[0m {}", msg),
        Level::Error => println!("\x1b[31m[ERROR]:\x1b[0m {}", msg),
        Level::Fatal => println!("\x1b[31;1m[FATAL]:\x1b[0m {}", msg),
        Level::Debug => println!("\x1b[36m[DEBUG]:\x1b[0m {}", msg),
    }
}
