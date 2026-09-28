use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::Route;

#[component]
pub fn TagPage(tag: String) -> Element {
    let index: VaultIndex = use_context();

    let matching_notes: Vec<_> = index
        .values()
        .filter(|note| note.tags.iter().any(|t| t.eq_ignore_ascii_case(&tag)))
        .cloned()
        .collect();

    rsx! {
        div { class: "notes-home-container",
            header { class: "notes-header",
                div { class: "hero-badge",
                    "Tag"
                }
                h1 {
                    "Notes marquées #{tag}"
                }
            }

            if matching_notes.is_empty() {
                p { class: "empty-notes-msg",
                    "Aucune note trouvée pour ce tag."
                }
            } else {
                div { class: "notes-grid",
                    for note in matching_notes {
                        Link {
                            to: Route::NotePage { slug: note.slug.clone() },
                            class: "note-card-item",
                            div {
                                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;",
                                    h3 { "{note.title}" }
                                    if let Some(date) = &note.date {
                                        span { style: "font-size: 0.75rem; color: var(--text-soft); white-space: nowrap; margin-left: 0.5rem;", "{date}" }
                                    }
                                }
                                p { "{note.summary}..." }
                            }
                        }
                    }
                }
            }
        }
    }
}
