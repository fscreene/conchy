#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::sync::{Arc, Mutex};
use rocket::fairing::AdHoc;

mod store;
mod commands;
mod endpoints;

pub struct Server {
    local_store: store::ConfigStore,
}

fn main() {
    let server = Server { local_store: store::ConfigStore::new() };
    let state = Arc::new(Mutex::new(server));
    rocket::ignite()
        .manage(state.clone())
        .mount("/", routes![
                endpoints::get_config,
                endpoints::put_config,
                endpoints::all
            ])
        .attach(AdHoc::on_launch("Config", |_rocket| {
            commands::load_configs(state);
        }))
        .launch();
}
