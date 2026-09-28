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
        div { class: "notes-home-container",
            if let Some(note) = note_opt {
                div { class: "main-content-card",
                    NoteObsidian { note: note.clone() }
                }
            } else {
                div { class: "main-content-card", style: "text-align: center; padding: 4rem 1rem;",
                    h1 { style: "font-size: 1.875rem; font-weight: 700; color: var(--text-heading); margin-bottom: 1rem;", "404 - Note introuvable" }
                    p { style: "color: var(--text-soft); font-size: 0.875rem; margin-bottom: 1.5rem;", "La note '{slug}' n'existe pas ou n'a pas été publiée." }
                    Link {
                        to: Route::NotesHome {},
                        class: "sb-pill-green",
                        style: "padding: 0.75rem 1.5rem; display: inline-block;",
                        "Retourner aux notes"
                    }
                }
            }
        }
    }
}