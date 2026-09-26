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
        div { class: "container mx-auto px-2 py-4 max-w-5xl space-y-12",
            // Hero Section - Starbucks Warm Cream / Dark House Green Hero
            section { class: "text-center py-12 md:py-16 px-6 md:px-12 theme-bg-surface rounded-[24px] sb-card-shadow relative overflow-hidden transition-colors duration-250",
                div { class: "inline-block mb-4 px-4 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-bold rounded-full tracking-wide uppercase",
                    {tr(l, "Portfolio & Jardin Numérique", "Portfolio & Digital Garden")}
                }
                h1 { class: "text-4xl sm:text-5xl md:text-6xl font-semibold tracking-tight theme-text-heading mb-6",
                    "Baptiste Chachura"
                }
                p { class: "text-lg md:text-xl theme-text-soft max-w-2xl mx-auto leading-relaxed mb-8 font-normal",
                    {tr(l, "Développeur & étudiant passionné par l'architecture logicielle, l'écosystème Rust, le web moderne et les mathématiques.", "Software engineer & student passionate about software architecture, the Rust ecosystem, modern web, and mathematics.")}
                }
                div { class: "flex flex-wrap gap-4 justify-center items-center",
                    Link {
                        to: Route::NotesHome {},
                        class: "px-7 py-3.5 sb-pill-green text-sm flex items-center gap-2.5 shadow-sm",
                        {tr(l, "Explorer mes Notes 📚", "Explore my Notes 📚")}
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "px-7 py-3.5 sb-pill-outline text-sm flex items-center gap-2.5 theme-bg-card",
                        {tr(l, "Consulter mon CV 📄", "View my Resume 📄")}
                    }
                }
            }

            // Quick Overview Cards - Semantic 12px Theme Cards
            section { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                div { class: "p-8 theme-bg-card rounded-[12px] sb-card-shadow hover:shadow-md transition duration-200 flex flex-col justify-between group",
                    div {
                        div { class: "w-12 h-12 rounded-full bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] flex items-center justify-center text-2xl font-bold mb-6 group-hover:scale-105 transition transform duration-200",
                            "📓"
                        }
                        h2 { class: "text-2xl font-semibold theme-text-main mb-3 group-hover:text-[#006241] dark:group-hover:text-[#d4e9e2] transition",
                            {tr(l, "Jardin Numérique", "Digital Garden")}
                        }
                        p { class: "theme-text-soft leading-relaxed mb-6 text-sm",
                            {tr(l, "Mon coffre de connaissances Obsidian rendu dynamiquement en Rust via Dioxus. Retrouvez des cours, réflexions, théorèmes et résumés techniques.", "My Obsidian knowledge vault rendered dynamically in Rust via Dioxus. Explore notes, thoughts, theorems, and technical summaries.")}
                        }
                    }
                    Link {
                        to: Route::NotesHome {},
                        class: "inline-flex items-center text-sm font-semibold text-[#00754A] dark:text-[#80c7b3] hover:text-[#006241] dark:hover:text-[#d4e9e2] gap-1 group-hover:translate-x-1 transition duration-200",
                        {tr(l, "Accéder au vault →", "Open the vault →")}
                    }
                }

                div { class: "p-8 theme-bg-card rounded-[12px] sb-card-shadow hover:shadow-md transition duration-200 flex flex-col justify-between group",
                    div {
                        div { class: "w-12 h-12 rounded-full bg-[#faf6ee] dark:bg-[#282218] text-[#cba258] flex items-center justify-center text-2xl font-bold mb-6 group-hover:scale-105 transition transform duration-200",
                            "🎓"
                        }
                        h2 { class: "text-2xl font-semibold theme-text-main mb-3 group-hover:text-[#006241] dark:group-hover:text-[#d4e9e2] transition",
                            {tr(l, "Parcours & Compétences", "Background & Skills")}
                        }
                        p { class: "theme-text-soft leading-relaxed mb-6 text-sm",
                            {tr(l, "Découvrez mon curriculum vitae, mes formations académiques, compétences techniques et projets réalisés.", "Discover my curriculum vitae, academic background, technical skills, and projects.")}
                        }
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "inline-flex items-center text-sm font-semibold text-[#00754A] dark:text-[#80c7b3] hover:text-[#006241] dark:hover:text-[#d4e9e2] gap-1 group-hover:translate-x-1 transition duration-200",
                        {tr(l, "Voir le CV & télécharger le PDF →", "View Resume & download PDF →")}
                    }
                }
            }

            // Starbucks Signature House Green (#1E3932) Feature Band
            section { class: "p-8 md:p-12 bg-[#1E3932] text-white rounded-[24px] sb-card-shadow flex flex-col md:flex-row items-center justify-between gap-8",
                div { class: "space-y-3 max-w-xl",
                    div { class: "inline-block px-3 py-1 bg-[#cba258] text-[#1E3932] text-xs font-bold rounded-full uppercase tracking-wider",
                        {tr(l, "Projets & Recherche", "Featured Projects")}
                    }
                    h2 { class: "text-2xl md:text-3xl font-semibold tracking-tight text-white",
                        {tr(l, "Architecture logicielle & développement système", "Software architecture & systems engineering")}
                    }
                    p { class: "text-sm text-[rgba(255,255,255,0.70)] leading-relaxed",
                        {tr(l, "Explorez mes projets en Rust, mes expérimentations WebAssembly et la structure de ce portfolio compilé directement en binaire performant.", "Explore my Rust projects, WebAssembly experiments, and the architecture of this portfolio compiled into a high-performance binary.")}
                    }
                }
                div { class: "flex flex-shrink-0 flex-col sm:flex-row gap-3 w-full md:w-auto",
                    Link {
                        to: Route::NotesHome {},
                        class: "px-6 py-3.5 sb-pill-white text-sm text-center shadow-md",
                        {tr(l, "Explorer les notes", "Explore notes")}
                    }
                }
            }

            // Featured Notes Preview
            if !notes.is_empty() {
                section { class: "space-y-6",
                    div { class: "flex justify-between items-end pb-2 border-b border-[var(--border-subtle)]",
                        div {
                            h2 { class: "text-2xl font-semibold theme-text-heading",
                                {tr(l, "Aperçu des Notes", "Notes Preview")}
                            }
                            p { class: "text-xs theme-text-soft mt-1",
                                {tr(l, "Dernières notes synchronisées dans le vault", "Recently synchronized notes in the vault")}
                            }
                        }
                        Link {
                            to: Route::NotesHome {},
                            class: "text-sm font-semibold text-[#00754A] dark:text-[#80c7b3] hover:underline",
                            {tr(l, "Tout voir →", "See all →")}
                        }
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        for note in notes {
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "group block p-6 theme-bg-card rounded-[12px] sb-card-shadow hover:shadow-md transition duration-200",
                                h3 { class: "text-lg font-semibold theme-text-main group-hover:text-[#00754A] dark:group-hover:text-[#d4e9e2] transition mb-2",
                                    "{note.title}"
                                }
                                p { class: "text-xs theme-text-soft line-clamp-2 mb-4 leading-relaxed",
                                    "{note.summary}..."
                                }
                                if !note.tags.is_empty() {
                                    div { class: "flex flex-wrap gap-1.5",
                                        for tag in &note.tags {
                                            span { class: "text-xs px-2.5 py-0.5 rounded-full bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] font-medium",
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
