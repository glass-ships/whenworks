use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use std::process::exit;

use serde::{Serialize, Deserialize};

use threads::ThreadPool;
use database::{load_db, Event, EVENT_LIST, Hash, UserEntry, EventEntry};
use macros::Type;
use logger::{logger, FATAL, WARN, ERROR, DEBUG, INFO};
use args_parser::ARGS;

mod threads;
mod macros;
mod database;
mod logger;
mod args_parser;

const HTTP: &str = "HTTP/1.1";
const POLICY: &str = "Access-Control-Allow-Origin: *";

fn main() {
    args_parser::init_args();

    let Ok(listener) = TcpListener::bind(unsafe{ARGS.addr}) else {
        log!(FATAL, "Failed to bind to address");
        exit(1);
    };

    let pool = ThreadPool::new(4);

    unsafe{ load_db(); }

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            log!(ERROR, "Failed to accept connection");
            continue;
        };
        pool.execute(|| handle_conn(stream));
    }
}


fn handle_conn(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let bytes_ = match stream.read(&mut buf) {
        Ok(b) if b == 0 => return,
        Ok(b) => b,
        Err(e) => {
            log!(ERROR, "{e}");
            return;
        },
    };

    let Ok(req) = str::from_utf8(&buf[..bytes_]) else {
        resp!(stream, 400, "invalid utf8");
        return;
    };
    let (head, body) = req.split_once("\r\n\r\n").unwrap_or((req, ""));

    log!(DEBUG, "\nHead: \n{head}\n\nBody: \n{body}");

    let type_ = head.lines().next().unwrap().split_whitespace().collect::<Vec<&str>>();
    // example: GET / HTTP/1.1_
    // [0] = GET
    // [1] = /
    // [2] = HTTP/1.1

    match type_[0] {
        "GET" => handle_get(type_[1], &mut stream),
        "POST" => handle_post(type_[1], body, &mut stream),
        _ => {
            log!(WARN, "Recieved unhandled request type: `{}`", type_[0]);
            resp!(stream, 400, "Unhandled Request Type");
        },
    }

    stream.flush().unwrap();
}

fn handle_get(arg: &str, stream: &mut TcpStream) {
    // handle favicon
    if arg == "/favicon.ico" { 
        let Ok(mut icon) = std::fs::File::open("favicon.ico") else {
            resp!(stream, 500, "failed to read favicon");
            log!(ERROR, "favicon.ico not found");
            return;
        };
        let mut buf = Vec::new();
        icon.read_to_end(&mut buf).unwrap();

        let resp = format!("{HTTP} 200 OK\r\nContent-Type: image/x-icon\r\n{POLICY}\r\nContent-Length: {}\r\n\r\n", buf.len());
        let resp = &[resp.as_bytes(), &buf].concat();
        stream.write_all(resp).unwrap_or_else(|e| log!(ERROR, "{e}"))
    }
    
    let Some(arg) = arg.strip_prefix("/api/") else {
        // handle root
        let Ok(file) = std::fs::read_to_string(unsafe{ARGS.index_file}) else {
            log!(ERROR, "index file not found");
            resp!(stream, 500, "failed to read root file");
            return;
        };
        resp!(stream, 200, Type::Html, file);
        return;
    };
    
    // parse arg into event id
    if arg.len() != 6 {
        resp!(stream, 400, format!("invalid event id length. expected 6, got {}", arg.len()));
        return;
    }

    let db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };
    let Some(ref event_id) = str_to_hash(arg) else {
        resp!(stream, 400, "event id is not a valid hash");
        return;
    };
    let Some(event) = db.get(event_id) else {
        resp!(stream, 404, "event does not exist");
        return;
    };

    let Ok(json_event) = serde_json::to_string(event) else {
        resp!(stream, 500, "failed to serialize event");
        return;
    };

    dbg!(&json_event);
    
    resp!(stream, 200, Type::Json, json_event);
}

#[derive(Serialize)]
struct Hashes {
    event_id: String,
    edit_hash: String,
}

#[derive(Deserialize)]
struct Boiled {
    pass: [char; 8],
    name: String,
}

