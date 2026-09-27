use dioxus::prelude::*;

use crate::vault_assets::resolve_vault_asset;
use super::model::NoteMetaData;


// Pixel transparent, utilisé uniquement si une image référencée dans une note
// n'existe plus dans le vault (lien cassé, fichier renommé...) — évite
// l'icône "image cassée" plutôt que de planter le rendu.
const MISSING_IMAGE: &str =
    "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7";

#[component]
pub fn NoteObsidian(note: NoteMetaData) -> Element {
    let body_html = resolve_images(&note);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        article { class: "note-obsidian",
            header { class: "note-obsidian__header",
                h1 { class: "note-obsidian__title", "{note.title}" }
                div { class: "note-obsidian__meta",
                    if let Some(date) = &note.date {
                        time { class: "note-obsidian__date", "{date}" }
                    }
                    if !note.tags.is_empty() {
                        div { class: "note-obsidian__tags",
                            for tag in note.tags.iter() {
                                span { key: "{tag}", class: "note-obsidian__tag", "#{tag}" }
                            }
                        }
                    }
                }
            }

            div {
                class: "note-obsidian__body",
                dangerous_inner_html: "{body_html}",
            }

            if !note.backlinks.is_empty() {
                footer { class: "note-obsidian__backlinks",
                    h2 { class: "note-obsidian__backlinks-title", "Mentionné dans" }
                    ul { class: "note-obsidian__backlinks-list",
                        for bl in note.backlinks.iter() {
                            li { key: "{bl.slug}",
                                a { href: "/notes/{bl.slug}", class: "wikilink", "{bl.title}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Remplace chaque jeton `vault-asset:<nom-original>` laissé par build.rs par
/// l'URL finale de l'image, résolue via le fichier généré vault_assets.rs.
fn resolve_images(note: &NoteMetaData) -> String {
    let mut html = note.html.clone();
    for img_name in &note.images {
        let token = format!("vault-asset:{img_name}");
        let url = resolve_vault_asset(img_name)
            .map(|asset| asset.to_string())
            .unwrap_or_else(|| MISSING_IMAGE.to_string());
        html = html.replace(&token, &url);
    }
    html
}