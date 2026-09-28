use dioxus::prelude::*;
use crate::components::file_tree::FileTree;
use crate::components::popover::PopoverContent;
use crate::components::popover::PopoverTrigger;
use crate::components::popover::PopoverRoot;
use crate::content::VaultIndex;
use crate::i18n::{tr, Language};
use crate::Route;
use crate::Theme;

#[component]
pub fn Navbar() -> Element {
    let index: VaultIndex = use_context();
    let notes: Vec<_> = index.values().cloned().collect();

    let mut lang: Signal<Language> = use_context();
    let l = lang();
    let mut theme: Signal<Theme> = use_context();
    let current_route = use_route::<Route>();

    let is_notes_route = matches!(
        current_route,
        Route::NotesHome {} | Route::NotePage { .. } | Route::TagPage { .. }
    );

    let link_class = |is_active: bool| -> &'static str {
        if is_active {
            "nav-link active"
        } else {
            "nav-link"
        }
    };
    let mut open = use_signal(|| false);
    
    rsx! {
        div {
            class: "app-container {theme().to_class()}",
            "data-theme": theme().to_class(),

            // Global Nav Header
            nav { class: "site-nav",
                div { class: "nav-inner",
                    // Brand / Logo
                    Link {
                        to: Route::Home {},
                        class: "brand-logo",
                        span { class: "brand-avatar",
                            "BC"
                        }
                        span { class: "hidden-mobile",
                            "Baptiste Chachura"
                        }
                    }

                    // Main Nav Links + Theme & Language Settings Popover
                    div { class: "nav-links",
                        Link {
                            to: Route::Home {},
                            class: link_class(matches!(current_route, Route::Home {})),
                            {tr(l, "Accueil", "Home")}
                        }
                        Link {
                            to: Route::NotesHome {},
                            class: link_class(is_notes_route),
                            {tr(l, "Notes", "Notes")}
                        }
                        Link {
                            to: Route::CvPage {},
                            class: link_class(matches!(current_route, Route::CvPage {})),
                            {tr(l, "CV & Parcours", "Resume")}
                        }
                        
                        PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                            PopoverTrigger { {tr(l,"Réglages", "Settings") }}
                            PopoverContent {
                                div { class: "popover-settings-group",
                                    // Theme Toggle Switcher Button
                                    button {
                                        class: "popover-option-btn",
                                        onclick: move |_| theme.write().toggle(),
                                        span { "Thème" }
                                        span { class: "tag-pill", if theme().is_dark() { "Dark 🌙" } else { "Light ☀️" } }
                                    }

                                    // Language Toggle Switcher Button
                                    button {
                                        class: "popover-option-btn",
                                        onclick: move |_| lang.write().toggle(),
                                        span { "Langue" }
                                        span { class: "tag-pill", if lang() == Language::Fr { "FR 🇫🇷" } else { "EN 🇬🇧" } }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main Content Area with Conditional FileTree Sidebar for Notes
            if is_notes_route {
                div { class: "main-wrapper notes-layout",
                    FileTree { notes }
                    main { class: "main-content-card",
                        Outlet::<Route> {}
                    }
                }
            } else {
                div { class: "main-wrapper",
                    main { class: "main-content-card",
                        Outlet::<Route> {}
                    }
                }
            }

            // Floating "Frap" Signature Circular CTA Button
            Link {
                to: Route::NotesHome {},
                class: "sb-frap-button",
                style: "position: fixed; bottom: 1.5rem; right: 1.5rem; z-index: 50;",
                title: tr(l, "Recherche rapide / Vault", "Quick Vault Search"),
                svg {
                    width: "24",
                    height: "24",
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

            // Starbucks House Green Footer Bookend
            footer { class: "site-footer",
                div { class: "footer-inner",
                    div { class: "footer-top",
                        div { class: "space-y-1",
                            div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                span { style: "width: 0.75rem; height: 0.75rem; border-radius: 50%; background-color: var(--gold);" }
                                span { style: "font-size: 0.75rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--gold);", "Baptiste Chachura — Portfolio" }
                            }
                        }

                        div { class: "footer-nav",
                            Link { to: Route::Home {}, {tr(l, "Accueil", "Home")} }
                            Link { to: Route::NotesHome {}, {tr(l, "Notes & Vault", "Notes & Vault")} }
                            Link { to: Route::CvPage {}, {tr(l, "CV & Expérience", "Resume")} }
                        }
                    }

                    div { class: "footer-bottom",
                        p { "© 2026 Baptiste Chachura. Built in Rust." }
                        div { style: "display: flex; gap: 1rem;",
                            a { href: "https://github.com/baptxste", target: "_blank", "GitHub ↗" }
                            a { href: "mailto:baptiste.chachura@me.com", "Contact Email" }
                        }
                    }
                }
            }
        }
    }
}
