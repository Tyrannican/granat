use anyhow::{Result, Error, anyhow};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::store::{EntryValue, KVPair};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralStore {
    pub store: HashMap<String, EntryValue>
}

impl GeneralStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub fn set(&mut self, kv: KVPair) -> Result<()> {
        let (key, value) = kv;
        let insert_value = EntryValue::convert(value);
        self.store.insert(key, insert_value);

        Ok(())
    }

    pub fn set_multiple(&mut self, kvs: Vec<KVPair>) -> Result<()> {
        for kv in kvs.into_iter() {
            let (key, value) = kv;
            let insert_value = EntryValue::convert(value);
            self.store.insert(key, insert_value);
        }

        Ok(())
    }

    pub fn safe_set(&mut self, kv: KVPair) -> Result<()> {
        let (key, value) = kv;
        match self.store.get(&key) {
            Some(_) => {},
            None => {
                let insert_value = EntryValue::convert(value);
                self.store.insert(key, insert_value);
            }
        }

        Ok(())
    }

    pub fn safe_set_multiple(&mut self, kvs: Vec<KVPair>) -> Result<()> {
        for kv in kvs.into_iter() {
            let (key, value) = kv;
            match self.store.get(&key) {
                Some(_) => {},
                None => {
                    let insert_value = EntryValue::convert(value);
                    self.store.insert(key, insert_value);
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, key: String) -> Option<String> {
        if let Some(value) = self.store.get(&key) {
            return Some(value.as_string());
        }

        return None;
    }

    pub fn get_multiple(&self, keys: Vec<String>) -> Vec<Option<String>> {
        return keys.into_iter()
            .map(|k| {
                if let Some(value) = self.store.get(&k) {
                    return Some(value.as_string());
                }

                return None;
            })
            .collect::<Vec<Option<String>>>();
    }
}
