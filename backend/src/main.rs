#![allow(forbidden_lint_groups)]
#![forbid(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::pedantic, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn, clippy::cast_possible_truncation)]

use axum::{routing::{post, get}, http::StatusCode};
use axum::extract::{Json, Path};

use serde_json::{json, Value};

use std::sync::{LazyLock, Arc};
use std::ops::Not;

mod event;
use event::{Event, DB, Hash, User};

static DB: LazyLock<DB> = LazyLock::new(DB::new);
static MAX_EVENT_AGE: LazyLock<u64> = LazyLock::new(||
    std::env::var("MAX_EVENT_AGE")
        .ok().and_then(|s| s.parse().ok())
        .unwrap_or(30 * 24 * 60 * 60) // 30 days
);
static BOOKKEEPING_INTERVAL: LazyLock<u64> = LazyLock::new(||
    std::env::var("BOOKKEEPING_INTERVAL")
        .ok().and_then(|s| s.parse().ok())
        .unwrap_or(30 * 60) // 30 min
);

#[tokio::main]
async fn main() {
    let addr = std::env::args().skip(1).next()
        .unwrap_or_else(|| String::from("127.0.0.1:8080"));

    // bookkeeping
    tokio::spawn(async {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_secs();

        DB.write().retain(|_, (_, e)| e.creation_date + *MAX_EVENT_AGE > now);
        tokio::time::sleep(std::time::Duration::from_secs(*BOOKKEEPING_INTERVAL)).await;
    });

    #[cfg(debug_assertions)]
    println!("{:#?}", *DB.read());

    let router = axum::Router::new()
        .route("/api",               get(|| async { "this is the whenworks api/db" }))
        .route("/api/new",           post(new_event))
        .route("/api/:id/edit",      post(edit_event))
        .route("/api/:id/del",       post(del_event))
        .route("/api/:id/user/edit", post(edit_user))
        .route("/api/:id/user/del",  post(del_user))
        .route("/api/:id/user/new",  post(add_user))
        .route("/api/:id",           get(get_event));
    
    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect("Error binding listener");
    axum::serve(listener, router).await.unwrap();
}

type Response<T> = Result<(StatusCode, T), (StatusCode, &'static str)>;

async fn get_event(Path(id): Path<String>) -> Response<Json<(u64, Arc<Event>)>> {
    let hash = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;

    DB.read().get(&hash)
        .ok_or((StatusCode::NOT_FOUND, "Event not found"))
        .map(|(_, e)| (StatusCode::OK, Json((e.creation_date, Arc::clone(e)))))
}

async fn new_event(Json(mut event): Json<Event>) -> Response<Json<Value>> {
    (event.name.len() > 32).not().then_some(())
        .ok_or((StatusCode::BAD_REQUEST, "Name too long (max 32 chars)"))?;

    (event.name.is_empty()).not().then_some(())
        .ok_or((StatusCode::BAD_REQUEST, "Name cannot be empty"))?;

    event.desc.as_ref().filter(|d| d.len() <= 256)
        .ok_or((StatusCode::BAD_REQUEST, "Description too long (max 256 chars)"))?;

    let id = {
        let db = DB.read();
        loop {
            let id = Hash::new();
            if !db.contains_key(&id) { break id; }
        }
    };

    event.creation_date = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap().as_secs();

    let key = Hash::new();
    DB.write().insert(id, (key, Arc::new(event)));

    Ok((StatusCode::CREATED, Json(json!({
        "uid": id.to_string(),
        "key": key.to_string(),
    }))))
}

async fn edit_event(Path(id): Path<Box<str>>, Json((key, mut event)): Json<(Box<str>, Event)>) -> Response<&'static str> {
    let id = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;
    let key = Hash::from(&key).ok_or((StatusCode::BAD_REQUEST, "Invalid key"))?;

    DB.read().get(&id).ok_or((StatusCode::NOT_FOUND, "Event not found"))
        .and_then(|(k, e)| key.eq(k).then_some(())
            .ok_or((StatusCode::FORBIDDEN, "Invalid key"))
            .map(|_| event.creation_date = e.creation_date)
        )?;

    DB.write().get_mut(&id).unwrap().1 = Arc::new(event);

    Ok((StatusCode::OK, "OK"))
}

async fn del_event(Path(id): Path<Box<str>>, Json(key): Json<Box<str>>) -> Response<&'static str> {
    let id = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;
    let key = Hash::from(&key).ok_or((StatusCode::BAD_REQUEST, "Invalid key"))?;

    DB.read().get(&id).ok_or((StatusCode::NOT_FOUND, "Event not found"))
        .and_then(|(k, _)| key.eq(k).then_some(())
            .ok_or((StatusCode::FORBIDDEN, "Invalid key")))?;

    DB.write().remove(&id);

    Ok((StatusCode::OK, "OK"))
}


async fn add_user(Path(id): Path<Box<str>>, Json((pass, mut user)): Json<(Box<[u8]>, User)>) -> Response<&'static str> {
    let id = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;

    DB.read().get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Event not found"))
        .and_then(|(_, e)| e.users.lock().unwrap()
            .iter().any(|u| u.name == user.name).not().then_some(())
            .ok_or((StatusCode::CONFLICT, "User already exists"))
        )?;

    user.pass_hash = bcrypt::hash(pass, bcrypt::DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password"))
        .map(|p| Arc::from(p))?;

    DB.write().get(&id).unwrap().1.users
        .lock().unwrap().push(user);

    Ok((StatusCode::CREATED, "OK"))
}

async fn edit_user(Path(id): Path<Box<str>>, Json((pass, mut user)): Json<(Box<[u8]>, User)>) -> Response<&'static str> {
    let id = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;

    DB.read().get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Event Not found"))
        .and_then(|(_, e)| e.users.lock().unwrap()
            .iter().find(|u| u.name == user.name)
            .map(|u| user.pass_hash = u.pass_hash.clone())
            .ok_or((StatusCode::NOT_FOUND, "User not found")))?;

    bcrypt::verify(pass, &user.pass_hash).map_or(
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Error hashing key")),
        |b| b.then_some(()).ok_or((StatusCode::FORBIDDEN, "Invalid key"))
    )?;

    let _ = std::mem::replace(
        DB.write().get(&id).unwrap().1.users
            .lock().unwrap()
            .iter_mut().find(|u| u.name == user.name)
            .unwrap(), 
        user
    );

    Ok((StatusCode::OK, "OK"))
}

async fn del_user(Path(id): Path<Box<str>>, Json((pass, uname)): Json<(Box<[u8]>, Box<str>)>) -> Response<&'static str> {
    let id = Hash::from(&id).ok_or((StatusCode::BAD_REQUEST, "Invalid id"))?;

    DB.read().get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Event Not found"))
        .and_then(|(_, e)| e.users.lock().unwrap()
            .iter().find(|u| u.name == uname)
            .map(|u| u.pass_hash.clone())
            .ok_or((StatusCode::NOT_FOUND, "User not found")))
        .and_then(|k| bcrypt::verify(pass, &k).map_or(
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error hashing key")),
            |b| b.then_some(()).ok_or((StatusCode::FORBIDDEN, "Invalid key"))
        ))?;

    DB.write().get(&id).unwrap().1.users
        .lock().unwrap().retain(|u| u.name != uname);

    Ok((StatusCode::OK, "OK"))
}
