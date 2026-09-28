use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::i18n::{tr, Language};
use crate::Route;
use std::collections::HashSet;

#[component]
pub fn NotesHome() -> Element {
    let index: VaultIndex = use_context();
    let lang: Signal<Language> = use_context();
    let l = lang();
    let mut search_query = use_signal(|| String::new());

    let notes: Vec<_> = index.values().cloned().collect();

    // Extract all unique tags
    let mut all_tags: Vec<String> = notes
        .iter()
        .flat_map(|n| n.tags.iter().cloned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    all_tags.sort();

    // Filter notes based on search query
    let filtered_notes: Vec<_> = notes
        .into_iter()
        .filter(|n| {
            let query = search_query.read().to_lowercase();
            if query.is_empty() {
                return true;
            }
            n.title.to_lowercase().contains(&query)
                || n.summary.to_lowercase().contains(&query)
                || n.tags.iter().any(|t| t.to_lowercase().contains(&query))
        })
        .collect();

    rsx! {
        div { class: "notes-home-container",
            // Header / Title Section
            header { class: "notes-header",
                h1 { {tr(l, "Jardin Numérique & Vault Obsidian", "Digital Garden & Obsidian Vault")} }
                p { {tr(l, "Explorez l'ensemble des notes, théorèmes et articles. Utilisez la recherche ou l'arborescence à gauche.", "Explore all notes, theorems, and articles. Use search or the file tree on the left.")} }
            }

            // Search Bar & Filter Section
            div { class: "search-filter-card",
                div { class: "search-input-wrapper",
                    input {
                        r#type: "text",
                        placeholder: tr(l, "Rechercher une note, un sujet, un tag...", "Search for a note, subject, tag..."),
                        class: "search-input",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }

                if !all_tags.is_empty() {
                    div { class: "tags-filter-list",
                        span { class: "tags-filter-label", "Tags:" }
                        for tag in all_tags {
                            Link {
                                to: Route::TagPage { tag: tag.clone() },
                                class: "tag-pill",
                                "#{tag}"
                            }
                        }
                    }
                }
            }

            // Notes List Grid
            div { class: "notes-grid",
                if filtered_notes.is_empty() {
                    div { class: "empty-notes-msg",
                        {tr(l, "Aucune note ne correspond à votre recherche.", "No notes match your search query.")}
                    }
                } else {
                    for note in filtered_notes {
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
                            if !note.tags.is_empty() {
                                div { class: "tags-row",
                                    for tag in &note.tags {
                                        span { class: "tag-pill",
                                            "#{tag}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
