use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::store::entry::StoreEntry;
use crate::store::KVPair;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct SetStore {
    pub store: HashMap<String, HashSet<StoreEntry>>,
}

impl SetStore {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
        };
    }

    pub fn add(&mut self, kv: KVPair) {
        let (key, value) = kv;
        if let Some(inner) = self.store.get_mut(&key) {
            inner.insert(value.clone());
        } else {
            let mut inner = HashSet::new();
            inner.insert(value.clone());
            self.store.insert(key.to_string(), inner);
        }
    }

    pub fn cardinality(&self, key: impl AsRef<str>) -> usize {
        if let Some(set) = self.store.get(key.as_ref()) {
            return set.len();
        }

        return 0;
    }

    pub fn diff(&self, lhs: impl AsRef<str>, rhs: impl AsRef<str>) -> HashSet<StoreEntry> {
        let ph = HashSet::new();
        let left = self.store.get(lhs.as_ref()).unwrap_or(&ph);
        let right = self.store.get(rhs.as_ref()).unwrap_or(&ph);

        let result = left.difference(right);
        return result
            .into_iter()
            .map(|e| e.clone())
            .collect::<HashSet<StoreEntry>>();
    }
}

/*
* Add - DONE
* Cardinality - Number of items in a set DONE
* Difference - Set difference
* DiffStore - Store difference between sets in a new entry
* Intersection - Intersection of different sets
* InterStore - Intersection but store result
* IsMember - Check if an entry is in the given set
* Members - Return all items in a set
* Move - Move item from one set to another
* Pop - Remove X items from the store
* RandMember
*   - +ve Returns up to X random elements or Cardinality (whichever is lower)
*   - -ve Returns random elements up to the count (can be the same elements)
*
* Remove - Remove memberd from the set
* Union - Give the union of the given sets
* UnionStore - Union but store the result
*/

#[cfg(test)]
mod set_tests {
    use super::*;

    fn create_kv_pair(key: impl AsRef<str>, value: impl AsRef<str>) -> KVPair {
        return (key.as_ref().to_string(), StoreEntry::new(value));
    }

    #[test]
    fn add_items() {
        let mut ss = SetStore::new();

        let entries = (0..100)
            .into_iter()
            .map(|n| create_kv_pair("test", n.to_string()))
            .collect::<Vec<KVPair>>();

        for entry in entries.iter() {
            ss.add(entry.clone());
        }

        assert_eq!(ss.cardinality("test"), 100);
    }

    #[test]
    fn check_cardinality() {
        let mut ss = SetStore::new();
        assert_eq!(ss.cardinality("test"), 0);

        for i in (0..3).into_iter() {
            let kv = create_kv_pair("test", i.to_string());
            ss.add(kv);
        }

        assert_eq!(ss.cardinality("test"), 3);
    }

    #[test]
    fn difference() {
        let mut ss = SetStore::new();
    }
}
