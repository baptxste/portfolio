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
        div { class: "container mx-auto px-2 py-4 max-w-4xl",
            if let Some(note) = note_opt {
                article { class: "bg-white p-6 sm:p-10 rounded-[12px] sb-card-shadow",
                    header { class: "mb-8 pb-6 border-b border-[#edebe9]",
                        h1 { class: "text-3xl sm:text-4xl font-bold text-[#006241] mb-2 tracking-tight",
                            "{note.title}"
                        }
                        if let Some(date) = note.date {
                            p { class: "text-xs text-[rgba(0,0,0,0.58)] mb-4 font-medium", "Publié le {date}" }
                        }
                        if !note.tags.is_empty() {
                            div { class: "flex flex-wrap gap-2 mt-4",
                                for tag in note.tags {
                                    Link {
                                        to: Route::TagPage { tag: tag.clone() },
                                        class: "px-3 py-1 text-xs font-semibold bg-[#d4e9e2] text-[#006241] rounded-full hover:bg-[#00754A] hover:text-white transition duration-150",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }

                    // Render converted Markdown HTML
                    div {
                        class: "markdown-body mt-6 leading-relaxed text-[rgba(0,0,0,0.87)]",
                        dangerous_inner_html: "{note.html}",
                    }

                    // Render Backlinks if any
                    if !note.backlinks.is_empty() {
                        section { class: "mt-12 pt-8 border-t border-[#edebe9]",
                            h3 { class: "text-lg font-bold text-[#006241] mb-4 flex items-center gap-2",
                                span { "🔗" }
                                "Backlinks (Notes liées)"
                            }
                            ul { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                for backlink in note.backlinks {
                                    li {
                                        Link {
                                            to: Route::NotePage { slug: backlink.slug.clone() },
                                            class: "block p-4 rounded-[12px] bg-[#f9f9f9] hover:bg-[#d4e9e2] transition duration-200 group border border-slate-200/60",
                                            span { class: "font-semibold text-[#00754A] group-hover:text-[#006241] transition text-sm",
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
                div { class: "text-center py-16 bg-white rounded-[12px] p-8 sb-card-shadow",
                    h1 { class: "text-3xl font-bold text-[#006241] mb-4", "404 - Note introuvable" }
                    p { class: "text-[rgba(0,0,0,0.58)] text-sm mb-6", "La note '{slug}' n'existe pas ou n'a pas été publiée." }
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
