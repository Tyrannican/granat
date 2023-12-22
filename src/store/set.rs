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

    pub fn diff(&self, target: impl AsRef<str>, comps: Vec<impl AsRef<str>>) -> Vec<StoreEntry> {
        let mut combo = HashSet::new();
        let ph = HashSet::new();
        let target_set = self.store.get(target.as_ref()).unwrap_or(&ph);

        for comp in comps.iter() {
            let comp_set = self.store.get(comp.as_ref()).unwrap_or(&ph);
            combo.extend(comp_set.clone().into_iter());
        }

        let diff = target_set.difference(&combo);

        return diff
            .into_iter()
            .map(|e| e.to_owned())
            .collect::<Vec<StoreEntry>>();
    }

    pub fn diff_store(
        &mut self,
        dst: impl AsRef<str>,
        target: impl AsRef<str>,
        comps: Vec<impl AsRef<str>>,
    ) {
        let diffs = self.diff(target, comps);
        let hs = diffs.into_iter().collect::<HashSet<StoreEntry>>();
        if !hs.is_empty() {
            self.store.insert(dst.as_ref().to_string(), hs);
        }
    }
}

/*
* Add - DONE
* Cardinality - Number of items in a set DONE
* Difference - Set difference DONE
* DiffStore - Store difference between sets in a new entry DONE
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
        let convert = |diff: &Vec<StoreEntry>| {
            let mut result = diff
                .iter()
                .map(|i| i.value.clone())
                .collect::<Vec<String>>();
            result.sort();

            result
        };

        // Single diff
        ss.add(create_kv_pair("singlediff1", "a"));
        ss.add(create_kv_pair("singlediff1", "b"));
        ss.add(create_kv_pair("singlediff1", "c"));
        ss.add(create_kv_pair("singlediff2", "a"));
        ss.add(create_kv_pair("singlediff2", "c"));
        ss.add(create_kv_pair("singlediff2", "e"));

        let diff = ss.diff("singlediff1", vec!["singlediff2"]);
        let result = convert(&diff);
        assert_eq!(result, vec!["b"]);

        // Multi Diff
        ss.add(create_kv_pair("multidiff1", "a"));
        ss.add(create_kv_pair("multidiff1", "b"));
        ss.add(create_kv_pair("multidiff1", "c"));
        ss.add(create_kv_pair("multidiff1", "d"));
        ss.add(create_kv_pair("multidiff2", "c"));
        ss.add(create_kv_pair("multidiff3", "a"));
        ss.add(create_kv_pair("multidiff3", "c"));
        ss.add(create_kv_pair("multidiff3", "e"));

        let diff = ss.diff("multidiff1", vec!["multidiff2", "multidiff3"]);
        let result = convert(&diff);
        assert_eq!(result, vec!["b".to_string(), "d".to_string()]);

        // No comparisons
        ss.add(create_kv_pair("nocomp1", "a"));

        let diff = ss.diff("nocomp1", vec!["nocomp2"]);
        let result = convert(&diff);
        assert_eq!(result, vec!["a"]);

        let diff = ss.diff("nocomp2", vec!["nocomp1"]);
        let result = convert(&diff);
        assert!(result.is_empty());

        // Empty
        let diff = ss.diff("empty1", vec!["empty2"]);
        assert!(diff.is_empty());
    }

    #[test]
    fn diffstore() {
        let mut ss = SetStore::new();

        ss.add(create_kv_pair("table1", "a"));
        ss.add(create_kv_pair("table1", "b"));
        ss.add(create_kv_pair("table1", "c"));
        ss.add(create_kv_pair("table2", "a"));
        ss.add(create_kv_pair("table2", "c"));
        ss.add(create_kv_pair("table2", "e"));

        ss.diff_store("table3", "table1", vec!["table2"]);
        assert!(ss.store.contains_key("table3"));

        let hs = ss.store.get("table3").unwrap();
        assert_eq!(hs.len(), 1);
        assert!(hs.contains(&StoreEntry::new("b".to_string())));
    }
}
