use dioxus::prelude::*;
use crate::i18n::{tr, Language};

#[component]
pub fn CvPage() -> Element {
    let lang: Signal<Language> = use_context();
    let l = lang();
    let mut show_pdf_embed = use_signal(|| true);

    rsx! {
        div { class: "container mx-auto px-2 py-4 max-w-5xl space-y-8",
            // Header Bar & Download Action - Starbucks 12px Theme Card
            header { class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-4 p-6 sm:p-8 theme-bg-card rounded-[12px] sb-card-shadow transition-colors duration-250",
                div {
                    div { class: "inline-block mb-2 px-3 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-bold rounded-full uppercase tracking-wider",
                        {tr(l, "Curriculum Vitae", "Curriculum Vitae")}
                    }
                    h1 { class: "text-3xl font-semibold theme-text-heading mb-1 tracking-tight",
                        "Baptiste Chachura"
                    }
                    p { class: "text-xs theme-text-soft font-medium",
                        {tr(l, "Visualisateur & Téléchargement du CV", "Resume Viewer & Download")}
                    }
                }

                div { class: "flex flex-wrap items-center gap-3",
                    button {
                        class: "px-5 py-2.5 sb-pill-outline text-xs",
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
                        class: "px-6 py-2.5 sb-pill-green text-xs flex items-center gap-2 shadow-sm",
                        svg {
                            class: "w-4 h-4",
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

            // PDF Viewer Container (when show_pdf_embed is true)
            if show_pdf_embed() {
                div { class: "theme-bg-card p-4 rounded-[12px] sb-card-shadow transition-colors duration-250",
                    div { class: "mb-3 flex justify-between items-center px-2 text-xs theme-text-soft font-medium",
                        span { {tr(l, "Aperçu du fichier /assets/cv-chachura.pdf", "Preview /assets/cv-chachura.pdf")} }
                        a {
                            href: "/assets/cv-chachura.pdf",
                            target: "_blank",
                            class: "text-[#00754A] dark:text-[#80c7b3] font-semibold hover:underline",
                            {tr(l, "Ouvrir dans un nouvel onglet ↗", "Open in new tab ↗")}
                        }
                    }
                    object {
                        data: "/assets/cv-chachura.pdf",
                        r#type: "application/pdf",
                        class: "w-full h-[800px] rounded-[8px] theme-bg-subtle border border-[var(--border-subtle)]",
                        div { class: "text-center py-20 px-4 space-y-4",
                            p { class: "theme-text-soft font-medium text-sm",
                                {tr(l, "Le fichier PDF 'cv-chachura.pdf' sera affiché directement ici dès qu'il sera déposé dans le dossier assets.", "The PDF file 'cv-chachura.pdf' will be rendered directly here once placed in the assets directory.")}
                            }
                            a {
                                href: "/assets/cv-chachura.pdf",
                                download: "CV-Baptiste-Chachura.pdf",
                                class: "inline-block px-5 py-2.5 sb-pill-green text-xs shadow-sm",
                                {tr(l, "Télécharger le PDF", "Download PDF")}
                            }
                        }
                    }
                }
            }

            // Web Structured CV Presentation - 12px Theme Card
            div { class: "theme-bg-card p-8 md:p-12 rounded-[12px] sb-card-shadow space-y-10 transition-colors duration-250",
                // Profile & Info Header
                div { class: "border-b border-[var(--border-subtle)] pb-8 flex flex-col md:flex-row justify-between gap-6",
                    div {
                        h2 { class: "text-3xl font-semibold theme-text-heading mb-2 tracking-tight", "Baptiste Chachura" }
                        p { class: "text-base text-[#00754A] dark:text-[#80c7b3] font-semibold mb-4",
                            {tr(l, "Étudiant & Développeur Software / Web", "Student & Software / Web Developer")}
                        }
                        p { class: "text-sm theme-text-main max-w-2xl leading-relaxed",
                            {tr(l, "Passionné par l'architecture logicielle, le développement en Rust, les technologies web modernes et la modélisation mathématique.", "Passionate about software architecture, Rust development, modern web technologies, and mathematical modeling.")}
                        }
                    }
                    div { class: "space-y-2 text-xs theme-text-main min-w-[220px] bg-[#faf6ee] dark:bg-[#251e13] p-4 rounded-[12px] border border-[#dfc49d]/40",
                        div { class: "flex items-center gap-2",
                            span { class: "font-bold text-[#006241] dark:text-[#d4e9e2]", "EMAIL:" }
                            a { href: "mailto:baptiste.chachura@me.com", class: "hover:underline text-[#00754A] dark:text-[#80c7b3] font-semibold", "baptiste.chachura@me.com" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "font-bold text-[#006241] dark:text-[#d4e9e2]", {tr(l, "LOCALISATION:", "LOCATION:")} }
                            span { "France" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "font-bold text-[#006241] dark:text-[#d4e9e2]", "STATUT:" }
                            span { class: "text-[#00754A] dark:text-[#80c7b3] font-bold", "Disponible" }
                        }
                    }
                }

                // Grid: Competences & Formations
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    // Competences
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold theme-text-heading flex items-center gap-2.5",
                            span { class: "p-1.5 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] rounded-full text-sm", "💻" }
                            {tr(l, "Compétences Techniques", "Technical Skills")}
                        }
                        div { class: "space-y-4",
                            div {
                                span { class: "text-xs font-bold uppercase theme-text-soft block mb-2 tracking-wider", {tr(l, "Langages & Frameworks", "Languages & Frameworks")} }
                                div { class: "flex flex-wrap gap-2",
                                    span { class: "px-3.5 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-semibold rounded-full", "Rust" }
                                    span { class: "px-3.5 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-semibold rounded-full", "Dioxus" }
                                    span { class: "px-3.5 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-semibold rounded-full", "TypeScript / JS" }
                                    span { class: "px-3.5 py-1 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] text-xs font-semibold rounded-full", "HTML5 / Tailwind CSS" }
                                    span { class: "px-3.5 py-1 bg-[#faf6ee] dark:bg-[#251e13] text-[#cba258] text-xs font-semibold rounded-full border border-[#dfc49d]/50", "Python" }
                                }
                            }
                            div {
                                span { class: "text-xs font-bold uppercase theme-text-soft block mb-2 tracking-wider", {tr(l, "Outils & Méthodes", "Tools & Methods")} }
                                div { class: "flex flex-wrap gap-2",
                                    span { class: "px-3.5 py-1 theme-bg-subtle theme-text-main text-xs font-medium rounded-full border border-[var(--border-subtle)]", "Git & GitHub" }
                                    span { class: "px-3.5 py-1 theme-bg-subtle theme-text-main text-xs font-medium rounded-full border border-[var(--border-subtle)]", "Obsidian" }
                                    span { class: "px-3.5 py-1 theme-bg-subtle theme-text-main text-xs font-medium rounded-full border border-[var(--border-subtle)]", "Linux" }
                                    span { class: "px-3.5 py-1 theme-bg-subtle theme-text-main text-xs font-medium rounded-full border border-[var(--border-subtle)]", "WebAssembly (WASM)" }
                                }
                            }
                        }
                    }

                    // Formations
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold theme-text-heading flex items-center gap-2.5",
                            span { class: "p-1.5 bg-[#d4e9e2] dark:bg-[#24463e] text-[#006241] dark:text-[#d4e9e2] rounded-full text-sm", "🎓" }
                            {tr(l, "Formation & Parcours", "Education & Background")}
                        }
                        div { class: "space-y-4 border-l-2 border-[#d4e9e2] dark:border-[#24463e] pl-4",
                            div { class: "relative pl-2",
                                div { class: "absolute -left-[23px] top-1.5 w-3 h-3 rounded-full bg-[#00754A] dark:bg-[#80c7b3]" }
                                h4 { class: "font-semibold theme-text-main text-sm",
                                    {tr(l, "Études Supérieures en Informatique & Mathématiques", "Higher Education in Computer Science & Mathematics")}
                                }
                                p { class: "text-xs theme-text-soft mt-0.5",
                                    {tr(l, "Parcours Académique", "Academic Path")}
                                }
                                p { class: "text-xs theme-text-main mt-2 leading-relaxed",
                                    {tr(l, "Apprentissage des algorithmes, des mathématiques appliquées et de la programmation système.", "Studying algorithms, applied mathematics, and systems programming.")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}