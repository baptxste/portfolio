use dioxus::prelude::*;
use crate::i18n::{Language,tr};
use crate::components::scroll_area::{ScrollArea, ScrollDirection};
// ---------------------------------------------------------------------
// Une entrée de timeline = pure donnée, aucun style.
// ---------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct TimelineEntry {
    /// Optionnel. Format "mm/yyyy", ex: "09/2023"r
    pub date: Option<&'static str>,
    pub title_fr: &'static str,
    pub title_en: &'static str,
    pub subtitle_fr: &'static str,
    pub subtitle_en: &'static str,
    pub description_fr: &'static str,
    pub description_en: &'static str,
}

// ---------------------------------------------------------------------
// En-tête de section : icône ronde + titre bilingue.
// Le style de l'icône (fond/texte) vient de .timeline-icon-badge,
// plus du tout d'une prop -> identique partout où ce composant est utilisé.
// ---------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SectionHeaderProps {
    pub l: Language,
    pub icon: &'static str,
    pub title_fr: &'static str,
    pub title_en: &'static str,
}
// markdown like helpers
fn inline_bold(text: &str) -> Element {
    rsx! {
        for (i, part) in text.split("**").enumerate() {
            if i % 2 == 1 {
                strong { class: "font-semibold theme-text-main", "{part}" }
            } else {
                "{part}"
            }
        }
    }
}

fn rich_text(text: &str) -> Element {
    rsx! {
        for raw_line in text.trim().split('\n') {
            if let Some(rest) = raw_line.trim().strip_prefix("- ") {
                div { class: "timeline-bullet-item",
                    span { class: "timeline-bullet-dot" }
                    span { { inline_bold(rest) } }
                }
            } else if !raw_line.trim().is_empty() {
                p { class: "timeline-paragraph", { inline_bold(raw_line.trim()) } }
            }
        }
    }
}
// end helper 
#[component]
pub fn SectionHeader(props: SectionHeaderProps) -> Element {

    rsx! {
        h3 { class: "text-lg font-semibold theme-text-heading flex items-center gap-2.5",
            span { class: "timeline-icon-badge p-1.5 rounded-full text-sm", "{props.icon}" }
            { tr(props.l, props.title_fr, props.title_en) }
        }
    }
}

// ---------------------------------------------------------------------
// Liste d'entrées reliées par une ligne verticale.
// La ligne (.timeline-rail) et le point (.timeline-dot) sont figés ici,
// plus paramétrables depuis l'appelant.
// ---------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    pub l: Language,
    pub entries: Vec<TimelineEntry>,
}
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    rsx! {
        ScrollArea {  class: "timeline-rail",
            height: "20em",
            direction: ScrollDirection::Vertical,
            for entry in props.entries.iter() {
                div { class: "timeline-item",
                    div { class: "timeline-dot" }
                    div { class: "flex items-center justify-between gap-2",
                        h4 { class: "font-semibold theme-text-main text-sm",
                            { tr(props.l, entry.title_fr, entry.title_en) }
                        }
                        if let Some(date) = entry.date {
                            span { class: "text-[11px] font-medium theme-text-soft shrink-0", "{date}" }
                        }
                    }
                    p { class: "text-xs theme-text-soft mt-0.5",
                        { tr(props.l, entry.subtitle_fr, entry.subtitle_en) }
                    }
                    p { class: "text-xs theme-text-main mt-2 leading-relaxed",
                        { rich_text(tr(props.l, entry.description_fr, entry.description_en)) }
                    }
                }
            }
        }
    }
}
// ---------------------------------------------------------------------
// Composant final = header + timeline. Plus que le contenu en props,
// zéro style à passer.
// ---------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TimelineSectionProps {
    pub l: Language,
    pub icon: &'static str,
    pub title_fr: &'static str,
    pub title_en: &'static str,
    pub entries: Vec<TimelineEntry>,
}

#[component]
pub fn TimelineSection(props: TimelineSectionProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            SectionHeader {
                l: props.l,
                icon: props.icon,
                title_fr: props.title_fr,
                title_en: props.title_en,
            }
            Timeline {
                l: props.l,
                entries: props.entries,
            }
        }
    }
}

// ---------------------------------------------------------------------
// Utilisation : plus aucune couleur à passer, seulement le contenu.
// La date est optionnelle : Some("mm/yyyy") ou None.
// ---------------------------------------------------------------------

// fn example(l: Language) -> Element {
//     rsx! {
//         TimelineSection {
//             l: l,
//             icon: "🎓",
//             title_fr: "Formation & Parcours",
//             title_en: "Education & Background",
//             entries: vec![
//                 TimelineEntry {
//                     date: Some("09/2023"),
//                     title_fr: "Études Supérieures en Informatique & Mathématiques",
//                     title_en: "Higher Education in Computer Science & Mathematics",
//                     subtitle_fr: "Parcours Académique",
//                     subtitle_en: "Academic Path",
//                     description_fr: "Apprentissage des algorithmes, des mathématiques appliquées et de la programmation système.",
//                     description_en: "Studying algorithms, applied mathematics, and systems programming.",
//                 },
//                 TimelineEntry {
//                     date: None, // pas de date pour cette entrée -> rien ne s'affiche
//                     title_fr: "Autoformation continue",
//                     title_en: "Ongoing self-study",
//                     subtitle_fr: "Veille technologique",
//                     subtitle_en: "Tech watch",
//                     description_fr: "Approfondissement régulier via projets personnels et documentation.",
//                     description_en: "Ongoing deep-dives through personal projects and documentation.",
//                 },
//             ],
//         }
//     }
// }