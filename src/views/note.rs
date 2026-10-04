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

    let current_slug = slug.clone();
    use_effect(use_reactive!(|(current_slug,)| {
        let _ = current_slug;
        let _ = document::eval(
            r#"
            setTimeout(() => {
                if (window.renderMathInElement) {
                    renderMathInElement(document.body, {
                        delimiters: [
                            {left: '$$', right: '$$', display: true},
                            {left: '$', right: '$', display: false}
                        ],
                        throwOnError: false
                    });
                }
            }, 50);
            "#
        );
    }));

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