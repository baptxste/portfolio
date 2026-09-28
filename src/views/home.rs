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
        div { class: "home-container",
            // Hero Section
            section { class: "home-hero",
                div { class: "hero-badge",
                    {tr(l, "Portfolio", "Portfolio")}
                }
                h1 { class: "hero-title",
                    "Baptiste Chachura"
                }
                p { class: "hero-subtitle",
                    {tr(l, "Développeur & étudiant passionné par l'architecture logicielle, l'écosystème Rust, le web moderne et les mathématiques.", "Software engineer & student passionate about software architecture, the Rust ecosystem, modern web, and mathematics.")}
                }
                div { class: "hero-actions",
                    Link {
                        to: Route::NotesHome {},
                        class: "sb-pill-green",
                        {tr(l, "Mes Notes", "My Notes")}
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "sb-pill-outline",
                        {tr(l, "Mon CV", "My Resume")}
                    }
                }
            }

            // Quick Overview Cards
            section { class: "cards-grid",
                div { class: "overview-card",
                    div {
                        h2 { {tr(l, "Jardin Numérique", "Digital Garden")} }
                        p { {tr(l, "Mon coffre de connaissances Obsidian rendu dynamiquement en Rust via Dioxus. Retrouvez des cours, réflexions, théorèmes et résumés techniques.", "My Obsidian knowledge vault rendered dynamically in Rust via Dioxus. Explore notes, thoughts, theorems, and technical summaries.")} }
                    }
                    Link {
                        to: Route::NotesHome {},
                        class: "card-link",
                        {tr(l, "Accéder au vault →", "Open the vault →")}
                    }
                }

                div { class: "overview-card",
                    div {
                        h2 { {tr(l, "Parcours & Compétences", "Background & Skills")} }
                        p { {tr(l, "Découvrez mon curriculum vitae, mes formations académiques, compétences techniques et projets réalisés.", "Discover my curriculum vitae, academic background, technical skills, and projects.")} }
                    }
                    Link {
                        to: Route::CvPage {},
                        class: "card-link",
                        {tr(l, "Voir le CV & télécharger le PDF →", "View Resume & download PDF →")}
                    }
                }
            }

            // Signature Feature Band
            section { class: "feature-band",
                div { class: "space-y-3 max-w-xl",
                    div { class: "feature-band-badge",
                        {tr(l, "Projets & Recherche", "Featured Projects")}
                    }
                    h2 { {tr(l, "Architecture logicielle & développement système", "Software architecture & systems engineering")} }
                    p { {tr(l, "Explorez mes projets en Rust, mes expérimentations WebAssembly et la structure de ce portfolio compilé directement en binaire performant.", "Explore my Rust projects, WebAssembly experiments, and the architecture of this portfolio compiled into a high-performance binary.")} }
                }
                div {
                    Link {
                        to: Route::NotesHome {},
                        class: "sb-pill-white",
                        style: "padding: 0.75rem 1.5rem; display: inline-block;",
                        {tr(l, "Explorer les notes", "Explore notes")}
                    }
                }
            }

            // Featured Notes Preview
            if !notes.is_empty() {
                section {
                    div { class: "section-header",
                        div {
                            h2 { {tr(l, "Aperçu des Notes", "Notes Preview")} }
                            p { {tr(l, "Dernières notes synchronisées dans le vault", "Recently synchronized notes in the vault")} }
                        }
                        Link {
                            to: Route::NotesHome {},
                            class: "card-link",
                            {tr(l, "Tout voir →", "See all →")}
                        }
                    }

                    div { class: "notes-preview-grid",
                        for note in notes {
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "note-card-item",
                                h3 { "{note.title}" }
                                p { "{note.summary}..." }
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
}
