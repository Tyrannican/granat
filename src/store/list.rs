use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, LinkedList};

use crate::store::{
    entry::{Entry, EntryValue},
    KVPair,
};

fn idx_from_offset(list_size: usize, idx: isize) -> isize {
    if idx >= 0 {
        return idx;
    };

    return list_size as isize + idx;
}

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

    pub fn index(&self, key: impl AsRef<str>, idx: isize) -> Option<Entry> {
        if let Some(list) = self.store.get(key.as_ref()) {
            let target_idx = idx_from_offset(list.len(), idx);
            if target_idx < 0 {
                return None;
            }

            for (i, item) in list.iter().enumerate() {
                if target_idx as usize == i {
                    return Some(item.clone());
                }
            }
        }

        return None;
    }

    pub fn len(&self, key: impl AsRef<str>) -> usize {
        if let Some(list) = self.store.get(key.as_ref()) {
            return list.len();
        }

        return 0;
    }

    pub fn range(
        &self,
        key: impl AsRef<str>,
        mut start: isize,
        mut end: isize,
    ) -> LinkedList<Entry> {
        let mut ll = LinkedList::new();

        if let Some(list) = self.store.get(key.as_ref()) {
            start = idx_from_offset(list.len(), start);
            end = idx_from_offset(list.len(), end);
            let size = list.len() as isize;

            if start >= size || start > end {
                return ll;
            }

            if end >= size {
                end = size - 1;
            }

            println!("Range: {start} - {end}");
            for (i, item) in list.iter().enumerate() {
                if i as isize >= start && i as isize <= end {
                    ll.push_back(item.clone());
                }
            }
        }

        return ll;
    }

    pub fn set(&mut self, kv: KVPair, idx: isize) -> Result<()> {
        let (key, value) = kv;

        if let Some(list) = self.store.get_mut(&key) {
            let size = list.len() as isize;
            let target_idx = idx_from_offset(list.len(), idx);

            if target_idx < 0 || target_idx > size {
                return Err(anyhow!("index out of range"));
            }

            let mut split = list.split_off(target_idx as usize);
            split.push_front(value);
            list.append(&mut split);
        }

        return Ok(());
    }

    pub fn trim(&mut self, key: impl AsRef<str>, mut start: isize, mut end: isize) {
        if let Some(list) = self.store.get_mut(key.as_ref()) {
            let size = list.len() as isize;
            start = idx_from_offset(list.len(), start);
            end = idx_from_offset(list.len(), end);

            if start >= size as isize || start > end {
                self.store.remove(key.as_ref());
                return;
            }

            if end >= size {
                end = size - 1;
            }

            let mut first = list.split_off(start as usize);
            first.split_off(end as usize - 1);
            self.store.insert(key.as_ref().to_string(), first);
        }
    }

    pub fn remove(&mut self) {}
}

#[cfg(test)]
mod list_tests {
    use super::*;

    // L/R Push ✔
    // L/R Pop ✔
    // L/R Index ✔
    // Length ✔
    // Set ✔
    // Trim ✔
    // Range ✔
    // Remove
    //
    // TODO: Actually write the tests
    //

    fn create_kv_pair(key: impl AsRef<str>, value: impl AsRef<str>) -> KVPair {
        return (key.as_ref().to_string(), Entry::new(value));
    }

    #[test]
    fn test_output() {
        let mut ll = ListStore::new();
        ll.right_push(create_kv_pair("test", "0"));
        ll.right_push(create_kv_pair("test", "1"));
        ll.right_push(create_kv_pair("test", "2"));
        ll.right_push(create_kv_pair("test", "3"));
        ll.right_push(create_kv_pair("test", "4"));
        ll.right_push(create_kv_pair("test", "5"));

        ll.trim("test", 2, 4);
        println!("List: {:?}", ll.range("test", 0, -1));
    }
}
