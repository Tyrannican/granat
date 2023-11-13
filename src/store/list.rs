use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::store::{EntryValue, StoreMode};

use std::collections::{HashMap, LinkedList};


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

    pub fn push(&mut self, key: String, value: String) -> Result<()> {
        return Ok(())
    }
}
