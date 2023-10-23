use super::*;

use libc::{c_uint, srand, rand, time};

use std::ptr::null_mut;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::Mutex;
use std::process::exit;

use crate::args_parser::ARGS;
use crate::log;

use serde::{Deserialize, Serialize};
use flate2::{
    Compression,
    read::GzDecoder,
    write::ZlibEncoder,
};

pub static mut EVENT_LIST: Option<Mutex<EventList>> = None;

const ALPHANUMERIC: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z', 
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

unsafe fn gen_hash() -> Hash {
    // seed the rand
    srand(time(null_mut()) as c_uint);

    let mut buf = ['\0'; 6];
    (0..6).for_each(|i| buf[i] = ALPHANUMERIC[(rand() % 62) as usize]);
    buf
}

pub type Hash = [char; 6];
type EventList = HashMap<Hash, Event>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(skip)]
    pub edit_hash: Hash,
    pub name: String,
    pub desc: Option<String>,
    creation_date: u64,
    dates: Vec<DateRange>,
    users: HashMap<String, User>,
}

// TODO delete user/event
#[derive(Debug, Deserialize)]
pub struct EventEntry {
    pub name: String,
    pub desc: Option<String>,
    pub dates: Vec<DateRange>,
    pub deleted_users: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DateRange {
    from: u64, 
    to: u64, 
    preferred: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(skip)]
    pass: [char; 8],
    comment: Option<String>,
    avail_dates: Vec<DateRange>,
}

#[derive(Debug, Deserialize)]
pub struct UserEntry {
    pub pass: [char; 8],
    pub name: String,
    pub comment: Option<String>,
    pub avail_dates: Vec<DateRange>,
}

pub unsafe fn load_db() { 
    let mut db = match std::path::Path::new(ARGS.db_file).exists() {
        true => {
            std::fs::File::open(ARGS.db_file).unwrap_or_else(|e| {
                log!(ERROR, "{}", e);
                exit(1);
            })
        },
        false => {
            log!(INFO, "db file not found, creating...");
            std::fs::File::create(ARGS.db_file).unwrap_or_else(|e| {
                log!(ERROR, "{}", e);
                exit(1);
            });
            std::fs::File::open(ARGS.db_file).unwrap_or_else(|e| {
                log!(ERROR, "{}", e);
                exit(1);
            })
        },
    };

    let mut buf = Vec::new();
    if let Err(e) = db.read_to_end(&mut buf) {
        log!(ERROR, "{}", e);
        exit(1);
    };

    if buf.is_empty() {
        EVENT_LIST = Some(Mutex::new(HashMap::new()));
        return;
    }

    // decompress
    // let mut decomp = GzDecoder::new(&*buf);
    // let mut buf = Vec::new();
    // decomp.read_to_end(&mut buf).unwrap();

    // decode bincode
    let db_de: EventList = bincode::deserialize(&buf).unwrap();

    EVENT_LIST = Some(Mutex::new(db_de));
}

// TODO: have this only happen every X time, and have a graceful shutdown,
// we already have a cache in mem for the entirety of this so its cool to wait
pub unsafe fn store_db(db: &EventList) {
    // encode bincode
    let db_ser = match bincode::serialize(db) {
        Ok(db_ser) => db_ser,
        Err(e) => {
            log!(ERROR, "failed to serialize db {}", e);
            exit(1);
        },
    };

    // compress
    // let mut comp = ZlibEncoder::new(Vec::new(), Compression::default());
    // comp.write_all(&db_ser).unwrap();

    // std::fs::write(ARGS.db_file, comp.finish().unwrap()).unwrap();
    std::fs::write(ARGS.db_file, db_ser).unwrap();
}

impl Event {
    fn from_entry(
        edit_hash: [char; 6], 
        creation_date: u64, 
        event: EventEntry, 
        users: HashMap<String, User>
    ) -> Self {
        Self { 
            edit_hash,
            name: event.name,
            desc: event.desc,
            creation_date,
            dates: event.dates,
            users,
        }
    }

    pub fn add(mut event: Event) -> (Hash, Hash) {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let mut event_uid = unsafe{ gen_hash() };

        // make sure its unique
        while db.contains_key(&event_uid) {
            event_uid = unsafe{ gen_hash() };
        }

        // add event and edit hash to db
        let edit_hash = unsafe{ gen_hash() };
        event.edit_hash = edit_hash;

        db.insert(event_uid, event);
        unsafe{ store_db(&db) };

        (event_uid, edit_hash)
    }

    pub fn edit(event_id: Hash, edit: [char; 6], new_event: EventEntry){
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };
        // TODO: dissalow changing creation date and users

        let Some(event) = db.get(&event_id) else {
            return;
        };

        let creation_date = event.creation_date;
        let users = event.users.clone();   // FIXME this is pretty bad

        db.insert(event_id, Event::from_entry(edit, creation_date, new_event, users));
        unsafe{ store_db(&db) };
    }

    // pub fn delete(event_id: Hash) {
    //     let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };
    //
    //     db.remove(&event_id);
    //     unsafe{ store_db(&db) };
    // }
    //
    pub fn add_user<'a>(event_id: Hash, user: UserEntry) -> Result<(), (u16, &'a str)> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err((404, "event doesn not exist"));
        };

        // make sure user doesnt already exist
        if event.users.contains_key(&user.name) {
            return Err((409, "user already exists"));
        }

        event.users.insert(user.name.clone(), User::from_entry(user));

        unsafe{ store_db(&db) };

        Ok(())
    }

    pub fn edit_user<'a>(event_id: Hash, new_user: UserEntry) -> Result<(), (u16, &'a str)> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err((404, "event doesn not exist"));
        };

        // make sure user exists
        let Some(user) = event.users.get(&new_user.name) else {
            return Err((404, "user does not exist"));
        };

        if user.pass != new_user.pass {
            return Err((403, "incorrect password"));
        }

        event.users.insert(new_user.name.clone(), User::from_entry(new_user));

        unsafe{ store_db(&db) };

        Ok(())
    }

    pub fn delete_user<'a>(event_id: Hash, user_name: &str) -> Result<(), (u16, &'a str)> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err((404, "event doesn not exist"));
        };

        // make sure user exists
        if !event.users.contains_key(user_name) {
            return Err((404, "user does not exist"));
        }

        event.users.remove(user_name);

        unsafe{ store_db(&db) };

        Ok(())
    }
    
    pub fn delete_user_en<'a>(event_id: Hash, user_name: &str, pass: [char; 8]) -> Result<(), (u16, &'a str)> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err((404, "event doesn not exist"));
        };

        // make sure user exists
        let Some(user) = event.users.get(user_name) else {
            return Err((404, "user does not exist"));
        };

        if pass != user.pass {
            return Err((403, "incorrect password"));
        }

        event.users.remove(user_name);

        unsafe{ store_db(&db) };

        Ok(())
    }
}

impl User {
    fn from_entry(user: UserEntry) -> Self {
        Self { pass: user.pass, comment: user.comment, avail_dates: user.avail_dates }
    }
}

