use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use log::info;

use crate::Server;
use std::fs;

pub fn load_configs(state: Arc<Mutex<Server>>) {
    let config_path = PathBuf::from("configs");
    let result = fs::read_dir(config_path).expect("failed to find config directory. have you created it at configs/?");
    result.for_each(|file| {
        let entry = file.expect("Couldn't get dir entry");
        let file_name = entry.file_name();
        let file_name = file_name.to_str().expect("Couldn't get filename");
        let file_content = fs::read_to_string(entry.path());
        let mut guard = match state.lock() {
            Err(poison) => poison.into_inner(),
            Ok(guard) => guard
        };
        info!("adding new config {} to store", file_name);
        guard.local_store.add(&file_name, file_content.expect("Can't get file content"));
    });
}
