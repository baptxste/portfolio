use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Parser};
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use walkdir::WalkDir;

// NoteMetaData / BacklinkInfo sont définies une seule fois, dans
// src/components/obsidian_note/model.rs, et incluses ici textuellement.
// L'app fait `mod ...;` de son côté pour désérialiser index.json avec
// exactement la même struct — plus de risque de désync.
include!("src/components/obsidian_note/model.rs");

#[derive(Deserialize)]
struct FrontMatter {
    title: Option<String>,
    tags: Option<Vec<String>>,
    date: Option<String>,
    publish: Option<bool>,
}

/// Rend les callouts Obsidian (`> [!type] Titre`) en blocs `<div>` HTML.
///
/// Contrairement à un simple `Regex::replace_all` sur la seule ligne d'en-tête,
/// cette fonction suit un état "callout ouvert / fermé" en parcourant les lignes
/// une à une, pour fermer correctement le `<div>` :
/// - dès qu'une ligne ne commence plus par `>` (règle Obsidian/CommonMark :
///   une ligne vide, ou toute ligne sans `>`, termine le blockquote courant) ;
/// - ou dès qu'un nouvel en-tête `> [!type]` apparaît alors qu'un callout
///   était encore ouvert (deux callouts collés sans ligne vide entre eux).
///
/// Sans ça, le `<div>` du premier callout ne se refermait jamais et engloutissait
/// tout le contenu suivant, y compris un deuxième callout séparé par un saut de ligne.
fn render_callouts(input: &str) -> String {
    let header_re = Regex::new(r"^>\s*\[!([a-zA-Z0-9_\-]+)\]\s*(.*)$").unwrap();
    let mut out = String::with_capacity(input.len());
    let mut in_callout = false;

    for line in input.lines() {
        if let Some(caps) = header_re.captures(line) {
            if in_callout {
                out.push_str("</div>\n\n");
            }
            let kind = caps[1].to_lowercase();
            let title = if caps[2].trim().is_empty() {
                caps[1].to_uppercase()
            } else {
                caps[2].trim().to_string()
            };
            out.push_str(&format!(
                "<div class=\"callout callout-{}\"><div class=\"callout-title\">{}</div>\n",
                kind, title
            ));
            in_callout = true;
        } else if in_callout && line.trim_start().starts_with('>') {
            let content = line.trim_start().trim_start_matches('>').trim_start();
            out.push_str(content);
            out.push('\n');
        } else {
            if in_callout {
                out.push_str("</div>\n\n");
                in_callout = false;
            }
            out.push_str(line);
            out.push('\n');
        }
    }

    if in_callout {
        out.push_str("</div>\n");
    }

    out
}

/// Transforme un nom de fichier arbitraire en identifiant Rust valide et
/// unique (pour les `pub static IMG_XXX` générés).
fn make_rust_ident(sanitized_filename: &str, used: &mut HashSet<String>) -> String {
    let base: String = sanitized_filename
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
        .collect();
    let mut ident = format!("IMG_{}", base);
    let mut n = 2;
    while used.contains(&ident) {
        ident = format!("IMG_{}_{}", base, n);
        n += 1;
    }
    used.insert(ident.clone());
    ident
}

/// Scanne src/components/**/style.css et concatène tout dans
/// assets/styling/components.css.
/// Règle simple : un nouveau composant = un style.css dans son dossier.
/// Rien d'autre à déclarer nulle part.
fn bundle_component_css() {
    let components_dir = Path::new("src/components");
    let out_file = Path::new("assets/styling/components.css");

    let mut bundle = String::from("/* Auto-généré par build.rs — ne pas éditer */\n");

    // Tri alphabétique pour un ordre stable entre builds
    let mut css_files: Vec<_> = WalkDir::new(components_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "style.css")
        .map(|e| e.into_path())
        .collect();
    css_files.sort();

    for path in &css_files {
        let component_name = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        bundle.push_str(&format!("\n/* ── {} ── */\n", component_name));
        match fs::read_to_string(path) {
            Ok(css) => bundle.push_str(&css),
            Err(e) => println!("cargo:warning=CSS non lisible {}: {}", path.display(), e),
        }
    }

    if let Some(parent) = out_file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(out_file, &bundle)
        .expect("Impossible d'écrire assets/styling/components.css");

    println!(
        "cargo:warning=CSS composants bundlés ({} fichiers) → {}",
        css_files.len(),
        out_file.display()
    );
}

