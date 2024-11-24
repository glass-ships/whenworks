use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::sync::Arc;
use std::fs::File;
use std::io::{Write, Read};

use serde::{Deserialize, Serialize};

use crate::hash::Hash;

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
	#[serde(skip)]
	pub creation_date: u64,
	pub name:  Box<str>,
	pub desc:  Option<Box<str>>,
	dates:     Vec<DateRange>,
	pub users: Mutex<Vec<User>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	#[serde(skip)]
	pub pass_hash: Arc<str>,
	pub name: Box<str>,
	comment:  Option<Box<str>>,
	dates:    Vec<DateRange>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DateRange {
	from: u64, 
	to:   u64, 
	preferred: bool,
}


type Events = HashMap<Hash, (Hash, Arc<Event>)>;
pub struct DB(Mutex<(File, Events)>);

impl DB {
	pub fn new() -> Self { 
		let path = std::env::var("DB_PATH")
			.unwrap_or_else(|_| String::from("db.bin"));

		let mut file = std::fs::OpenOptions::new()
			.read(true).write(true).create(true)
			.truncate(true).open(&path)
			.expect("Error opening DB file");

		let mut buf = Vec::with_capacity(file.metadata().unwrap().len() as usize);
		file.read_to_end(&mut buf).expect("Error reading DB file");

		Self(Mutex::new((
			file,
			bincode::deserialize(&buf).unwrap_or_default(),
		))) 
	}

	pub fn read(&self) -> DBGuard
	{ DBGuard(self.0.lock().unwrap()) }

	pub fn write(&self) -> DBGuardMut
	{ DBGuardMut(self.0.lock().unwrap()) }
}


pub struct DBGuard<'a>(MutexGuard<'a, (File, Events)>);
pub struct DBGuardMut<'a>(MutexGuard<'a, (File, Events)>);

impl std::ops::Deref for DBGuard<'_> {
	type Target = Events;
	fn deref(&self) -> &Self::Target 
	{ &self.0.1 }
}

impl std::ops::Deref for DBGuardMut<'_> {
	type Target = Events;
	fn deref(&self) -> &Self::Target 
	{ &self.0.1 }
}

impl std::ops::DerefMut for DBGuardMut<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target 
	{ &mut self.0.1}
}

impl Drop for DBGuardMut<'_> {
	fn drop(&mut self) {
		let events = bincode::serialize(&self.0.1)
			.expect("Error serializing DB");

		self.0.0.write_all(&events).expect("Error writing to DB");
	}
}
