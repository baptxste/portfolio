use dioxus::prelude::*;
use crate::components::project_view::{ProjectView, ProjectItem, ProjectLink, ProjectSection};

pub fn project_data() -> ProjectItem {
    ProjectItem {
        slug: "portfolio-website".to_string(),
        title: "Portfolio & Vault Obsidian".to_string(),
        tageline: "Site web personnel & jardin numérique interactif bâti en Rust et Dioxus".to_string(),
        summary: "Conception et développement d'un portfolio moderne couplé à un parseur complet de vault Obsidian (wikilinks, callouts, KaTeX, coloration syntaxique syntect) déployé sur GitHub Pages via WebAssembly.".to_string(),
        cover_image: "/assets/header.svg".to_string(),
        tags: vec!["Rust".to_string(), "Dioxus 0.7".to_string(), "WebAssembly".to_string(), "Obsidian".to_string()],
        date: Some("2026".to_string()),
        status: Some("En production".to_string()),
        links: vec![
            ProjectLink {
                label: "Code Source GitHub".to_string(),
                url: "https://github.com/baptxste/portfolio".to_string(),
                icon: Some("github".to_string()),
            },
        ],
        highlights: vec![
            "Génération statique et compilation vers WebAssembly sans runtime lourd".to_string(),
            "Support des formules mathématiques KaTeX et coloration syntaxique syntect".to_string(),
            "Gestion fine du mode sombre / clair et support multilingue (FR/EN)".to_string(),
        ],
        sections: vec![
            ProjectSection {
                title: "Contexte & Objectifs".to_string(),
                content: "L'objectif était de créer un espace personnel centralisant à la fois un CV interactif, un jardin numérique basé sur un coffre Obsidian existant et une vitrine pour les projets réalisés. Tout est compilé en Rust avec Dioxus 0.7 pour des performances optimales côté client.".to_string(),
                images: vec![],
            },
            ProjectSection {
                title: "Architecture Technique".to_string(),
                content: "Un script build.rs scanne et précompile les fichiers Markdown et les images lors du build Cargo pour produire un index JSON compact et du code Rust typé. L'application Dioxus consomme ensuite cet index et offre une navigation fluide sans rechargement de page.".to_string(),
                images: vec![],
            },
        ],
    }
}

#[component]
pub fn PortfolioWebsiteProject() -> Element {
    rsx! { ProjectView { project: project_data() } }
}
