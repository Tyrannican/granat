use anyhow::{Result, anyhow};
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

    pub fn increment(&mut self, key: String, incr: i64) -> Result<String> {
        if let Some(value) = self.store.get_mut(&key) {
            match value {
                EntryValue::Integer(i) => {
                    *i += incr;
                },
                _ => {
                    eprintln!("cannot increment non-integer value");
                    return Err(anyhow!("cannot increment non-integer value"));
                }
            }

            return Ok(value.as_string());
        }

        let initial_value = 0 + incr;
        self.store.insert(key, EntryValue::Integer(initial_value));
        return Ok(initial_value.to_string());
    }

    pub fn increment_float(&mut self, key: String, incr: f64) -> Result<String> {
        if let Some(value) = self.store.get_mut(&key) {
            match value {
                EntryValue::Float(i) => {
                    *i += incr;
                },
                _ => {
                    eprintln!("cannot increment non-integer value");
                    return Err(anyhow!("cannot increment non-integer value"));
                }
            }

            return Ok(value.as_string());
        }

        let initial_value = 0. + incr;
        self.store.insert(key, EntryValue::Float(initial_value));
        return Ok(initial_value.to_string());
    }
}

#[cfg(test)]
mod general_store_tests {
    use super::*;
}
