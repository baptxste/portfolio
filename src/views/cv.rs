use dioxus::prelude::*;
use crate::i18n::{tr, Language};
use crate::components::{
    tag_list::TagList,
    timeline_section::{TimelineEntry, TimelineSection}
    };


#[component]
pub fn CvPage() -> Element {
    let lang: Signal<Language> = use_context();
    let l = lang();
    let mut show_pdf_embed = use_signal(|| true);

    rsx! {
        div { class: "cv-container",
            // Header Bar & Download Action
            header { class: "cv-header-card",
                div { class: "cv-header-title",
                    div { class: "hero-badge",
                        {tr(l, "Curriculum Vitae", "Curriculum Vitae")}
                    }
                    h1 { "Baptiste Chachura" }
                    p { {tr(l, "Visualisateur & Téléchargement du CV", "Resume Viewer & Download")} }
                }

                div { class: "cv-header-actions",
                    button {
                        class: "sb-pill-outline",
                        onclick: move |_| show_pdf_embed.set(!show_pdf_embed()),
                        if show_pdf_embed() {
                            {tr(l, "Afficher le format Web", "Show Web View")}
                        } else {
                            {tr(l, "Afficher le lecteur PDF", "Show PDF Reader")}
                        }
                    }

                    a {
                        href: "/assets/cv-chachura.pdf",
                        download: "CV-Baptiste-Chachura.pdf",
                        target: "_blank",
                        class: "sb-pill-green",
                        style: "display: inline-flex; align-items: center; gap: 0.5rem;",
                        svg {
                            width: "16",
                            height: "16",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                            }
                        }
                        {tr(l, "Télécharger le PDF", "Download PDF")}
                    }
                }
            }

            // Web Structured CV Presentation
            div { class: "cv-body-card",
                // Profile & Info Header
                div { class: "cv-profile-section",
                    div { class: "cv-profile-info",
                        h2 { "Baptiste Chachura" }
                        p { class: "cv-profile-role",
                            {tr(l, "Étudiant & Développeur Software / Web", "Student & Software / Web Developer")}
                        }
                        p { class: "cv-profile-desc",
                            {tr(l, "Passionné par l'architecture logicielle, le développement en Rust, les technologies web modernes et la modélisation mathématique.", "Passionate about software architecture, Rust development, modern web technologies, and mathematical modeling.")}
                        }
                    }
                    div { class: "cv-contact-box",
                        div { class: "contact-row",
                            span { class: "contact-label", "EMAIL:" }
                            a { href: "mailto:baptiste.chachura@me.com", style: "color: var(--text-link); font-weight: 600;", "baptiste.chachura@me.com" }
                        }
                        div { class: "contact-row",
                            span { class: "contact-label", {tr(l, "LOCALISATION:", "LOCATION:")} }
                            span { "France" }
                        }
                        div { class: "contact-row",
                            span { class: "contact-label", "STATUT:" }
                            span { style: "color: var(--text-link); font-weight: 700;", "Disponible" }
                        }
                    }
                }

                // Grid: Competences & Formations
                div { class: "cv-sections-grid",
                    // Competences
                    div { style: "display: flex; flex-direction: column; gap: 1rem;",
                        h3 { style: "font-size: 1.125rem; font-weight: 700; color: var(--text-heading); display: flex; align-items: center; gap: 0.5rem;",
                            span { style: "padding: 0.375rem; background-color: var(--bg-pill-light); border-radius: 50%; font-size: 0.875rem;", "💻" }
                            {tr(l, "Compétences Techniques", "Technical Skills")}
                        }
                        div { style: "display: flex; flex-direction: column; gap: 1rem;",
                            TagList {
                                l: l,
                                title_fr: "Langages & Frameworks",
                                title_en: "Languages & Frameworks",
                                tags: vec!["Rust", "Python", "TypeScript"],
                            }
                            TagList {
                                l: l,
                                title_fr: "Outils & Méthodes",
                                title_en: "Tools & Methods",
                                tags: vec!["Git & GitHub", "Obsidian", "Linux", "WebAssembly (WASM)"],
                            }
                        }
                    }
                    TimelineSection {
                        l: l,
                        icon: "🎓",
                        title_fr: "Formation & Parcours",
                        title_en: "Education & Background",
                        entries: vec![
                            TimelineEntry {
                                date: Some("09/2025"),
                                title_fr: "Ecole d'Ingénieur - Centrale",
                                title_en: "Engineering school - Centrale",
                                subtitle_fr: "Centrale Marseille",
                                subtitle_en: "Centrale Marseille, (France)",
                                description_fr: "Cours intensifs en mathématiques, physique,\n -  chimie, \n - informatique, \n - sciences de l'ingénieur, anglais.",
                                description_en: "Intensive program preparing for the competitive entrance exams to French 'Grandes Écoles' with a focus on Advanced Mathematics, Physics, Chemistry, Computer Science, Engineering Sciences, and English.",
                            },
                            TimelineEntry {
                                date: Some("09/2022"),
                                title_fr: "CPGE PCSI / PC*",
                                title_en: "CPGE PCSI / PC*",
                                subtitle_fr: "Lycée Descartes, Tours",
                                subtitle_en: "Lycée Descartes, Tours (France)",
                                description_fr: "Cours intensifs en mathématiques, physique, chimie, informatique, sciences de l'ingénieur, anglais.",
                                description_en: "Intensive program preparing for the competitive entrance exams to French 'Grandes Écoles' with a focus on Advanced Mathematics, Physics, Chemistry, Computer Science, Engineering Sciences, and English.",
                            },
                            TimelineEntry {
                                date: Some("09/2019"),
                                title_fr: "Baccalauréat Scientifique",
                                title_en: "A-Levels in STEM",
                                subtitle_fr: "Lycée Marceau, Chartres",
                                subtitle_en: "Lycée Marceau, Chartres (France)",
                                description_fr: "Baccalauréat filière scientifique, mention Très Bien, mention européenne (physique-chimie), sport étude Volley Ball.",
                                description_en: "Graduated with highest honors (Mention Très Bien), European section (Physics & Chemistry in English), and student-athlete program in Volleyball.",
                            },
                        ],
                    }
                }
            }

            // PDF Viewer Container
            if show_pdf_embed() {
                div { class: "cv-pdf-embed-card",
                    div { class: "pdf-header",
                        span { {tr(l, "Aperçu du fichier /assets/cv-chachura.pdf", "Preview /assets/cv-chachura.pdf")} }
                        a {
                            href: "/assets/cv-chachura.pdf",
                            target: "_blank",
                            style: "color: var(--text-link); font-weight: 600;",
                            {tr(l, "Ouvrir dans un nouvel onglet ↗", "Open in new tab ↗")}
                        }
                    }
                    object {
                        data: "/assets/cv-chachura.pdf",
                        r#type: "application/pdf",
                        class: "pdf-object",
                        div { style: "text-center; padding: 5rem 1rem;",
                            p { style: "color: var(--text-soft); font-size: 0.875rem; margin-bottom: 1rem;",
                                {tr(l, "Le fichier PDF 'cv-chachura.pdf' sera affiché directement ici dès qu'il sera déposé dans le dossier assets.", "The PDF file 'cv-chachura.pdf' will be rendered directly here once placed in the assets directory.")}
                            }
                            a {
                                href: "/assets/cv-chachura.pdf",
                                download: "CV-Baptiste-Chachura.pdf",
                                class: "sb-pill-green",
                                style: "padding: 0.625rem 1.25rem; display: inline-block;",
                                {tr(l, "Télécharger le PDF", "Download PDF")}
                            }
                        }
                    }
                }
            }
        }
    }
}