use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, LinkedList};

use crate::store::{entry::Entry, KVPair};

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

            let mut split = list.split_off(start as usize);
            split.split_off(end as usize - 1);
            self.store.insert(key.as_ref().to_string(), split);
        }
    }

    pub fn remove(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
        mut count: isize,
    ) -> usize {
        let mut total_removed = 0;
        if let Some(list) = self.store.get_mut(key.as_ref()) {
            let target = value.as_ref().to_string();

            if count == 0 {
                while let Some(idx) = find_entry(list, &target, ListDirection::Left) {
                    let mut right = list.split_off(idx);
                    right.pop_front();
                    list.append(&mut right);
                    total_removed += 1;
                }
            } else if count > 0 {
                while let Some(idx) = find_entry(list, &target, ListDirection::Left) {
                    let mut right = list.split_off(idx);
                    right.pop_front();
                    list.append(&mut right);
                    total_removed += 1;
                    count -= 1;

                    if count == 0 {
                        break;
                    }
                }
            } else {
                while let Some(idx) = find_entry(list, &target, ListDirection::Right) {
                    let mut right = list.split_off(idx);
                    right.pop_front();
                    list.append(&mut right);
                    total_removed += 1;
                    count -= 1;

                    if count == 0 {
                        break;
                    }
                }
            }

            if list.len() == 0 {
                self.store.remove(key.as_ref());
            }
        }

        return total_removed;
    }
}

fn find_entry(list: &mut LinkedList<Entry>, target: &String, dir: ListDirection) -> Option<usize> {
    match dir {
        ListDirection::Left => {
            for (pos, entry) in list.iter().enumerate() {
                if &entry.value == target {
                    return Some(pos);
                }
            }
        }
        ListDirection::Right => {
            let offset = list.len() - 1;
            for (pos, entry) in list.iter().rev().enumerate() {
                if &entry.value == target {
                    return Some(offset - pos);
                }
            }
        }
    }

    None
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
    // Remove ✔
    //
    // TODO: Actually write the tests
    //

    fn create_kv_pair(key: impl AsRef<str>, value: impl AsRef<str>) -> KVPair {
        return (key.as_ref().to_string(), Entry::new(value));
    }

    #[test]
    fn push_left() {
        let mut list_store = ListStore::new();
        list_store.left_push(create_kv_pair("test", "2"));
        list_store.left_push(create_kv_pair("test", "1"));
        list_store.left_push(create_kv_pair("test", "0"));

        let inner = list_store.store.get_mut("test").unwrap();
        let mut val = inner.pop_front().unwrap().value;
        assert_eq!(val, "0".to_string());

        val = inner.pop_front().unwrap().value;
        assert_eq!(val, "1".to_string());

        val = inner.pop_front().unwrap().value;
        assert_eq!(val, "2".to_string());
    }

    #[test]
    fn push_right() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));

        let inner = list_store.store.get_mut("test").unwrap();
        let mut val = inner.pop_front().unwrap().value;
        assert_eq!(val, "0".to_string());

        val = inner.pop_front().unwrap().value;
        assert_eq!(val, "1".to_string());

        val = inner.pop_front().unwrap().value;
        assert_eq!(val, "2".to_string());
    }

    #[test]
    fn length() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));

        assert_eq!(list_store.len("test"), 3);
    }

    #[test]
    fn pop_left() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));

        let mut entry = list_store.left_pop("test");
        assert!(entry.is_some());
        let mut value = entry.unwrap().value;
        assert_eq!(value, "0".to_string());
        assert_eq!(list_store.len("test"), 2);

        entry = list_store.left_pop("test");
        assert!(entry.is_some());
        value = entry.unwrap().value;
        assert_eq!(value, "1".to_string());
        assert_eq!(list_store.len("test"), 1);

        entry = list_store.left_pop("test");
        assert!(entry.is_some());
        value = entry.unwrap().value;
        assert_eq!(value, "2".to_string());
        assert_eq!(list_store.len("test"), 0);

        entry = list_store.left_pop("test");
        assert!(entry.is_none());
    }

    #[test]
    fn pop_right() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));

        let mut entry = list_store.right_pop("test");
        assert!(entry.is_some());
        let mut value = entry.unwrap().value;
        assert_eq!(value, "2".to_string());
        assert_eq!(list_store.len("test"), 2);

        entry = list_store.right_pop("test");
        assert!(entry.is_some());
        value = entry.unwrap().value;
        assert_eq!(value, "1".to_string());
        assert_eq!(list_store.len("test"), 1);

        entry = list_store.right_pop("test");
        assert!(entry.is_some());
        value = entry.unwrap().value;
        assert_eq!(value, "0".to_string());
        assert_eq!(list_store.len("test"), 0);

        entry = list_store.right_pop("test");
        assert!(entry.is_none());
    }

    #[test]
    fn index_from_left() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));
        list_store.right_push(create_kv_pair("test", "3"));
        list_store.right_push(create_kv_pair("test", "4"));

        for i in 0..5 {
            let entry = list_store.index("test", i as isize);
            assert!(entry.is_some());
            let value = entry.unwrap().value;
            assert_eq!(value, i.to_string());
        }

        let entry = list_store.index("test", 563);
        assert!(entry.is_none());
    }

    #[test]
    fn index_from_right() {
        let mut list_store = ListStore::new();
        list_store.right_push(create_kv_pair("test", "0"));
        list_store.right_push(create_kv_pair("test", "1"));
        list_store.right_push(create_kv_pair("test", "2"));
        list_store.right_push(create_kv_pair("test", "3"));
        list_store.right_push(create_kv_pair("test", "4"));

        let mut str_val = 4;
        for i in (-5..0).rev() {
            let entry = list_store.index("test", i as isize);
            assert!(entry.is_some());
            let value = entry.unwrap().value;
            assert_eq!(value, str_val.to_string());
            str_val -= 1;
        }

        let entry = list_store.index("test", -324);
        assert!(entry.is_none());
    }
}
