use anyhow::{Result, Error, anyhow};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type KVPair = (String, String);

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EntryValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    String(String)
}

impl EntryValue {
    pub fn convert(value: impl AsRef<str>) -> Self {
        if let Ok(v) = value.as_ref().parse::<i8>() {
            return Self::I8(v);
        } else if let Ok(v) = value.as_ref().parse::<i16>() {
            return Self::I16(v);
        } else if let Ok(v) = value.as_ref().parse::<i32>() {
            return Self::I32(v);
        } else if let Ok(v) = value.as_ref().parse::<i64>() {
            return Self::I64(v);
        } else if let Ok(v) = value.as_ref().parse::<i128>() {
            return Self::I128(v);
        } else {
            return Self::String(value.as_ref().to_string());
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::I8(v) => {
                return format!("{v}");
            }
            Self::I16(v) => {
                return format!("{v}");
            }
            Self::I32(v) => {
                return format!("{v}");
            }
            Self::I64(v) => {
                return format!("{v}");
            }
            Self::I128(v) => {
                return format!("{v}");
            }
            Self::String(v) => {
                return v.to_string();
            }
        }
    }
}


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

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn create_entry_value() {
        let sm = "12".to_string();
        let md = "412".to_string();
        let nm = "100000".to_string();
        let lg = "3000000000".to_string();
        let hg = "170131183460469231731687303".to_string();
        let s = "I am a string".to_string();

        let sm_v = EntryValue::convert(sm);
        assert_eq!(sm_v, EntryValue::I8(12));

        let md_v = EntryValue::convert(md);
        assert_eq!(md_v, EntryValue::I16(412));

        let nm_v = EntryValue::convert(nm);
        assert_eq!(nm_v, EntryValue::I32(100_000));

        let lg_v = EntryValue::convert(lg);
        assert_eq!(lg_v, EntryValue::I64(3000000000));

        let hg_v = EntryValue::convert(hg);
        assert_eq!(hg_v, EntryValue::I128(170131183460469231731687303));

        let s_v = EntryValue::convert(s);
        assert_eq!(s_v, EntryValue::String("I am a string".to_string()));
    }

    #[test]
    fn convert_entry_value_back_to_string() {
        let sm_v = EntryValue::I8(12);
        let md_v = EntryValue::I16(412);
        let nm_v = EntryValue::I32(100_000);
        let lg_v = EntryValue::I64(3000000000);
        let hg_v = EntryValue::I128(170131183460469231731687303);
        let s_v = EntryValue::String("I am a string".to_string());

        let sm = sm_v.as_string();
        let md = md_v.as_string();
        let nm = nm_v.as_string();
        let lg = lg_v.as_string();
        let hg = hg_v.as_string();
        let s = s_v.as_string();

        assert_eq!(sm, "12".to_string());
        assert_eq!(md, "412".to_string());
        assert_eq!(nm, "100000".to_string());
        assert_eq!(lg, "3000000000".to_string());
        assert_eq!(hg, "170131183460469231731687303".to_string());
        assert_eq!(s, "I am a string".to_string());
    }
}
