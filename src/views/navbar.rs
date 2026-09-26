use dioxus::prelude::*;
use crate::components::FileTree;
use crate::content::VaultIndex;
use crate::i18n::{tr, Language};
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let index: VaultIndex = use_context();
    let notes: Vec<_> = index.values().cloned().collect();

    let mut lang: Signal<Language> = use_context();
    let l = lang();
    let current_route = use_route::<Route>();

    let is_notes_route = matches!(
        current_route,
        Route::NotesHome {} | Route::NotePage { .. } | Route::TagPage { .. }
    );

    let link_class = |is_active: bool| -> &'static str {
        if is_active {
            "px-4 py-2 text-sm font-semibold text-[#006241] bg-[#d4e9e2] rounded-full transition duration-200"
        } else {
            "px-4 py-2 text-sm font-medium text-[rgba(0,0,0,0.87)] hover:text-[#006241] hover:bg-[#edebe9] rounded-full transition duration-200"
        }
    };

    rsx! {
        div { class: "min-h-screen flex flex-col bg-[#f2f0eb] text-[rgba(0,0,0,0.87)] selection:bg-[#d4e9e2] selection:text-[#006241]",
            // Global Nav Header - Starbucks white nav bar with triple shadow stack
            nav { class: "bg-white sticky top-0 z-50 sb-nav-shadow flex-shrink-0",
                div { class: "container mx-auto px-4 sm:px-6 h-20 flex items-center justify-between max-w-7xl",
                    // Brand / Logo (Starbucks Siren-inspired green roundel)
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold text-[#006241] flex items-center gap-3 group tracking-tight",
                        span { class: "w-10 h-10 bg-[#006241] text-white rounded-full flex items-center justify-center font-bold text-sm tracking-wider shadow-sm group-hover:bg-[#00754A] transition duration-200 group-active:scale-95",
                            "BC"
                        }
                        span { class: "hidden sm:inline font-bold text-[#006241] group-hover:text-[#00754A] transition",
                            "Baptiste Chachura"
                        }
                    }

                    // Main Nav Links & Language Switcher Pill
                    div { class: "flex items-center space-x-1 sm:space-x-3",
                        Link {
                            to: Route::Home {},
                            class: link_class(matches!(current_route, Route::Home {})),
                            {tr(lang(), "Accueil", "Home")}
                        }
                        Link {
                            to: Route::NotesHome {},
                            class: link_class(is_notes_route),
                            {tr(lang(), "Notes", "Notes")}
                        }
                        Link {
                            to: Route::CvPage {},
                            class: link_class(matches!(current_route, Route::CvPage {})),
                            {tr(lang(), "CV & Parcours", "Resume")}
                        }

                        // Language Toggle (Full-pill outlined style)
                        div { class: "pl-2 flex items-center",
                            button {
                                class: "px-3.5 py-1.5 text-xs font-bold rounded-full border border-slate-300 bg-white text-slate-800 hover:border-[#00754A] hover:text-[#00754A] active:scale-95 transition duration-200 flex items-center gap-1.5 shadow-sm",
                                onclick: move |_| lang.write().toggle(),
                                span { class: if lang() == Language::Fr { "text-[#006241] font-extrabold" } else { "opacity-50" }, "FR" }
                                span { class: "opacity-30", "|" }
                                span { class: if lang() == Language::En { "text-[#006241] font-extrabold" } else { "opacity-50" }, "EN" }
                            }
                        }
                    }
                }
            }

            // Main Content Area with Conditional FileTree Sidebar for Notes
            if is_notes_route {
                div { class: "flex-1 flex flex-col md:flex-row max-w-7xl w-full mx-auto px-4 py-6 gap-6",
                    FileTree { notes }
                    main { class: "flex-1 p-6 md:p-8 bg-white rounded-[12px] sb-card-shadow overflow-y-auto",
                        Outlet::<Route> {}
                    }
                }
            } else {
                div { class: "flex-1 flex flex-col max-w-7xl w-full mx-auto px-4 py-6",
                    main { class: "flex-1 p-4 md:p-6 overflow-y-auto",
                        Outlet::<Route> {}
                    }
                }
            }

            // Floating "Frap" Signature Circular CTA Button (Section 4.8 of DESIGN.md)
            Link {
                to: Route::NotesHome {},
                class: "fixed bottom-6 right-6 z-50 sb-frap-button group",
                title: tr(lang(), "Recherche rapide / Vault", "Quick Vault Search"),
                svg {
                    class: "w-6 h-6 text-white group-hover:scale-110 transition duration-200",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2.5",
                        d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    }
                }
            }

            // Starbucks House Green (#1E3932) Footer Bookend (Section 1 & 2 of DESIGN.md)
            footer { class: "bg-[#1E3932] text-white py-12 px-6 mt-16 flex-shrink-0",
                div { class: "container mx-auto max-w-7xl space-y-8",
                    div { class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-6 border-b border-[rgba(255,255,255,0.15)] pb-8",
                        div { class: "space-y-1",
                            div { class: "flex items-center gap-2",
                                span { class: "w-3 h-3 rounded-full bg-[#cba258]" }
                                span { class: "text-xs font-bold uppercase tracking-wider text-[#cba258]", "Baptiste Chachura — Portfolio" }
                            }
                            p { class: "text-sm text-[rgba(255,255,255,0.70)] max-w-lg",
                                {tr(l, "Développeur & étudiant passionné par l'architecture logicielle, Rust et le web moderne.", "Software engineer & student passionate about software architecture, Rust, and modern web.")}
                            }
                        }

                        div { class: "flex flex-wrap gap-4 text-sm font-semibold",
                            Link { to: Route::Home {}, class: "text-white hover:text-[#cba258] transition", {tr(l, "Accueil", "Home")} }
                            Link { to: Route::NotesHome {}, class: "text-white hover:text-[#cba258] transition", {tr(l, "Notes & Vault", "Notes & Vault")} }
                            Link { to: Route::CvPage {}, class: "text-white hover:text-[#cba258] transition", {tr(l, "CV & Expérience", "Resume")} }
                        }
                    }

                    div { class: "flex flex-col sm:flex-row justify-between items-center text-xs text-[rgba(255,255,255,0.70)] gap-4",
                        p { "© 2026 Baptiste Chachura. Built with Dioxus 0.7 & Rust." }
                        div { class: "flex items-center gap-4",
                            a { href: "https://github.com/baptxste", target: "_blank", class: "hover:text-white transition", "GitHub ↗" }
                            a { href: "mailto:baptiste.chachura@me.com", class: "hover:text-white transition", "Contact Email" }
                        }
                    }
                }
            }
        }
    }
}
