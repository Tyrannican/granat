pub mod entry;
pub mod general;
pub mod list;

use entry::Entry;

pub type KVPair = (String, Entry);

#[derive(Default, Debug, PartialEq)]
pub enum StoreMode {
    /// Do things as normal
    #[default]
    Normal,

    /// Only perform operation if key already exists
    Safe,
}
