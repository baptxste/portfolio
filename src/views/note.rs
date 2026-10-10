use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::Route;
use crate::components::obsidian_note::NoteObsidian;

#[component]
pub fn NotePage(slug: String) -> Element {
    let index: VaultIndex = use_context();

    let slugified = slug::slugify(&slug);
    let note_opt = index
        .get(&slug)
        .or_else(|| index.get(&slugified))
        .cloned();



    rsx! {
        if let Some(note) = note_opt {
            div { class: "main-content-card",
                NoteObsidian { note: note.clone() }
            }
        } else {
            div { class: "main-content-card",
                crate::views::NotFound { route: vec!["notes".to_string(), slug.clone()] }
            }
        }
    }
}