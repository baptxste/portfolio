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
    let current_route = use_route::<Route>();

    let is_notes_route = matches!(
        current_route,
        Route::NotesHome {} | Route::NotePage { .. } | Route::TagPage { .. }
    );

    let link_class = |is_active: bool| -> &'static str {
        if is_active {
            "px-3 py-1.5 text-sm font-semibold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-950/60 rounded-lg transition"
        } else {
            "px-3 py-1.5 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg transition"
        }
    };

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            // Header Top Bar
            nav { class: "bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 sticky top-0 z-50 backdrop-blur-md bg-opacity-90 dark:bg-opacity-90 flex-shrink-0",
                div { class: "container mx-auto px-4 h-16 flex items-center justify-between max-w-7xl",
                    // Brand / Logo
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold text-gray-900 dark:text-white flex items-center gap-2.5",
                        span { class: "px-2 py-1 bg-indigo-600 text-white rounded-lg text-xs font-mono font-extrabold tracking-wider shadow-sm", "BC" }
                        span { class: "hidden sm:inline", "Baptiste Chachura" }
                    }

                    // Main Nav Links + Language Toggle Switch
                    div { class: "flex items-center space-x-2 sm:space-x-4",
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
                            {tr(lang(), "CV", "Resume")}
                        }

                        // Language Switcher Toggle
                        div { class: "pl-2 border-l border-gray-200 dark:border-gray-700 flex items-center",
                            button {
                                class: "px-2.5 py-1 text-xs font-bold rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:border-indigo-500 dark:hover:border-indigo-500 hover:text-indigo-600 dark:hover:text-indigo-400 transition flex items-center gap-1",
                                onclick: move |_| lang.write().toggle(),
                                span { class: if lang() == Language::Fr { "font-extrabold text-indigo-600 dark:text-indigo-400" } else { "opacity-60" }, "FR" }
                                span { class: "opacity-40", "|" }
                                span { class: if lang() == Language::En { "font-extrabold text-indigo-600 dark:text-indigo-400" } else { "opacity-60" }, "EN" }
                            }
                        }
                    }
                }
            }

            // Main Content Area with Conditional FileTree Sidebar for Notes
            if is_notes_route {
                div { class: "flex-1 flex flex-col md:flex-row max-w-7xl w-full mx-auto",
                    FileTree { notes }
                    main { class: "flex-1 p-6 md:p-8 overflow-y-auto",
                        Outlet::<Route> {}
                    }
                }
            } else {
                div { class: "flex-1 flex flex-col max-w-7xl w-full mx-auto",
                    main { class: "flex-1 p-6 md:p-8 overflow-y-auto",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
