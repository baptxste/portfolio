use dioxus::prelude::*;
use crate::content::VaultIndex;
use crate::i18n::{tr, Language};
use crate::Route;

#[component]
pub fn Home() -> Element {
    let index: VaultIndex = use_context();
    let lang: Signal<Language> = use_context();
    let l = lang();

    let mut notes: Vec<_> = index.values().cloned().collect();
    notes.truncate(4);

    rsx! {
        div { class: "container mx-auto px-4 py-8 max-w-5xl space-y-16",
            // Hero Section
            section { class: "text-center py-12 md:py-20 rounded-2xl bg-gradient-to-br from-indigo-50 via-white to-purple-50 dark:from-gray-800 dark:via-gray-900 dark:to-indigo-950 border border-indigo-100/50 dark:border-gray-800 shadow-sm p-8 md:p-12",
                div { class: "inline-block mb-4 px-3 py-1 bg-indigo-100 dark:bg-indigo-900/60 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-full uppercase tracking-wider",
                    {tr(l, "Portfolio & Notes", "Portfolio & Notes")}
                }
                h1 { class: "text-4xl md:text-6xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6",
                    "Baptiste Chachura"
                }
                p { class: "text-lg md:text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto leading-relaxed mb-8",
                    {tr(l, "Développeur & étudiant passionné par l'informatique, l'écosystème Rust, le web moderne et les mathématiques.", "Software engineer & student passionate about computer science, the Rust ecosystem, modern web, and mathematics.")}
                }
                div { class: "flex flex-wrap gap-4 justify-center items-center",
                    Link {
                        to: Route::NotesHome {},
                        class: "px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-xl shadow-sm hover:shadow transition duration-200 flex items-center gap-2",
                        {tr(l, "Explorer mes Notes 📚", "Explore my Notes 📚")}
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "px-6 py-3 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 font-medium rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm transition duration-200 flex items-center gap-2",
                        {tr(l, "Consulter mon CV 📄", "View my Resume 📄")}
                    }
                }
            }

            // Quick Overview Cards
            section { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                div { class: "p-8 bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 shadow-sm hover:border-indigo-200 dark:hover:border-indigo-800 transition",
                    div { class: "w-12 h-12 rounded-xl bg-indigo-100 dark:bg-indigo-950 text-indigo-600 dark:text-indigo-400 flex items-center justify-center text-2xl font-bold mb-6",
                        "📓"
                    }
                    h2 { class: "text-2xl font-bold text-gray-900 dark:text-white mb-3",
                        {tr(l, "Jardin Numérique", "Digital Garden")}
                    }
                    p { class: "text-gray-600 dark:text-gray-300 leading-relaxed mb-6",
                        {tr(l, "Mon coffre de connaissances Obsidian rendu dynamiquement en Rust via Dioxus. Retrouvez des cours, réflexions, théorèmes et résumés techniques.", "My Obsidian knowledge vault rendered dynamically in Rust via Dioxus. Explore notes, thoughts, theorems, and technical summaries.")}
                    }
                    Link {
                        to: Route::NotesHome {},
                        class: "inline-flex items-center text-sm font-semibold text-indigo-600 dark:text-indigo-400 hover:underline gap-1",
                        {tr(l, "Accéder au vault →", "Open the vault →")}
                    }
                }

                div { class: "p-8 bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 shadow-sm hover:border-indigo-200 dark:hover:border-indigo-800 transition",
                    div { class: "w-12 h-12 rounded-xl bg-purple-100 dark:bg-purple-950 text-purple-600 dark:text-purple-400 flex items-center justify-center text-2xl font-bold mb-6",
                        "🎓"
                    }
                    h2 { class: "text-2xl font-bold text-gray-900 dark:text-white mb-3",
                        {tr(l, "Parcours & Compétences", "Background & Skills")}
                    }
                    p { class: "text-gray-600 dark:text-gray-300 leading-relaxed mb-6",
                        {tr(l, "Découvrez mon curriculum vitae, mes formations académiques, compétences techniques et projets réalisés.", "Discover my curriculum vitae, academic background, technical skills, and projects.")}
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "inline-flex items-center text-sm font-semibold text-indigo-600 dark:text-indigo-400 hover:underline gap-1",
                        {tr(l, "Voir le CV & télécharger le PDF →", "View Resume & download PDF →")}
                    }
                }
            }

            // Featured Notes Preview
            if !notes.is_empty() {
                section { class: "space-y-6",
                    div { class: "flex justify-between items-end border-b border-gray-100 dark:border-gray-800 pb-4",
                        div {
                            h2 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                                {tr(l, "Aperçu des Notes", "Notes Preview")}
                            }
                            p { class: "text-sm text-gray-500 dark:text-gray-400",
                                {tr(l, "Dernières notes synchronisées", "Recently synchronized notes")}
                            }
                        }
                        Link {
                            to: Route::NotesHome {},
                            class: "text-sm font-semibold text-indigo-600 dark:text-indigo-400 hover:underline",
                            {tr(l, "Tout voir →", "See all →")}
                        }
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        for note in notes {
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "group block p-6 bg-white dark:bg-gray-800 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm hover:shadow-md hover:border-indigo-200 dark:hover:border-indigo-800 transition",
                                h3 { class: "text-lg font-bold text-gray-900 dark:text-white group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition mb-2",
                                    "{note.title}"
                                }
                                p { class: "text-sm text-gray-600 dark:text-gray-300 line-clamp-2 mb-4",
                                    "{note.summary}..."
                                }
                                if !note.tags.is_empty() {
                                    div { class: "flex flex-wrap gap-1.5",
                                        for tag in &note.tags {
                                            span { class: "text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300",
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
}
