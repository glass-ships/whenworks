#[macro_export]
macro_rules! status {
    ($code:expr) => {
        format!("{HTTP} {code}\r\n\r\n", code=$code).as_bytes()
    };
}

#[macro_export]
macro_rules! file {
    ($contents:expr) => {
        format!("{HTTP} 200 OK\r\nContent-Length: {len}\r\n\r\n{contents}", len=$contents.len(), contents=$contents).as_bytes()
    };
}

#[macro_export]
macro_rules! json {
    ($contents:expr) => {
        format!("{HTTP} 200 OK\r\nContent-Length: {len}\r\nContent-Type: application/json\r\n\r\n{contents}", len=$contents.len(), contents=$contents).as_bytes()
    };
}

