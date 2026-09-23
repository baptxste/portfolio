use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::Route;
use std::collections::HashSet;

#[component]
pub fn NotesHome() -> Element {
    let index: VaultIndex = use_context();
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
        div { class: "container mx-auto px-4 py-4 max-w-5xl",
            // Header / Title Section
            header { class: "mb-6 pb-4 border-b border-gray-100 dark:border-gray-800",
                h1 { class: "text-3xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-2",
                    "Jardin Numérique & Vault Obsidian"
                }
                p { class: "text-gray-600 dark:text-gray-300 text-sm",
                    "Explorez l'ensemble des notes, théorèmes et articles. Utilisez la recherche ou l'arborescence à gauche."
                }
            }

            // Search Bar & Filter Section
            div { class: "mb-8 flex flex-col md:flex-row gap-4 justify-between items-center bg-white dark:bg-gray-800 p-4 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700",
                div { class: "relative w-full md:w-96",
                    input {
                        r#type: "text",
                        placeholder: "Rechercher une note, un sujet, un tag...",
                        class: "w-full pl-4 pr-10 py-2.5 rounded-lg border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-indigo-500 focus:outline-none transition",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }

                if !all_tags.is_empty() {
                    div { class: "flex flex-wrap gap-1.5 items-center",
                        span { class: "text-xs font-semibold text-gray-400 mr-1 uppercase tracking-wider", "Tags:" }
                        for tag in all_tags {
                            Link {
                                to: Route::TagPage { tag: tag.clone() },
                                class: "px-2.5 py-1 text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-indigo-600 hover:text-white transition",
                                "#{tag}"
                            }
                        }
                    }
                }
            }

            // Notes List Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                if filtered_notes.is_empty() {
                    div { class: "col-span-2 text-center py-12 text-gray-500 dark:text-gray-400",
                        "Aucune note ne correspond à votre recherche."
                    }
                } else {
                    for note in filtered_notes {
                        article { class: "flex flex-col justify-between p-6 bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700 hover:shadow-md transition",
                            div {
                                div { class: "flex justify-between items-start mb-2",
                                    Link {
                                        to: Route::NotePage { slug: note.slug.clone() },
                                        class: "text-xl font-bold text-gray-900 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition",
                                        "{note.title}"
                                    }
                                    if let Some(date) = &note.date {
                                        span { class: "text-xs text-gray-400 whitespace-nowrap ml-2", "{date}" }
                                    }
                                }
                                p { class: "text-sm text-gray-600 dark:text-gray-300 mb-4 line-clamp-3",
                                    "{note.summary}..."
                                }
                            }
                            if !note.tags.is_empty() {
                                div { class: "flex flex-wrap gap-1.5 pt-4 border-t border-gray-100 dark:border-gray-700/50",
                                    for tag in &note.tags {
                                        Link {
                                            to: Route::TagPage { tag: tag.clone() },
                                            class: "text-xs font-medium text-indigo-600 dark:text-indigo-400 hover:underline",
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
