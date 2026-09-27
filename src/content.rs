use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::components::obsidian_note::model::{NoteMetaData, BacklinkInfo};

pub type VaultIndex = HashMap<String, NoteMetaData>;

const INDEX_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/index.json"));

pub fn get_vault_index() -> VaultIndex {
    serde_json::from_str(INDEX_JSON).unwrap_or_default()
}
