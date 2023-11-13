use serde::{Serialize, Deserialize};

use crate::store::EntryValue;

use std::collections::{HashMap, LinkedList};


#[derive(Debug, Serialize, Deserialize)]
pub struct ListStore {
    pub store: HashMap<String, LinkedList<EntryValue>>
}

impl ListStore {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new()
        };
    }


}
