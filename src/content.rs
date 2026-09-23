use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NoteMetaData {
    pub slug: String,
    pub title: String,
    pub relative_path: String,
    pub folder: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub summary: String,
    pub html: String,
    pub backlinks: Vec<BacklinkInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BacklinkInfo {
    pub slug: String,
    pub title: String,
}

pub type VaultIndex = HashMap<String, NoteMetaData>;

const INDEX_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/index.json"));

pub fn get_vault_index() -> VaultIndex {
    serde_json::from_str(INDEX_JSON).unwrap_or_default()
}
