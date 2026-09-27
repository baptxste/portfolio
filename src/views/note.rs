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
        div { class: "container mx-auto px-2 py-4 max-w-4xl",
            if let Some(note) = note_opt {
                article { class: "theme-bg-card p-6 sm:p-10 rounded-[12px] sb-card-shadow transition-colors duration-250",
                    header { class: "mb-8 pb-6 border-b border-[var(--border-subtle)]",
                        h1 { class: "text-3xl sm:text-4xl font-bold theme-text-heading mb-2 tracking-tight",
                            "{note.title}"
                        }
                        if let Some(date) = &note.date {
                            p { class: "text-xs theme-text-soft mb-4 font-medium", "Publié le {date}" }
                        }
                        if !note.tags.is_empty() {
                            div { class: "flex flex-wrap gap-2 mt-4",
                                for tag in &note.tags {
                                    Link {
                                        to: Route::TagPage { tag: tag.clone() },
                                        class: "px-3 py-1 text-xs font-semibold bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] rounded-full hover:bg-[#00754A] hover:text-white transition duration-150",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }

                    // Render converted Markdown HTML
                    div {
                        class: "markdown-body mt-6 leading-relaxed theme-text-main",
                        NoteObsidian { note: note.clone() }
                    }

                    // Render Backlinks if any
                    if !note.backlinks.is_empty() {
                        section { class: "mt-12 pt-8 border-t border-[var(--border-subtle)]",
                            h3 { class: "text-lg font-bold theme-text-heading mb-4 flex items-center gap-2",
                                span { "🔗" }
                                "Backlinks (Notes liées)"
                            }
                            ul { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                for backlink in &note.backlinks {
                                    li {
                                        Link {
                                            to: Route::NotePage { slug: backlink.slug.clone() },
                                            class: "block p-4 rounded-[12px] theme-bg-subtle hover:bg-[#d4e9e2] dark:hover:bg-[#24463e] transition duration-200 group border border-[var(--border-subtle)]",
                                            span { class: "font-semibold text-[#00754A] dark:text-[#80c7b3] group-hover:text-[#006241] dark:group-hover:text-[#d4e9e2] transition text-sm",
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
                div { class: "text-center py-16 theme-bg-card rounded-[12px] p-8 sb-card-shadow",
                    h1 { class: "text-3xl font-bold theme-text-heading mb-4", "404 - Note introuvable" }
                    p { class: "theme-text-soft text-sm mb-6", "La note '{slug}' n'existe pas ou n'a pas été publiée." }
                    Link {
                        to: Route::Home {},
                        class: "inline-block px-6 py-3 sb-pill-green text-sm shadow-sm",
                        "Retourner à l'accueil"
                    }
                }
            }
        }
    }
}