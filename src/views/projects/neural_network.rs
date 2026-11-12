use dioxus::prelude::*;
use crate::components::project_view::{ProjectView, ProjectItem, ProjectSection};

pub fn project_data() -> ProjectItem {
    ProjectItem {
        slug: "neural-network-from-scratch".to_string(),
        title: "Réseau de Neurones & MLP from Scratch".to_string(),
        tageline: "Implémentation complète d'un perceptron multicouche et rétropropagation".to_string(),
        summary: "Conception from scratch d'un moteur d'apprentissage profond : tenseurs, descente de gradient, fonctions d'activation non-linéaires et optimisation stochastique par batch.".to_string(),
        cover_image: "/assets/vault/mlp.png".to_string(),
        tags: vec!["Python".to_string(), "Machine Learning".to_string(), "Mathématiques".to_string(), "NumPy".to_string()],
        date: Some("2026".to_string()),
        status: Some("Terminé".to_string()),
        links: vec![],
        highlights: vec![
            "Calcul analytique des gradients et rétropropagation automatique".to_string(),
            "Implémentation des fonctions d'activation (ReLU, Sigmoïde, Tanh)".to_string(),
            "Visualisation des frontières de décision et métriques d'apprentissage".to_string(),
        ],
        sections: vec![
            ProjectSection {
                title: "Présentation".to_string(),
                content: "Ce projet explore les fondements théoriques et mathématiques de l'apprentissage profond en réimplémentant un MLP complet sans librairie de deep learning (type PyTorch ou TensorFlow).".to_string(),
                images: vec![],
            },
        ],
    }
}

#[component]
pub fn NeuralNetworkProject() -> Element {
    rsx! { ProjectView { project: project_data() } }
}
