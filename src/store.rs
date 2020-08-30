use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub struct ConfigStore {
    in_memory: HashMap<String, String>,
}

impl ConfigStore {
    pub fn new() -> ConfigStore {
        ConfigStore { in_memory: HashMap::new() }
    }

    pub fn add(&mut self, key: &str, value: String) {
        self.in_memory.insert(String::from(key), value);
    }

    pub fn get(&self, key: &str) -> String {
        String::from(self.in_memory.get(key).expect("Value not found"))
    }

    pub fn keys(&self) -> Keys<String, String> {
        self.in_memory.keys()
    }
}