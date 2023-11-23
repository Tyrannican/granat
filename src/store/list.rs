use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, LinkedList};

use crate::store::{
    entry::{Entry, EntryValue},
    KVPair,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListStore {
    pub store: HashMap<String, LinkedList<Entry>>,
}

impl ListStore {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
        };
    }

    pub fn left_push(&mut self, kv: KVPair) {}
    pub fn right_push(&mut self, kv: KVPair) {}
}
