use dioxus::prelude::*;
use crate::i18n::{tr, Language};

#[component]
pub fn CvPage() -> Element {
    let lang: Signal<Language> = use_context();
    let l = lang();
    let mut show_pdf_embed = use_signal(|| true);

    rsx! {
        div { class: "container mx-auto px-4 py-8 max-w-5xl space-y-8",
            // Header Bar & Download Action
            header { class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-4 p-6 bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 shadow-sm",
                div {
                    h1 { class: "text-3xl font-extrabold text-gray-900 dark:text-white mb-1",
                        {tr(l, "Curriculum Vitae", "Curriculum Vitae")}
                    }
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
                        {tr(l, "Baptiste Chachura — Visualisateur & Téléchargement", "Baptiste Chachura — Viewer & Download")}
                    }
                }

                div { class: "flex flex-wrap items-center gap-3",
                    button {
                        class: "px-4 py-2 text-sm font-medium rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-100 transition",
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
                        class: "px-5 py-2.5 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-semibold rounded-lg shadow-sm hover:shadow transition flex items-center gap-2",
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
                        {tr(l, "Télécharger mon CV (PDF)", "Download my Resume (PDF)")}
                    }
                }
            }

            // PDF Viewer Container (when show_pdf_embed is true)
            if show_pdf_embed() {
                div { class: "bg-white dark:bg-gray-800 p-4 rounded-2xl border border-gray-100 dark:border-gray-700 shadow-sm",
                    div { class: "mb-3 flex justify-between items-center px-2 text-xs text-gray-500 dark:text-gray-400",
                        span { {tr(l, "Aperçu du fichier /assets/cv-chachura.pdf", "Preview of /assets/cv-chachura.pdf")} }
                        a {
                            href: "/assets/cv-chachura.pdf",
                            target: "_blank",
                            class: "text-indigo-600 dark:text-indigo-400 hover:underline",
                            {tr(l, "Ouvrir dans un nouvel onglet ↗", "Open in new tab ↗")}
                        }
                    }
                    object {
                        data: "/assets/cv-chachura.pdf",
                        r#type: "application/pdf",
                        class: "w-full h-[800px] rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900",
                        div { class: "text-center py-20 px-4 space-y-4",
                            p { class: "text-gray-600 dark:text-gray-300 font-medium",
                                {tr(l, "Le fichier PDF 'cv-chachura.pdf' sera affiché directement ici dès qu'il sera déposé dans le dossier assets.", "The PDF file 'cv-chachura.pdf' will be rendered directly here once placed in the assets directory.")}
                            }
                            a {
                                href: "/assets/cv-chachura.pdf",
                                download: "CV-Baptiste-Chachura.pdf",
                                class: "inline-block px-4 py-2 bg-indigo-600 text-white rounded-lg text-sm font-medium hover:bg-indigo-700 transition",
                                {tr(l, "Télécharger le PDF", "Download PDF")}
                            }
                        }
                    }
                }
            }

            // Web Structured CV Presentation
            div { class: "bg-white dark:bg-gray-800 p-8 md:p-12 rounded-2xl border border-gray-100 dark:border-gray-700 shadow-sm space-y-10",
                // Profile & Info Header
                div { class: "border-b border-gray-100 dark:border-gray-700 pb-8 flex flex-col md:flex-row justify-between gap-6",
                    div {
                        h2 { class: "text-3xl font-bold text-gray-900 dark:text-white mb-2", "Baptiste Chachura" }
                        p { class: "text-lg text-indigo-600 dark:text-indigo-400 font-medium mb-4",
                            {tr(l, "Étudiant & Développeur Software / Web", "Student & Software / Web Developer")}
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-300 max-w-2xl leading-relaxed",
                            {tr(l, "Passionné par l'architecture logicielle, le développement en Rust, les technologies web modernes et la modélisation mathématique.", "Passionate about software architecture, Rust development, modern web technologies, and mathematical modeling.")}
                        }
                    }
                    div { class: "space-y-2 text-sm text-gray-600 dark:text-gray-300 min-w-[200px]",
                        div { class: "flex items-center gap-2",
                            span { class: "font-semibold text-gray-900 dark:text-white", "Email:" }
                            a { href: "mailto:baptiste.chachura@me.com", class: "hover:underline text-indigo-600 dark:text-indigo-400", "baptiste.chachura@me.com" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "font-semibold text-gray-900 dark:text-white", {tr(l, "Localisation:", "Location:")} }
                            span { "France" }
                        }
                    }
                }

                // Grid: Competences & Formations
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    // Competences
                    div { class: "space-y-4",
                        h3 { class: "text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2",
                            span { class: "p-1.5 bg-indigo-100 dark:bg-indigo-950 text-indigo-600 rounded-lg text-sm", "💻" }
                            {tr(l, "Compétences Techniques", "Technical Skills")}
                        }
                        div { class: "space-y-3",
                            div {
                                span { class: "text-xs font-semibold uppercase text-gray-400 block mb-1.5", {tr(l, "Langages & Frameworks", "Languages & Frameworks")} }
                                div { class: "flex flex-wrap gap-2",
                                    span { class: "px-3 py-1 bg-indigo-50 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-md", "Rust" }
                                    span { class: "px-3 py-1 bg-indigo-50 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-md", "Dioxus" }
                                    span { class: "px-3 py-1 bg-indigo-50 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-md", "TypeScript / JS" }
                                    span { class: "px-3 py-1 bg-indigo-50 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-md", "HTML5 / Tailwind CSS" }
                                    span { class: "px-3 py-1 bg-indigo-50 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 text-xs font-semibold rounded-md", "Python" }
                                }
                            }
                            div {
                                span { class: "text-xs font-semibold uppercase text-gray-400 block mb-1.5", {tr(l, "Outils & Méthodes", "Tools & Methods")} }
                                div { class: "flex flex-wrap gap-2",
                                    span { class: "px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs font-medium rounded-md", "Git & GitHub" }
                                    span { class: "px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs font-medium rounded-md", "Obsidian" }
                                    span { class: "px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs font-medium rounded-md", "Linux" }
                                    span { class: "px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs font-medium rounded-md", "WebAssembly (WASM)" }
                                }
                            }
                        }
                    }

                    // Formations
                    div { class: "space-y-4",
                        h3 { class: "text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2",
                            span { class: "p-1.5 bg-indigo-100 dark:bg-indigo-950 text-indigo-600 rounded-lg text-sm", "📚" }
                            {tr(l, "Formation & Parcours", "Education & Background")}
                        }
                        div { class: "space-y-4 border-l-2 border-indigo-100 dark:border-gray-700 pl-4",
                            div { class: "relative",
                                div { class: "absolute -left-[21px] top-1.5 w-2.5 h-2.5 rounded-full bg-indigo-600" }
                                h4 { class: "font-semibold text-gray-900 dark:text-white",
                                    {tr(l, "Études Supérieures en Informatique & Mathématiques", "Higher Education in Computer Science & Mathematics")}
                                }
                                p { class: "text-xs text-gray-500 dark:text-gray-400",
                                    {tr(l, "Parcours Académique", "Academic Path")}
                                }
                                p { class: "text-sm text-gray-600 dark:text-gray-300 mt-1",
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