fn main() {
    println!("cargo:rerun-if-changed=.vault");
    println!("cargo:rerun-if-changed=src/components");

    // -----------------------------------------------------------------------
    // Bundle automatique des CSS composants → assets/styling/components.css
    // Règle : ajouter un style.css dans src/components/mon_composant/
    //         c'est suffisant, rien d'autre à faire.
    // -----------------------------------------------------------------------
    bundle_component_css();

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR non défini");
    let exported_dir = Path::new(&out_dir).join("exported");
    let _ = fs::remove_dir_all(&exported_dir);

    let vault_root = Path::new(".vault/notes");

    // 1. Run obsidian-export if available on .vault/notes, or fallback to copy
    let export_status = Command::new("obsidian-export")
        .arg(vault_root)
        .arg(&exported_dir)
        .status();

    let source_dir = match export_status {
        Ok(status) if status.success() => exported_dir,
        _ => {
            println!("cargo:warning=obsidian-export non disponible ou échec, fallback sur la lecture directe du vault .vault/notes");
            vault_root.to_path_buf()
        }
    };

    if !source_dir.exists() {
        println!("cargo:warning=Le dossier source {} n'existe pas", source_dir.display());
        let empty_map: HashMap<String, NoteMetaData> = HashMap::new();
        let json = serde_json::to_string(&empty_map).unwrap();
        fs::write(Path::new(&out_dir).join("index.json"), json).unwrap();
        // On écrit quand même un vault_assets.rs vide pour que le include!() de
        // l'app ne casse pas la compilation si le vault est absent.
        fs::write(Path::new(&out_dir).join("vault_assets.rs"),
            "use dioxus::prelude::*;\npub fn resolve_vault_asset(_n: &str) -> Option<Asset> { None }\n"
        ).unwrap();
        return;
    }

    // -----------------------------------------------------------------
    // 2. Images du vault : copie réelle des fichiers + génération d'un
    //    asset!() littéral par image (voir explication en tête du fichier
    //    de sortie ci-dessous).
    // -----------------------------------------------------------------
    let vault_assets_src = Path::new(".vault/notes/assets");
    let vault_assets_dest = Path::new("assets/vault");
    let _ = fs::remove_dir_all(vault_assets_dest);
    fs::create_dir_all(vault_assets_dest).expect("impossible de créer assets/vault");

    // nom original (minuscules, tel qu'écrit dans ![[...]]) -> nom de fichier assaini copié sur disque
    let mut image_name_to_file: HashMap<String, String> = HashMap::new();
    let mut used_idents: HashSet<String> = HashSet::new();
    // (nom_original_minuscule, nom_assaini, identifiant_rust)
    let mut asset_entries: Vec<(String, String, String)> = Vec::new();

    for entry in WalkDir::new(vault_assets_src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico") {
            continue;
        }

        let original_name = path.file_name().unwrap().to_string_lossy().to_string();
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();

        // Nom de fichier assaini (slugifié) + gestion des collisions après slugification.
        let mut sanitized = format!("{}.{}", slug::slugify(&stem), ext);
        let mut dest = vault_assets_dest.join(&sanitized);
        let mut n = 2;
        while dest.exists() {
            sanitized = format!("{}-{}.{}", slug::slugify(&stem), n, ext);
            dest = vault_assets_dest.join(&sanitized);
            n += 1;
        }

        if let Err(e) = fs::copy(path, &dest) {
            println!("cargo:warning=Échec copie image {}: {}", original_name, e);
            continue;
        }

        let ident = make_rust_ident(&sanitized, &mut used_idents);
        image_name_to_file.insert(original_name.to_lowercase(), sanitized.clone());
        asset_entries.push((original_name.to_lowercase(), sanitized, ident));
        println!("cargo:warning=Image copiée: {}", original_name);
    }

    // Génère OUT_DIR/vault_assets.rs : un `asset!()` littéral par image (c'est
    // exactement le même code qu'un humain écrirait à la main, juste généré),
    // + une fonction de résolution nom -> Asset. Ce fichier est inclus par
    // src/vault_assets.rs dans le crate principal.
    let mut gen = String::new();
    gen.push_str("// Fichier généré par build.rs — ne pas éditer à la main.\n");
    gen.push_str("use dioxus::prelude::*;\n\n");
    for (_, sanitized, ident) in &asset_entries {
        gen.push_str(&format!(
            "pub static {ident}: Asset = asset!(\"/assets/vault/{sanitized}\");\n"
        ));
    }
    gen.push_str("\npub fn resolve_vault_asset(original_name: &str) -> Option<Asset> {\n");
    gen.push_str("    match original_name.to_lowercase().as_str() {\n");
    for (original_lower, _, ident) in &asset_entries {
        gen.push_str(&format!("        \"{original_lower}\" => Some({ident}),\n"));
    }
    gen.push_str("        _ => None,\n    }\n}\n");

    // ⚠️ C'est cette écriture qui manquait : sans elle, l'ancien
    // OUT_DIR/vault_assets.rs d'un build précédent restait en place et
    // continuait à être compilé tel quel.
    fs::write(Path::new(&out_dir).join("vault_assets.rs"), gen)
        .expect("Échec écriture vault_assets.rs");

    // -----------------------------------------------------------------
    // 3. Parse markdown notes
    // -----------------------------------------------------------------
    let matter = Matter::<YAML>::new();
    let mut raw_notes: Vec<(String, String, FrontMatter, String, String, String)> = Vec::new();
    let tag_regex = Regex::new(r"(?:^|\s)#([a-zA-Z0-9_\-]+)").unwrap();
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let wikilink_regex = Regex::new(r"\[\[([^\]\|]+)(?:\|([^\]]+))?\]\]").unwrap();
    let embed_img_regex = Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();

    for entry in WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let path = entry.path();

        if path.components().any(|c| {
            let name = c.as_os_str().to_string_lossy();
            name.starts_with('.') && name != ".vault"
        }) {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let result = matter.parse(&content);
        let fm: FrontMatter = match result.data {
            Some(data) => data.deserialize().unwrap_or(FrontMatter {
                title: None,
                tags: None,
                date: None,
                publish: Some(true),
            }),
            None => FrontMatter {
                title: None,
                tags: None,
                date: None,
                publish: Some(true),
            },
        };

        if fm.publish == Some(false) {
            continue;
        }

        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let slug = slug::slugify(&stem);

        let rel_path = path
            .strip_prefix(&source_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let folder = path
            .strip_prefix(&source_dir)
            .unwrap_or(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        raw_notes.push((stem, slug, fm, result.content, rel_path, folder));
    }

    let mut slug_to_title: HashMap<String, String> = HashMap::new();
    let mut stem_to_slug: HashMap<String, String> = HashMap::new();

    for (stem, slug, fm, _, _, _) in &raw_notes {
        let title = fm.title.clone().unwrap_or_else(|| stem.clone());
        slug_to_title.insert(slug.clone(), title);
        stem_to_slug.insert(stem.to_lowercase(), slug.clone());
    }

    let mut backlinks_map: HashMap<String, HashSet<String>> = HashMap::new();

    for (_, source_slug, _, body, _, _) in &raw_notes {
        for cap in link_regex.captures_iter(body) {
            let target_path = &cap[2];
            let target_stem = Path::new(target_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string().to_lowercase())
                .unwrap_or_default();
            if let Some(target_slug) = stem_to_slug.get(&target_stem) {
                backlinks_map.entry(target_slug.clone()).or_default().insert(source_slug.clone());
            }
        }

        for cap in wikilink_regex.captures_iter(body) {
            let target_name = cap[1].trim().to_lowercase();
            if let Some(target_slug) = stem_to_slug.get(&target_name) {
                backlinks_map.entry(target_slug.clone()).or_default().insert(source_slug.clone());
            }
        }
    }

    let mut notes_map: HashMap<String, NoteMetaData> = HashMap::new();

    for (stem, slug, fm, body, relative_path, folder) in raw_notes {
        let title = fm.title.unwrap_or(stem);

        let mut tags = fm.tags.unwrap_or_default();
        for cap in tag_regex.captures_iter(&body) {
            let tag_name = cap[1].to_string();
            if !tags.contains(&tag_name) {
                tags.push(tag_name);
            }
        }

        let body_no_tags = tag_regex.replace_all(&body, "");
        let body_callouts = render_callouts(&body_no_tags);

        // Images : on ne baque plus d'URL finale ici (build.rs ne connaît pas
        // le chemin hashé, seul le compilateur/CLI Dioxus le connaît). On pose
        // juste un jeton `vault-asset:<nom>` que NoteObsidian résoudra au
        // rendu via `resolve_vault_asset()`.
        let mut used_images: Vec<String> = Vec::new();
        let processed_images = embed_img_regex.replace_all(&body_callouts, |caps: &regex::Captures| {
            let img_raw = caps[1].trim();
            let img_name = img_raw.to_lowercase();
            if !image_name_to_file.contains_key(&img_name) {
                println!("cargo:warning=Image introuvable dans le vault: {}", img_raw);
            }
            if !used_images.contains(&img_name) {
                used_images.push(img_name.clone());
            }
            format!(
                "<img src=\"vault-asset:{}\" alt=\"{}\" class=\"rounded-lg my-4 shadow\" loading=\"lazy\" />",
                img_name, img_raw
            )
        });

        let processed_body = wikilink_regex.replace_all(&processed_images, |caps: &regex::Captures| {
            let target = caps[1].trim();
            let label = caps.get(2).map(|m| m.as_str()).unwrap_or(target);
            let target_slug = stem_to_slug.get(&target.to_lowercase()).cloned().unwrap_or_else(|| slug::slugify(target));
            format!("<a href=\"/notes/{}\" class=\"wikilink\">{}</a>", target_slug, label)
        });

        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);

        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax_theme = &ts.themes["base16-ocean.dark"];

        let parser = Parser::new_ext(&processed_body, options);
        let mut html_output = String::new();
        let mut in_code_block = false;
        let mut current_lang = String::new();
        let mut code_accumulator = String::new();

        for event in parser {
            match event {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_accumulator.clear();
                    current_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                        pulldown_cmark::CodeBlockKind::Indented => String::new(),
                    };
                }
                pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let lang_str = current_lang.trim();
                    let syntax = if !lang_str.is_empty() {
                        ss.find_syntax_by_token(lang_str)
                            .or_else(|| ss.find_syntax_by_extension(lang_str))
                            .unwrap_or_else(|| ss.find_syntax_plain_text())
                    } else {
                        ss.find_syntax_plain_text()
                    };

                    let display_lang = if !lang_str.is_empty() { lang_str } else { "code" };
                    let highlighted = highlighted_html_for_string(&code_accumulator, &ss, syntax, syntax_theme)
                        .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code_accumulator));

                    html_output.push_str("<div class=\"obsidian-code-block\">");
                    html_output.push_str(&format!(
                        "<div class=\"obsidian-code-header\"><span class=\"obsidian-code-lang\">{}</span><button class=\"obsidian-copy-btn\" onclick=\"copyObsidianCode(this)\" aria-label=\"Copier le code\"><svg class=\"icon-copy\" width=\"14\" height=\"14\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><rect x=\"9\" y=\"9\" width=\"13\" height=\"13\" rx=\"2\" ry=\"2\"></rect><path d=\"M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1\"></path></svg><svg class=\"icon-check\" width=\"14\" height=\"14\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><polyline points=\"20 6 9 17 4 12\"></polyline></svg></button></div>",
                        display_lang
                    ));
                    html_output.push_str("<div class=\"obsidian-code-content\">");
                    html_output.push_str(&highlighted);
                    html_output.push_str("</div></div>");
                }
                pulldown_cmark::Event::Text(text) if in_code_block => {
                    code_accumulator.push_str(&text);
                }
                _ => {
                    if !in_code_block {
                        let mut temp = String::new();
                        html::push_html(&mut temp, std::iter::once(event));
                        html_output.push_str(&temp);
                    }
                }
            }
        }

        let ext_link_regex = Regex::new(r#"<a href="(https?://[^"]+)">"#).unwrap();
        let html_output = ext_link_regex.replace_all(&html_output, r#"<a href="$1" target="_blank" rel="noopener noreferrer" class="external-link">"#).to_string();

        let plain_text = regex::Regex::new(r"<[^>]*>").unwrap().replace_all(&html_output, "");
        let summary: String = plain_text.chars().take(160).collect();

        let mut backlinks = Vec::new();
        if let Some(source_slugs) = backlinks_map.get(&slug) {
            for src_slug in source_slugs {
                if let Some(src_title) = slug_to_title.get(src_slug) {
                    backlinks.push(BacklinkInfo {
                        slug: src_slug.clone(),
                        title: src_title.clone(),
                    });
                }
            }
        }

        notes_map.insert(
            slug.clone(),
            NoteMetaData {
                slug,
                title,
                relative_path,
                folder,
                tags,
                date: fm.date,
                summary,
                html: html_output,
                images: used_images,
                backlinks,
            },
        );
    }

    let json_output = serde_json::to_string_pretty(&notes_map).expect("Failed to serialize index.json");
    let target_file = Path::new(&out_dir).join("index.json");
    fs::write(target_file, json_output).expect("Failed to write index.json");
}