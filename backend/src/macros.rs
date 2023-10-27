use crate::logger::{logger, DEBUG};
use crate::log;

#[macro_export]
macro_rules! resp {
    ($stream:ident, $($resp:expr),*) => {
        $stream.write_all({
            let responce = fmt!($($resp),*);
            log!(DEBUG, "{}", responce);
            responce
        }.as_bytes()).unwrap_or_else(|e| log!(ERROR, "{e}"))
    };
}

pub enum Type {
    Html,
    Json,
}

#[macro_export]
macro_rules! fmt {
    ($code:expr, $type:expr, $content:expr) => {
        format!("{HTTP} {} {}\r\nContent-Type: {}\r\n{POLICY}\r\nContent-Length: {}\r\n\r\n{}",
        $code, match $code {
            200 => "OK",
            400 => "BAD REQUEST",
            403 => "FORBIDDEN",
            404 => "NOT FOUND",
            409 => "CONFLICT",
            422 => "UNPROCESSABLE ENTITY",
            500 => "INTERNAL SERVER ERROR",
            _   => "UNHANDLED",
        }, match $type {
            Type::Html  => "text/html",
            Type::Json  => "application/json",
        }, $content.len(), $content)
    };
    ($code:expr, $content:expr) => {
        format!("{HTTP} {} {}\r\nContent-Type: text/plain\r\n{POLICY}\r\nContent-Length: {}\r\n\r\n{}",
        $code, match $code {
            200 => "OK",
            400 => "BAD REQUEST",
            403 => "FORBIDDEN",
            404 => "NOT FOUND",
            409 => "CONFLICT",
            422 => "UNPROCESSABLE ENTITY",
            500 => "INTERNAL SERVER ERROR",
            _   => "UNHANDLED",
        }, $content.len(), $content)
    };
    ($code:expr) => {
        format!("{HTTP} {} {}\r\n\r\n", $code, match $code {
            200 => "OK",
            400 => "BAD REQUEST",
            403 => "FORBIDDEN",
            404 => "NOT FOUND",
            409 => "CONFLICT",
            422 => "UNPROCESSABLE ENTITY",
            500 => "INTERNAL SERVER ERROR",
            _   => "UNHANDLED",
        })
    };
}
