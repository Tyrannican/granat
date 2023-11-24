use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, LinkedList};

use crate::store::{
    entry::{Entry, EntryValue},
    KVPair,
};

#[derive(Debug, PartialEq)]
enum ListDirection {
    Left,
    Right,
}

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

    pub fn left_push(&mut self, kv: KVPair) {
        self.push(kv, ListDirection::Left);
    }

    pub fn right_push(&mut self, kv: KVPair) {
        self.push(kv, ListDirection::Right);
    }

    fn push(&mut self, kv: KVPair, dir: ListDirection) {
        let (key, value) = kv;

        if let Some(list) = self.store.get_mut(&key) {
            match dir {
                ListDirection::Left => list.push_front(value),
                ListDirection::Right => list.push_back(value),
            }
        } else {
            let mut list = LinkedList::new();
            match dir {
                ListDirection::Left => list.push_front(value),
                ListDirection::Right => list.push_back(value),
            }

            self.store.insert(key.to_string(), list);
        }
    }

    pub fn left_pop(&mut self, key: impl AsRef<str>) -> Option<Entry> {
        return self.pop(key, ListDirection::Left);
    }

    pub fn right_pop(&mut self, key: impl AsRef<str>) -> Option<Entry> {
        return self.pop(key, ListDirection::Right);
    }

    fn pop(&mut self, key: impl AsRef<str>, dir: ListDirection) -> Option<Entry> {
        if let Some(list) = self.store.get_mut(key.as_ref()) {
            let item = match dir {
                ListDirection::Left => list.pop_front(),
                ListDirection::Right => list.pop_back(),
            };

            if list.len() == 0 {
                self.store.remove(key.as_ref());
            }

            return item;
        }

        return None;
    }

    pub fn index(&self, key: impl AsRef<str>, idx: usize) {
        if let Some(list) = self.store.get(key.as_ref()) {
            for item in list.iter().rev().enumerate() {}
        }
    }

    pub fn len(&self, key: impl AsRef<str>) -> usize {
        if let Some(list) = self.store.get(key.as_ref()) {
            return list.len();
        }

        return 0;
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    // L/R Push ✔
    // L/R Pop ✔
    // L/R Index
    // Length ✔
    // Set
    // Trim
    // Remove
}
