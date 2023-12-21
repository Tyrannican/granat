use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::store::entry::StoreEntry;
use crate::store::KVPair;

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralStore {
    pub store: HashMap<String, StoreEntry>,
}

impl GeneralStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, kv: KVPair) -> Result<()> {
        let (key, value) = kv;
        self.store.insert(key, value);

        return Ok(());
    }

    pub fn set_multiple(&mut self, kvs: Vec<KVPair>) -> Result<()> {
        for kv in kvs.into_iter() {
            let (key, value) = kv;
            self.store.insert(key, value);
        }
        return Ok(());
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<StoreEntry> {
        if let Some(raw) = self.store.get(key.as_ref()) {
            return Some(raw.clone());
        }

        return None;
    }

    pub fn get_multiple(&self, keys: Vec<impl AsRef<str>>) -> Vec<Option<StoreEntry>> {
        return keys
            .into_iter()
            .map(|k| {
                if let Some(raw) = self.store.get(k.as_ref()) {
                    return Some(raw.clone());
                }

                return None;
            })
            .collect::<Vec<Option<StoreEntry>>>();
    }

    pub fn increment(&mut self, key: impl AsRef<str>, incr: i64) -> Result<i64> {
        if let Some(raw) = self.store.get_mut(key.as_ref()) {
            match raw.value.parse::<i64>() {
                Ok(mut val) => {
                    val += incr;
                    raw.value = val.to_string();

                    return Ok(val);
                }
                Err(e) => {
                    return Err(anyhow!("unable to convert to integer: {e}"));
                }
            }
        }

        let initial_value = 0 + incr;
        let key_value = key.as_ref().to_string();

        self.store
            .insert(key_value, StoreEntry::new(initial_value.to_string()));

        return Ok(initial_value);
    }

    pub fn increment_float(&mut self, key: impl AsRef<str>, incr: f64) -> Result<f64> {
        if let Some(raw) = self.store.get_mut(key.as_ref()) {
            match raw.value.parse::<f64>() {
                Ok(mut val) => {
                    val += incr;
                    raw.value = val.to_string();

                    return Ok(val);
                }
                Err(e) => {
                    return Err(anyhow!("unable to convert to float: {e}"));
                }
            }
        }

        let initial_value = 0. + incr;
        let key_value = key.as_ref().to_string();
        self.store
            .insert(key_value, StoreEntry::new(initial_value.to_string()));

        return Ok(initial_value);
    }
}

#[cfg(test)]
mod general_store_tests {
    use super::*;

    fn create_kv(key: &str, value: &str) -> KVPair {
        return (key.to_string(), StoreEntry::new(value));
    }

    #[test]
    fn get_set_single() {
        let mut gs = GeneralStore::new();

        let _ = gs.set(create_kv("string", "string test value"));
        let _ = gs.set(create_kv("number", "120"));
        let _ = gs.set(create_kv("float", "347.84"));

        assert!(gs.store.len() == 3);

        let mut res = gs.get("string".to_string());
        assert!(res.is_some());
        let mut value = res.unwrap();
        assert_eq!(value.value, "string test value".to_string());

        res = gs.get("number".to_string());
        assert!(res.is_some());
        value = res.unwrap();
        assert_eq!(value.value, "120".to_string());

        res = gs.get("float".to_string());
        assert!(res.is_some());
        value = res.unwrap();
        assert_eq!(value.value, "347.84".to_string());
    }

    #[test]
    fn get_set_muiltiple() {
        let mut gs = GeneralStore::new();

        let inserts = vec![
            create_kv("test", "test_val"),
            create_kv("banana", "42"),
            create_kv("apple", "99.99"),
            create_kv("tucson", "arizona"),
        ];

        let _ = gs.set_multiple(inserts);

        let keys = vec![
            "banana".to_string(),
            "tucson".to_string(),
            "non-existant".to_string(),
            "apple".to_string(),
        ];

        let results = gs.get_multiple(keys);
        let some_total = results.iter().filter(|e| e.is_some()).count();
        let none_total = results.iter().filter(|e| e.is_none()).count();
        assert_eq!(some_total, 3);
        assert_eq!(none_total, 1);
    }

    #[test]
    fn increment_integer() {
        let mut gs = GeneralStore::new();
        let inserts = vec![
            create_kv("integer", "10"),
            create_kv("string", "not a number"),
        ];

        let _ = gs.set_multiple(inserts);

        let mut result = gs.increment("integer".to_string(), 10);
        assert!(result.is_ok());
        let mut inner = result.unwrap();
        assert_eq!(inner, 20);

        result = gs.increment("integer".to_string(), -25);
        assert!(result.is_ok());
        inner = result.unwrap();
        assert_eq!(inner, -5);

        result = gs.increment("string".to_string(), 10);
        assert!(result.is_err());
    }

    #[test]
    fn increment_float() {
        let mut gs = GeneralStore::new();
        let inserts = vec![
            create_kv("float", "10.2"),
            create_kv("string", "not a number"),
        ];

        let _ = gs.set_multiple(inserts);

        let mut result = gs.increment_float("float".to_string(), 30.7);
        assert!(result.is_ok());
        let inner = result.unwrap();
        assert_eq!(inner, 40.9);

        result = gs.increment_float("float".to_string(), -45.8);
        assert!(result.is_ok());

        result = gs.increment_float("string".to_string(), 10.0);
        assert!(result.is_err());
    }
}
