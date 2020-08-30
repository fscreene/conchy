use rocket::State;
use std::sync::{Arc, Mutex};
use crate::Server;

#[get("/config/<key>")]
pub fn get_config(store: State<Arc<Mutex<Server>>>, key: String) -> String {
    let guard = match store.lock() {
        Err(poisoned) => poisoned.into_inner(),
        Ok(lock) => lock
    };
    guard.local_store.get(&key)
}

#[put("/config/<key>/<value>")]
pub fn put_config(store: State<Arc<Mutex<Server>>>, key: String, value: String) {
    let mut guard = match store.lock() {
        Err(poisoned) => poisoned.into_inner(),
        Ok(lock) => lock
    };
    guard.local_store.add(&key, value)
}

#[get("/config/all")]
pub fn all(store: State<Arc<Mutex<Server>>>) -> String {
    let guard = match store.lock() {
        Err(poisoned) => poisoned.into_inner(),
        Ok(lock) => lock
    };
    let mut keys =vec![""];
    for key in guard.local_store.keys() {
        keys.push(key);
    };
    keys.join(",")
}