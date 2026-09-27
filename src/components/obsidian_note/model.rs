#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NoteMetaData {
    pub slug: String,
    pub title: String,
    pub relative_path: String,
    pub folder: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub summary: String,
    pub html: String,
    /// Noms (en minuscules, tels qu'écrits dans les `![[...]]` Obsidian) des
    /// images référencées par cette note. Sert à NoteObsidian pour savoir
    /// quels jetons `vault-asset:...` résoudre dans `html`, sans avoir à
    /// rescanner tout le HTML à chaque rendu.
    pub images: Vec<String>,
    pub backlinks: Vec<BacklinkInfo>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BacklinkInfo {

    pub slug: String,
    pub title: String,
}