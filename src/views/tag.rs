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
        div { class: "container mx-auto px-4 py-8 max-w-4xl",
            h1 { class: "text-3xl font-bold mb-6 text-gray-900 dark:text-white border-b pb-2",
                "Notes marquées avec #{tag}"
            }

            if matching_notes.is_empty() {
                p { class: "text-gray-500 dark:text-gray-400 italic",
                    "Aucune note trouvée pour ce tag."
                }
            } else {
                ul { class: "space-y-4",
                    for note in matching_notes {
                        li { class: "p-4 bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-100 dark:border-gray-700",
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "text-xl font-semibold text-indigo-600 dark:text-indigo-400 hover:underline",
                                "{note.title}"
                            }
                            if let Some(date) = &note.date {
                                span { class: "ml-3 text-xs text-gray-400", "{date}" }
                            }
                            p { class: "mt-2 text-sm text-gray-600 dark:text-gray-300",
                                "{note.summary}..."
                            }
                        }
                    }
                }
            }
        }
    }
}
