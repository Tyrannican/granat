pub mod general;
pub mod entry;
pub mod list;

pub type KVPair = (String, String);

#[derive(Default, Debug, PartialEq)]
pub enum StoreMode {
    /// Do things as normal
    #[default]
    Normal,

    /// Only perform operation if key already exists
    Safe
}

