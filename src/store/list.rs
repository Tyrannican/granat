use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::store::{EntryValue, StoreMode, KVPair};

use std::collections::{HashMap, LinkedList};

#[derive(Clone, Debug, PartialEq)]
pub enum ListDirection {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStore {
    pub store: HashMap<String, LinkedList<EntryValue>>,

    #[serde(skip)]
    pub mode: StoreMode
}

impl ListStore {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
            mode: StoreMode::default()
        };
    }

    pub fn mode(&mut self, mode: StoreMode) {
        self.mode = mode;
    }

    pub fn push(&mut self, kv: KVPair, dir: ListDirection) -> Result<()> {
        let (key, value) = kv;
        let inner_list = match self.store.get_mut(&key) {
            Some(v) => v,
            None => {
                self.store.insert(key.clone(), LinkedList::new());
                self.store.get_mut(&key).unwrap()
            }
        };

        let insert_value = EntryValue::convert(value);
        if dir == ListDirection::Left {
            inner_list.push_front(insert_value);
        } else {
            inner_list.push_back(insert_value);
        }

        return Ok(())
    }
    
    pub fn push_multiple(&mut self, kvs: Vec<KVPair>, dir: ListDirection) -> Result<()> {
        for kv in kvs.into_iter() {
            let _ = self.push(kv, dir.clone());
        }
        return Ok(());
    }
}
