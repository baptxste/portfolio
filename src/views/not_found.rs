use dioxus::prelude::*;
use crate::i18n::{tr, Language};
use crate::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let lang: Signal<Language> = use_context();
    let l = lang();
    let mut show_details = use_signal(|| false);

    let route_path = format!("/{}", route.join("/"));

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; text-align: center; gap: 1.5rem; padding: 4rem 2rem;",
                div { class: "hero-badge", style: "font-size: 0.875rem; padding: 0.35rem 1.25rem;", "404 Error" }
                
                h1 { class: "hero-title", style: "margin-bottom: 0.5rem;",
                    {tr(l, "Oups ! Page introuvable", "Oops! Page Not Found")}
                }
                
                p { class: "hero-subtitle", style: "margin-bottom: 1rem;",
                    {tr(l, "La page que vous recherchez n'existe pas ou a été déplacée.", "The page you are looking for does not exist or has been moved.")}
                }

                div { class: "hero-actions", style: "margin-bottom: 1rem;",
                    Link {
                        to: Route::Home {},
                        class: "sb-pill-green",
                        style: "padding: 0.75rem 1.5rem; display: inline-flex; align-items: center; gap: 0.5rem;",
                        {tr(l, "Retour à l'accueil", "Back to Home")}
                    }
                    Link {
                        to: Route::NotesHome {},
                        class: "sb-pill-outline",
                        style: "padding: 0.75rem 1.5rem; display: inline-flex; align-items: center; gap: 0.5rem;",
                        {tr(l, "Explorer les notes", "Explore Notes")}
                    }
                }

                // Bouton pour afficher/masquer le panneau de détails techniques
                button {
                    class: "sb-pill-white",
                    style: "font-size: 0.8rem; padding: 0.4rem 1rem; cursor: pointer;",
                    onclick: move |_| show_details.set(!show_details()),
                    if show_details() {
                        {tr(l, "Masquer les détails ▲", "Hide Details ▲")}
                    } else {
                        {tr(l, "Afficher les détails techniques ▼", "Show Technical Details ▼")}
                    }
                }

                // Panneau masquable contenant les informations sur l'URL / route qui ne peut pas être affichée
                if show_details() {
                    div {
                        style: "width: 100%; max-width: 32rem; margin-top: 1rem; text-align: left; background-color: var(--bg-subtle); border: 1px solid var(--border-subtle); border-radius: 12px; padding: 1.25rem; font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; box-shadow: var(--card-shadow);",
                        
                        div { style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem; font-weight: 700; color: var(--text-soft); border-bottom: 1px solid var(--border-subtle); padding-bottom: 0.35rem;",
                            span { {tr(l, "DÉTAILS DU ROUTAGE", "ROUTING DETAILS")} }
                            span { class: "tag-pill", style: "font-size: 0.65rem;", "HTTP 404" }
                        }
                        
                        div { style: "display: flex; flex-direction: column; gap: 0.4rem; color: var(--text-main);",
                            div {
                                span { style: "color: var(--text-soft);", {tr(l, "URL demandée : ", "Requested URL: ")} }
                                span { style: "color: var(--text-link); font-weight: 600;", "{route_path}" }
                            }
                            div {
                                span { style: "color: var(--text-soft);", {tr(l, "Segments de route : ", "Route Segments: ")} }
                                span { "{route:?}" }
                            }
                            div {
                                span { style: "color: var(--text-soft);", {tr(l, "Statut : ", "Status: ")} }
                                span { style: "color: #dc2626; font-weight: 600;", {tr(l, "Non trouvée / Non affichable", "Not Found / Unrenderable")} }
                            }
                        }
                    }
                }
        }
    }
}
