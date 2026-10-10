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

    let note_slug = note.slug.clone();
    use_effect(use_reactive!(|(note_slug,)| {
        let _ = note_slug;
        let _ = document::eval(
            r###"
            window.copyObsidianCode = function(button) {
                const block = button.closest('.obsidian-code-block');
                if (!block) return;
                const codeContent = block.querySelector('.obsidian-code-content pre, .obsidian-code-content code');
                if (!codeContent) return;
                const text = codeContent.innerText;
                navigator.clipboard.writeText(text).then(() => {
                    button.classList.add('copied');
                    setTimeout(() => {
                        button.classList.remove('copied');
                    }, 2500);
                }).catch(err => {
                    console.error('Erreur lors de la copie: ', err);
                });
            };

            function triggerKaTeX() {
                let retries = 0;
                const maxRetries = 20; // 2 secondes max (20 * 100ms)
                const interval = setInterval(() => {
                    const noteElem = document.querySelector('.note-obsidian');
                    if (typeof window.renderMathInElement === 'function' && noteElem) {
                        clearInterval(interval);
                        window.renderMathInElement(noteElem, {
                            delimiters: [
                                {left: '$$', right: '$$', display: true},
                                {left: '$', right: '$', display: false},
                                {left: '\\(', right: '\\)', display: false},
                                {left: '\\[', right: '\\]', display: true}
                            ],
                            ignoredClasses: [],
                            throwOnError: false,
                            strict: false,
                            ignoredTags: ["script", "noscript", "style", "textarea", "pre", "code"]
                        });

                        // renderMathInElement pour pulldown-cmark (balises <span class="math math-inline"> et <span class="math math-display">)
                        if (window.katex) {
                            noteElem.querySelectorAll('.math-inline').forEach(el => {
                                try {
                                    window.katex.render(el.textContent, el, { displayMode: false, throwOnError: false });
                                } catch(e) {}
                            });
                            noteElem.querySelectorAll('.math-display').forEach(el => {
                                try {
                                    window.katex.render(el.textContent, el, { displayMode: true, throwOnError: false });
                                } catch(e) {}
                            });
                        }
                    } else {
                        retries++;
                        if (retries >= maxRetries) {
                            clearInterval(interval);
                        }
                    }
                }, 100);
            }

            // Un micro-délai pour s'assurer que le DOM mis à jour par Dioxus est attaché
            setTimeout(triggerKaTeX, 30);
            "###
        );
    }));

    rsx! {
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