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
        div { class: "container mx-auto px-2 py-4 max-w-5xl space-y-8",
            // Header / Title Section
            header { class: "pb-4 border-b border-[var(--border-subtle)]",
                h1 { class: "text-3xl font-semibold tracking-tight theme-text-heading mb-2",
                    {tr(l, "Jardin Numérique & Vault Obsidian", "Digital Garden & Obsidian Vault")}
                }
                p { class: "theme-text-soft text-sm max-w-2xl leading-relaxed",
                    {tr(l, "Explorez l'ensemble des notes, théorèmes et articles. Utilisez la recherche ou l'arborescence à gauche.", "Explore all notes, theorems, and articles. Use search or the file tree on the left.")}
                }
            }

            // Search Bar & Filter Section - Theme Card
            div { class: "flex flex-col md:flex-row gap-4 justify-between items-center theme-bg-card p-5 rounded-[12px] sb-card-shadow transition-colors duration-250",
                div { class: "relative w-full md:w-96",
                    input {
                        r#type: "text",
                        placeholder: tr(l, "Rechercher une note, un sujet, un tag...", "Search for a note, subject, tag..."),
                        class: "w-full pl-4 pr-10 py-2.5 rounded-full border border-slate-300 dark:border-[#24463e] theme-bg-subtle theme-text-main placeholder:text-slate-400 text-sm focus:border-[#00754A] focus:outline-none transition shadow-sm",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }

                if !all_tags.is_empty() {
                    div { class: "flex flex-wrap gap-1.5 items-center",
                        span { class: "text-xs font-bold theme-text-soft uppercase tracking-wider mr-1", "Tags:" }
                        for tag in all_tags {
                            Link {
                                to: Route::TagPage { tag: tag.clone() },
                                class: "px-3 py-1 text-xs font-semibold bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] rounded-full hover:bg-[#00754A] hover:text-white transition duration-150",
                                "#{tag}"
                            }
                        }
                    }
                }
            }

            // Notes List Grid - Theme Cards
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                if filtered_notes.is_empty() {
                    div { class: "col-span-2 text-center py-12 theme-text-soft text-sm",
                        {tr(l, "Aucune note ne correspond à votre recherche.", "No notes match your search query.")}
                    }
                } else {
                    for note in filtered_notes {
                        article { class: "flex flex-col justify-between p-6 theme-bg-card rounded-[12px] sb-card-shadow hover:shadow-md transition duration-200 group",
                            div {
                                div { class: "flex justify-between items-start mb-2",
                                    Link {
                                        to: Route::NotePage { slug: note.slug.clone() },
                                        class: "text-lg font-semibold theme-text-main group-hover:text-[#00754A] dark:group-hover:text-[#d4e9e2] transition line-clamp-1",
                                        "{note.title}"
                                    }
                                    if let Some(date) = &note.date {
                                        span { class: "text-xs theme-text-soft whitespace-nowrap ml-2", "{date}" }
                                    }
                                }
                                p { class: "text-xs theme-text-soft mb-4 line-clamp-3 leading-relaxed",
                                    "{note.summary}..."
                                }
                            }
                            if !note.tags.is_empty() {
                                div { class: "flex flex-wrap gap-1.5 pt-3 border-t border-[var(--border-subtle)]",
                                    for tag in &note.tags {
                                        Link {
                                            to: Route::TagPage { tag: tag.clone() },
                                            class: "text-xs font-medium text-[#00754A] dark:text-[#80c7b3] hover:underline",
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
