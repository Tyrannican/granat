use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::store::entry::Entry;
use crate::store::{KVPair, StoreMode};

use std::collections::{HashMap, LinkedList};

#[derive(Clone, Debug, PartialEq)]
pub enum ListDirection {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStore {
    pub store: HashMap<String, LinkedList<Entry>>,

    #[serde(skip)]
    pub mode: StoreMode,
}

impl ListStore {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
            mode: StoreMode::default(),
        };
    }

    pub fn safe_mode(&mut self) {
        self.mode = StoreMode::Safe;
    }

    pub fn normal_mode(&mut self) {
        self.mode = StoreMode::Normal;
    }
}
