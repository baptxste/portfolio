use dioxus::prelude::*;
use crate::components::project_view::{ProjectView, ProjectItem, ProjectSection};

pub fn project_data() -> ProjectItem {
    ProjectItem {
        slug: "dimensionality-reduction-suite".to_string(),
        title: "Suite de Réduction de Dimension (PCA & LDA)".to_string(),
        tageline: "Algorithmes statistiques et géométriques de réduction d'espace".to_string(),
        summary: "Implémentation et étude comparative entre méthodes non-supervisées (Analyse en Composantes Principales) et supervisées (Analyse Discriminante Linéaire de Fisher) pour l'analyse de données multivariées.".to_string(),
        cover_image: "/assets/vault/pca-axe-principale.png".to_string(),
        tags: vec!["Machine Learning".to_string(), "Algèbre Linéaire".to_string(), "Data Science".to_string(), "Statistiques".to_string()],
        date: Some("2026".to_string()),
        status: Some("Terminé".to_string()),
        links: vec![],
        highlights: vec![
            "Décomposition en valeurs propres de matrices de covariance".to_string(),
            "Critère de Fisher pour la séparabilité inter-classes".to_string(),
            "Génération automatisée de figures didactiques vectorielles".to_string(),
        ],
        sections: vec![
            ProjectSection {
                title: "Principes & Visualisation".to_string(),
                content: "Comparaison des projections optimales selon que l'on souhaite maximiser la variance globale (PCA) ou maximiser la séparabilité entre classes (LDA).".to_string(),
                images: vec![],
            },
        ],
    }
}

#[component]
pub fn DimensionalityReductionProject() -> Element {
    rsx! { ProjectView { project: project_data() } }
}
