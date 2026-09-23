use dioxus::prelude::*;
use crate::components::FileTree;
use crate::content::VaultIndex;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let index: VaultIndex = use_context();
    let notes: Vec<_> = index.values().cloned().collect();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            // Header Top Bar
            nav { class: "bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 sticky top-0 z-50 backdrop-blur-md bg-opacity-90 dark:bg-opacity-90 flex-shrink-0",
                div { class: "container mx-auto px-4 h-14 flex items-center justify-between max-w-7xl",
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold text-gray-900 dark:text-white flex items-center gap-2",
                        span { class: "p-1.5 bg-indigo-600 text-white rounded-lg text-xs font-mono", "OX" }
                        "Obsidian Digital Garden"
                    }

                    div { class: "flex items-center space-x-6",
                        Link {
                            to: Route::Home {},
                            class: "text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition",
                            "Accueil"
                        }
                    }
                }
            }

            // Main Scaffold Layout (Sidebar + Content Outlet)
            div { class: "flex-1 flex flex-col md:flex-row max-w-7xl w-full mx-auto",
                FileTree { notes }
                main { class: "flex-1 p-6 md:p-8 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