fn handle_post(arg: &str, body: &str, stream: &mut TcpStream) {
    let Some(arg) = arg.strip_prefix("/api/") else {
        resp!(stream, 400, "no op");
        return;
    };

    //
    // create a new event
    if arg == "new" {
        if body.is_empty() {
            resp!(stream, 422, "missing body");
            return;
        }

        let Ok(event) = serde_json::from_str::<Event>(body) else {
            resp!(stream, 422, "invalid json in request body");
            return;
        };

        if event.name.len() > 32 {
            resp!(stream, 400, format!("event name too long. max 32, current {}", event.name.len()));
            return;
        }

        let hashes = Event::add(event);

        let Ok(response) = serde_json::to_string(&Hashes {
            event_id: hash_to_str(hashes.0),
            edit_hash: hash_to_str(hashes.1),
        }) else {
            resp!(stream, 500, "failed to serialize response");
            return;
        };

        resp!(stream, 200, Type::Json, response);
        return;
    }

    if arg.ends_with("/usr") && !arg.starts_with("//usr"){
        let arg_ = match arg.ends_with("?e") {
            true => &arg[1..arg.len()-2],
            false => &arg[1..],
        };

        let Some((event_id, _)) = arg_.split_once('/') else {
            resp!(stream, 400, "missing event id");
            return;
        };

        let Some(event_id) = str_to_hash(event_id) else {
            resp!(stream, 400, "event id is not a valid hash");
            return;
        };

        if body.is_empty() {
            resp!(stream, 422, "missing body");
            return;
        }

        if arg.ends_with("?d") {
            let Ok(user) = serde_json::from_str::<Boiled>(body) else {
                resp!(stream, 422, "invalid json in request body");
                return;
            };

            if let Err((c, r)) = Event::delete_user_en(event_id, &user.name, user.pass) {
                resp!(stream, c, r);
                return;
            } return;
        }

        let Ok(user) = serde_json::from_str::<UserEntry>(body) else {
            resp!(stream, 422, "invalid json in request body");
            return;
        };

        if user.name.len() > 32 {
            resp!(stream, 400, format!("user name too long. max 32, current {}", user.name.len()));
            return;
        }

        if arg.ends_with("?e") {
            if let Err((c, r)) =  Event::edit_user(event_id, user) {
                resp!(stream, c, r);
                return;
            }
        }

        else if let Err((c, r)) = Event::add_user(event_id, user) {
            resp!(stream, c, r);
            return;
        }

        resp!(stream, 200);
        return;
    }

    //
    // edit an event
    let Some((event_id, edit_hash)) = arg[1..].split_once('?') else {
        resp!(stream, 400, "missing edit hash");
        return;
    };

    let Some(event_id) = str_to_hash(event_id) else {
        resp!(stream, 400, "event id is not a valid hash");
        return;
    };

    let Some(edit_hash) = str_to_hash(edit_hash) else {
        resp!(stream, 400, "edit hash is not a valid hash");
        return;
    };

    if !validate_key(event_id, edit_hash, stream) { return; }

    if body.is_empty() {
        let db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };
        match db.get(&event_id) {
            Some(e) => {
                let Ok(json_event) = serde_json::to_string(e) else {
                    resp!(stream, 500, "failed to serialize event");
                    return;
                };
                resp!(stream, 200, Type::Json, json_event);
                return;
            },
            None => {
                resp!(stream, 404, "event does not exist");
                return;
            }
        };
    }

    let Ok(new_event) = serde_json::from_str::<EventEntry>(body) else {
        resp!(stream, 422, "invalid json in request body");
        return;
    };
    
    if new_event.name.len() > 32  {
        resp!(stream, 400, format!("event name too long. max 32, current {}", new_event.name.len()));   
        return;
    }

    if new_event.desc.as_ref().map(|d| d.len() > 256).unwrap_or(false) {
        resp!(stream, 400, format!("event description too long. max 256, current {}", new_event.desc.as_ref().unwrap().len()));
        return;
    }

    if let Some(ref del_usr) = new_event.deleted_users {
        for user in del_usr {
            if let Err((c, r)) = Event::delete_user(event_id, user) {
                resp!(stream, c, r);
                return;
            }
        }
    }

    Event::edit(event_id, edit_hash, new_event);

    resp!(stream, 200);
}

fn validate_key(event_id: Hash, edit_hash: Hash, stream: &mut TcpStream) -> bool {
    let db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

    match db.get(&event_id) {
        Some(e) if e.edit_hash != edit_hash => {
            resp!(stream, 403, "invalid edit hash");
            false
        },
        None => {
            resp!(stream, 404, "event does not exist");
            false
        },
        _ => true,
    }
}

fn str_to_hash(s: &str) -> Option<Hash> {
    if s.len() != 6 { return None; }

    let mut hash: Hash = ['\0'; 6];
    let mut s = s.chars();
    (0..6).for_each(|i| hash[i] = s.next().unwrap());
    Some(hash)
}

fn hash_to_str(hash: Hash) -> String {
    hash.iter().collect::<String>()
}

