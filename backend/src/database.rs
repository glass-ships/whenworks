use std::io::Read;
use std::collections::HashMap;
use std::sync::{Mutex, Once};

use serde::{Deserialize, Serialize};
use rand::Rng;

pub static mut EVENT_LIST: Option<Mutex<EventList>> = None;
static INIT_EVENT: Once = Once::new();

// default db file
static mut DB_FILE: &str = "db.bin";

const ALPHANUMARIC: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z', 
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub type Hash = [char; 6];
type EventList = HashMap<Hash, Event>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(skip)]
    pub edit_hash: Hash,
    name: String,
    desc: Option<String>,
    date: Vec<DateRange>,
    users: HashMap<String, User>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DateRange(u64, u64);

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(skip)]
    pass: u8,
    comment: Option<String>,
    aval_dates: Vec<DateRange>,
    pref_dates: Option<Vec<DateRange>>,
}

#[allow(clippy::option_map_unit_fn)]
pub unsafe fn load_db() { 
    INIT_EVENT.call_once(|| {
        std::env::var("DB_FILE").ok().map(|s| DB_FILE = Box::leak(s.into_boxed_str()));
    });

    let mut db = std::fs::File::open(DB_FILE).unwrap();
    let mut buf = Vec::new();
    db.read_to_end(&mut buf).unwrap();

    if buf.is_empty() {
        EVENT_LIST = Some(Mutex::new(HashMap::new()));
        return;
    }

    let db_de: EventList = bincode::deserialize(&buf).unwrap();

    EVENT_LIST = Some(Mutex::new(db_de));
}

pub unsafe fn store_db(db: &EventList) {
    let db_ser = bincode::serialize(db).unwrap();

    std::fs::write(DB_FILE, db_ser).unwrap();
}

impl Event {
    pub fn add(mut event: Self) -> (Hash, Hash) {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let mut event_uid = gen_hash();

        // make sure its unique
        while db.contains_key(&event_uid) {
            event_uid = gen_hash();
        }

        // add event and edit hash to db
        let edit_hash = gen_hash();
        event.edit_hash = edit_hash;

        db.insert(event_uid, event);
        unsafe{ store_db(&db) };

        (event_uid, edit_hash)
    }

    pub fn edit(event_id: Hash, new_event: Self) {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        db.insert(event_id, new_event);
        unsafe{ store_db(&db) };
    }

    pub fn add_user<'a>(event_id: Hash, user: User, username: String) -> Result<(), &'a str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404");
        };

        // make sure user doesnt already exist
        if event.users.contains_key(&username) {
            return Err("409");
        }

        event.users.insert(username, user);

        unsafe{ store_db(&db) };

        Ok(())
    }

    pub fn edit_user<'a>(event_id: Hash, username: String, new_user: User) -> Result<(), &'a str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404 NOT FOUND");
        };

        // make sure user exists
        let Some(user) = event.users.get_mut(&username) else {
            return Err("404 NOT FOUND");
        };

        if user.pass != new_user.pass {
            return Err("403 FORBIDDEN");
        }

        event.users.insert(username, new_user);

        unsafe{ store_db(&db) };

        Ok(())
    }
}

fn gen_hash() -> Hash {
    let mut rng = rand::thread_rng();
    let mut buf: Hash = ['\0'; 6];

    (0..6).for_each(|i| buf[i] = ALPHANUMARIC[rng.gen_range(0..62)]);
    buf
}
