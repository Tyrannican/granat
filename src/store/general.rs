use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::store::{EntryValue, StoreMode, KVPair};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralStore {
    pub store: HashMap<String, EntryValue>,

    #[serde(skip)]
    pub mode: StoreMode
}

impl GeneralStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            mode: StoreMode::default()
        }
    }

    pub fn safe_mode(&mut self) {
        self.mode = StoreMode::Safe;
    }

    pub fn normal_mode(&mut self) {
        self.mode = StoreMode::Normal;
    }

    pub fn set(&mut self, kv: KVPair) -> Result<()> {
        let (key, value) = kv;
        if self.store.get(&key).is_some() && self.mode == StoreMode::Safe {
            return Err(anyhow!("cannot set values in safe mode"));
        }

        let insert_value = EntryValue::convert(value);
        self.store.insert(key, insert_value);

        return Ok(());
    }

    pub fn set_multiple(&mut self, kvs: Vec<KVPair>) -> Result<()> {
        for kv in kvs.into_iter() {
            let (key, value) = kv;
            if self.store.get(&key).is_some() && self.mode == StoreMode::Safe {
                return Err(anyhow!("cannot set values in safe mode"));
            }

            let insert_value = EntryValue::convert(value);
            self.store.insert(key, insert_value);
        }
        return Ok(());
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

    fn create_kv(key: &str, value: &str) -> KVPair {
        return (key.to_string(), value.to_string());
    }

    #[test]
    fn set_get_single() {
        let mut gs = GeneralStore::new();

        let _ = gs.set(create_kv("string", "string test value"));
        let _ = gs.set(create_kv("number", "120"));
        let _ = gs.set(create_kv("float", "347.84"));

        assert!(gs.store.len() == 3);

        let mut res = gs.get("string".to_string());
        assert!(res.is_some());
        let mut value = res.unwrap();
        assert_eq!(value, "string test value".to_string());

        res = gs.get("number".to_string());
        assert!(res.is_some());
        value = res.unwrap();
        assert_eq!(value, "120".to_string());

        res = gs.get("float".to_string());
        assert!(res.is_some());
        value = res.unwrap();
        assert_eq!(value, "347.84".to_string());
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
            "apple".to_string()
        ];

        let results = gs.get_multiple(keys);
        assert_eq!(results[0], Some("42".to_string()));
        assert_eq!(results[1], Some("arizona".to_string()));
        assert_eq!(results[2], None);
        assert_eq!(results[3], Some("99.99".to_string()));
    }

    #[test]
    fn safe_set() {
        let mut gs = GeneralStore::new();
        gs.safe_mode();
        let _ = gs.set(create_kv("test", "original"));
        assert_eq!(gs.get("test".to_string()), Some("original".to_string()));

        let _ = gs.set(create_kv("test", "newer"));
        assert_eq!(gs.get("test".to_string()), Some("original".to_string()));
    }

    #[test]
    fn safe_set_multiple() {
        let mut gs = GeneralStore::new();
        let originals = vec![
            create_kv("banana", "pie"),
            create_kv("apple", "55"),
            create_kv("orange", "juice"),
        ];

        let replacements = vec![
            create_kv("banana", "no replace"),
            create_kv("apple", "no replace"),
            create_kv("orange", "no replace:#![warn()]"),
        ];

        gs.safe_mode();
        let _ = gs.set_multiple(originals);
        let _ = gs.set_multiple(replacements);

        assert_eq!(gs.get("banana".to_string()), Some("pie".to_string()));
        assert_eq!(gs.get("apple".to_string()), Some("55".to_string()));
        assert_eq!(gs.get("orange".to_string()), Some("juice".to_string()));
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
        assert_eq!(inner, "20".to_string());

        result = gs.increment("integer".to_string(), -25);
        assert!(result.is_ok());
        inner = result.unwrap();
        assert_eq!(inner, "-5".to_string());

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
        assert_eq!(inner, "40.9".to_string());

        result = gs.increment_float("float".to_string(), -45.8);
        assert!(result.is_ok());

        result = gs.increment_float("string".to_string(), 10.0);
        assert!(result.is_err());
    }
}
