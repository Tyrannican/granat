pub mod entry;
pub mod general;
pub mod list;
pub mod set;

use entry::StoreEntry;

pub type KVPair = (String, StoreEntry);
