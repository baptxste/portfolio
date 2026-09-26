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
        div { class: "container mx-auto px-2 py-4 max-w-4xl space-y-6",
            header { class: "pb-4 border-b border-[var(--border-subtle)]",
                div { class: "inline-block mb-2 px-3 py-1 bg-[#d4e9e2] dark:bg-[#1e3f36] text-[#006241] dark:text-[#00ff9d] text-xs font-bold rounded-full uppercase tracking-wider",
                    "Tag"
                }
                h1 { class: "text-3xl font-semibold theme-text-heading tracking-tight",
                    "Notes marquées #{tag}"
                }
            }

            if matching_notes.is_empty() {
                p { class: "theme-text-soft text-sm italic py-8 text-center theme-bg-card rounded-[12px] sb-card-shadow",
                    "Aucune note trouvée pour ce tag."
                }
            } else {
                ul { class: "space-y-4",
                    for note in matching_notes {
                        li { class: "p-6 theme-bg-card rounded-[12px] sb-card-shadow hover:shadow-md transition duration-200 group",
                            div { class: "flex justify-between items-start mb-2",
                                Link {
                                    to: Route::NotePage { slug: note.slug.clone() },
                                    class: "text-xl font-semibold theme-text-main group-hover:text-[#00754A] dark:group-hover:text-[#00ff9d] transition",
                                    "{note.title}"
                                }
                                if let Some(date) = &note.date {
                                    span { class: "ml-3 text-xs theme-text-soft", "{date}" }
                                }
                            }
                            p { class: "mt-2 text-xs theme-text-soft leading-relaxed line-clamp-2",
                                "{note.summary}..."
                            }
                        }
                    }
                }
            }
        }
    }
}
