use dioxus::prelude::*;
use crate::i18n::{Language,tr};
// ---------------------------------------------------------------------
// Composant réutilisable : remplace TOUS vos blocs "titre + liste de pills"
// (Outils & Méthodes, Langages, Compétences, etc.)
// ---------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TagListProps {
    /// Si `l` vient déjà d'un contexte global chez vous (use_context),
    /// vous pouvez supprimer ce champ et faire l'appel directement
    /// dans le corps du composant -> encore moins de code à chaque usage.
    pub l: Language,
    pub title_fr: &'static str,
    pub title_en: &'static str,
    pub tags: Vec<&'static str>,
}

#[component]
pub fn TagList(props: TagListProps) -> Element {
    rsx! {
        div {
            span {
                class: "tag-list-title",
                { tr(props.l, props.title_fr, props.title_en) }
            }
            div { class: "tag-list-pills",
                for tag in props.tags.iter() {
                    span { class: "tag-pill", "{tag}" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Utilisation : une section = un bloc de 5 lignes au lieu de 10+
// ---------------------------------------------------------------------

// fn example(l: Language) -> Element {
//     rsx! {
//         TagList {
//             l: l,
//             title_fr: "Outils & Méthodes",
//             title_en: "Tools & Methods",
//             tags: vec!["Git & GitHub", "Obsidian", "Linux", "WebAssembly (WASM)"],
//         }
//         TagList {
//             l: l,
//             title_fr: "Langages",
//             title_en: "Languages",
//             tags: vec!["Rust", "Python", "TypeScript"],
//         }
//     }
// }
