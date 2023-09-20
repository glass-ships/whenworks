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

#[allow(clippy::option_map_unit_fn)]
pub unsafe fn load_db() { 
    INIT_EVENT.call_once(|| {
        std::env::var("DB_FILE").ok().map(|s| DB_FILE = Box::leak(s.into_boxed_str()));
    });

    let mut db = std::fs::File::open(DB_FILE).unwrap();
    let mut buf = Vec::new();
    db.read_to_end(&mut buf).unwrap();

    // let mut d = GzDecoder::new(buf, Compression::default());
    // let mut s = String::new();

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
    pub fn add_user<'a>(event_id: Hash, user: UserEntry) -> Result<(), &'a str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404");
        };

        // make sure user doesnt already exist
        if event.users.contains_key(&user.name) {
            return Err("409");
        }

        event.users.insert(user.name.clone(), User::from_entry(user));

        unsafe{ store_db(&db) };

        Ok(())
    }

    pub fn edit_user<'a>(event_id: Hash, new_user: UserEntry) -> Result<(), &'a str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404 NOT FOUND");
        };

        // make sure user exists
        let Some(user) = event.users.get(&new_user.name) else {
            return Err("404 NOT FOUND");
        };

        if user.pass != new_user.pass {
            return Err("403 FORBIDDEN");
        }

        event.users.insert(new_user.name.clone(), User::from_entry(new_user));

        unsafe{ store_db(&db) };

        Ok(())
    }

    pub fn delete_user(event_id: Hash, user_name: &str) -> Result<(), &'static str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404 NOT FOUND");
        };

        // make sure user exists
        if !event.users.contains_key(user_name) {
            return Err("404 NOT FOUND");
        }

        event.users.remove(user_name);

        unsafe{ store_db(&db) };

        Ok(())
    }
    
    pub fn delete_user_en(event_id: Hash, user_name: &str, pass: [char; 8]) -> Result<(), &'static str> {
        let mut db = unsafe { EVENT_LIST.as_ref().unwrap().lock().unwrap() };

        let Some(event) = db.get_mut(&event_id) else {
            return Err("404 NOT FOUND");
        };

        // make sure user exists
        let Some(user) = event.users.get(user_name) else {
            return Err("404 NOT FOUND");
        };

        if pass != user.pass {
            return Err("403 FORBIDDEN");
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

fn gen_hash() -> Hash {
    let mut rng = rand::thread_rng();
    let mut buf: Hash = ['\0'; 6];

    (0..6).for_each(|i| buf[i] = ALPHANUMARIC[rng.gen_range(0..62)]);
    buf
}
