use dioxus::prelude::*;
use crate::components::project_view::{ProjectView, ProjectItem, ProjectLink, ProjectSection};

pub fn project_data() -> ProjectItem {
    ProjectItem {
        slug: "reproduction-study".to_string(),
        title: "Reproduction Study : Transferability of Features in Deep Neural Networks".to_string(),
        tageline: "Reproduction empirique de \"How transferable are features in deep neural networks?\" (Yosinski et al., NeurIPS 2014)".to_string(),
        summary: "Reproduction et analyse du papier fondateur de Yosinski et al. (2014) qui quantifie la généralité vs. spécificité des représentations apprises couche par couche dans un réseau convolutif, et mesure l'impact du transfert de features sur les performances en généralisation.".to_string(),
        cover_image: "/assets/vault/mlp.png".to_string(),
        tags: vec!["Deep Learning".to_string(), "Python".to_string(), "PyTorch".to_string(), "Transfer Learning".to_string(), "Recherche ML".to_string()],
        date: Some("2026".to_string()),
        status: Some("Recherche / Reproduction".to_string()),
        links: vec![
            ProjectLink {
                label: "Dépôt GitHub Reproduction-study".to_string(),
                url: "https://github.com/baptxste/Reproduction-study".to_string(),
                icon: Some("github".to_string()),
            },
            ProjectLink {
                label: "Papier de référence (NeurIPS 2014)".to_string(),
                url: "https://arxiv.org/abs/1411.1792".to_string(),
                icon: Some("paper".to_string()),
            },
        ],
        highlights: vec![
            "Reproduction du protocole expérimental : réseaux base/cible avec n couches transférées (frozen vs. fine-tuned)".to_string(),
            "Vérification empirique du gradient général→spécifique à travers les couches".to_string(),
            "Mise en évidence de la co-adaptation entre couches adjacentes comme facteur limitant le transfert".to_string(),
        ],
        sections: vec![
            ProjectSection {
                title: "Le papier : généralité vs. spécificité des features".to_string(),
                content: "Yosinski et al. démontrent que les premières couches d'un CNN apprennent des représentations génériques (filtres de Gabor, blobs de couleur) réutilisables sur n'importe quelle tâche visuelle, tandis que les couches hautes se spécialisent progressivement sur la tâche d'entraînement. La transition n'est pas abrupte mais graduelle, et sa localisation varie selon la distance sémantique entre tâche source et tâche cible.".to_string(),
                images: vec![],
            },
            ProjectSection {
                title: "Protocole expérimental".to_string(),
                content: "Le protocole consiste à entraîner un réseau de base, à copier les n premières couches vers un réseau cible (en les gelant ou en les laissant se fine-tuner), puis à mesurer la précision finale. Deux effets sont isolés : la spécialisation des couches hautes (attendue) et les difficultés d'optimisation dues à la rupture de co-adaptation entre couches adjacentes (inattendue).".to_string(),
                images: vec![],
            },
            ProjectSection {
                title: "Résultat clé : le fine-tuning dépasse toujours le scratch".to_string(),
                content: "Même en transférant des features depuis une tâche sémantiquement éloignée, initialiser avec des poids pré-entraînés et les fine-tuner aboutit à une meilleure généralisation qu'un entraînement from scratch. Ce résultat, contre-intuitif à l'époque, fonde toute la pratique moderne du transfer learning.".to_string(),
                images: vec![],
            },
        ],
    }
}

#[component]
pub fn ReproductionStudyProject() -> Element {
    rsx! { ProjectView { project: project_data() } }
}
