use crate::args_parser::ARGS;

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        logger(Level::Info, $msg)
    };
    ($lvl:expr, $msg:expr) => {
        logger($lvl, $msg)
    };
}

#[derive(PartialEq)]
pub enum Level {
    Info,  // default, white
    Warn,  // yellow
    Error, // red
    Fatal, // red, bold
    Debug, // cyan
}

pub fn logger<T: std::fmt::Display>(lvl: Level, msg: T) {
    unsafe {
        if ARGS.quiet && lvl != Level::Fatal { return; }
        if lvl == Level::Debug && !ARGS.debug { return; }
    }

    match lvl {
        Level::Info  => println!("{}", msg),
        Level::Warn  => println!("\x1b[33mWARN:\x1b[0m {}", msg),
        Level::Error => println!("\x1b[31mERROR:\x1b[0m {}", msg),
        Level::Fatal => println!("\x1b[31;1mFATAL:\x1b[0m {}", msg),
        Level::Debug => println!("\x1b[36mDEBUG:\x1b[0m {}", msg),
    }
}
