pub mod entry;
pub mod general;
pub mod list;

use entry::StoreEntry;
use general::GeneralStore;
use list::ListStore;

pub type KVPair = (String, StoreEntry);

// The Idea:
//
// 1. You can use this sync
//      * Just add to appropriate store
//      * Retrieve from store
// 2. You can use async
//      * Some kind of queue system?
//      * Means things are _eventually_ consistent
//      * i.e. queue an update and process it accordingly
//      * Might need a separate worker thread to pull from the queue?
pub struct GranatStore {
    general: GeneralStore,
    list: ListStore,
}

impl GranatStore {
    pub fn new() -> Self {
        Self {
            general: GeneralStore::new(),
            list: ListStore::new(),
        }
    }
}
