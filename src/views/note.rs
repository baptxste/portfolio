use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::Route;

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
        div { class: "container mx-auto px-4 py-8 max-w-4xl",
            if let Some(note) = note_opt {
                article { class: "prose dark:prose-invert max-w-none bg-white dark:bg-gray-800 p-8 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700",
                    header { class: "mb-8 pb-4 border-b border-gray-200 dark:border-gray-700",
                        h1 { class: "text-4xl font-extrabold text-gray-900 dark:text-white mb-2",
                            "{note.title}"
                        }
                        if let Some(date) = note.date {
                            p { class: "text-sm text-gray-500 dark:text-gray-400", "Publié le {date}" }
                        }
                        if !note.tags.is_empty() {
                            div { class: "flex flex-wrap gap-2 mt-4",
                                for tag in note.tags {
                                    Link {
                                        to: Route::TagPage { tag: tag.clone() },
                                        class: "px-2.5 py-1 text-xs font-medium bg-indigo-50 dark:bg-indigo-950 text-indigo-600 dark:text-indigo-300 rounded-full hover:bg-indigo-100 dark:hover:bg-indigo-900 transition",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }

                    // Render converted Markdown HTML
                    div {
                        class: "markdown-body mt-6 leading-relaxed text-gray-700 dark:text-gray-200",
                        dangerous_inner_html: "{note.html}",
                    }

                    // Render Backlinks if any
                    if !note.backlinks.is_empty() {
                        section { class: "mt-12 pt-6 border-t border-gray-200 dark:border-gray-700",
                            h3 { class: "text-lg font-bold text-gray-900 dark:text-white mb-4",
                                "Backlinks (Notes liées)"
                            }
                            ul { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                for backlink in note.backlinks {
                                    li {
                                        Link {
                                            to: Route::NotePage { slug: backlink.slug.clone() },
                                            class: "block p-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:border-indigo-500 hover:bg-indigo-50/50 dark:hover:bg-gray-700/50 transition",
                                            span { class: "font-semibold text-indigo-600 dark:text-indigo-400",
                                                "← {backlink.title}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "text-center py-16",
                    h1 { class: "text-4xl font-bold text-gray-800 dark:text-gray-200 mb-4", "404 - Note introuvable" }
                    p { class: "text-gray-600 dark:text-gray-400 mb-6", "La note '{slug}' n'existe pas ou n'a pas été publiée." }
                    Link {
                        to: Route::Home {},
                        class: "inline-block px-5 py-2.5 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition",
                        "Retourner à l'accueil"
                    }
                }
            }
        }
    }
}
