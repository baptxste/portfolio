use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteMetaData {
    pub slug: String,
    pub title: String,
    pub relative_path: String,
    pub folder: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub summary: String,
    pub html: String,
    pub backlinks: Vec<BacklinkInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinkInfo {
    pub slug: String,
    pub title: String,
}

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
            // Nouvel en-tête alors qu'un callout précédent était encore ouvert : on le ferme.
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
            // Ligne de corps du callout : on retire le '>' de tête, le reste (wikilinks,
            // embeds, texte) sera traité normalement par les passes suivantes.
            let content = line.trim_start().trim_start_matches('>').trim_start();
            out.push_str(content);
            out.push('\n');
        } else {
            // Toute ligne sans '>' termine le callout en cours, ligne vide incluse.
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

fn main() {
    println!("cargo:rerun-if-changed=.vault");

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
        return;
    }

    // Encode vault images as base64 data URIs.
    // Les assets Dioxus 0.7 doivent passer par asset!() pour être servis en dev ;
    // les fichiers copiés manuellement ne sont pas reconnus par dx serve et provoquent
    // des erreurs de routing. Le base64 inline est la seule approche fiable pour
    // du contenu dynamique dans un SPA Dioxus (pas de requête HTTP, marche partout).
    let vault_assets_src = Path::new(".vault/notes/.assets");
    let mut image_name_to_url: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(vault_assets_src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

        let mime = match ext.as_str() {
            "png"  => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif"  => "image/gif",
            "svg"  => "image/svg+xml",
            "webp" => "image/webp",
            "ico"  => "image/x-icon",
            _      => continue,
        };

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        match fs::read(path) {
            Ok(bytes) => {
                let mut b64 = String::with_capacity((bytes.len() * 4 / 3) + 4);
                const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                for chunk in bytes.chunks(3) {
                    let b0 = chunk[0] as usize;
                    let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
                    let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
                    b64.push(CHARS[(b0 >> 2)] as char);
                    b64.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
                    b64.push(if chunk.len() > 1 { CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char } else { '=' });
                    b64.push(if chunk.len() > 2 { CHARS[b2 & 0x3f] as char } else { '=' });
                }
                let data_uri = format!("data:{};base64,{}", mime, b64);
                image_name_to_url.insert(file_name.to_lowercase(), data_uri);
                println!("cargo:warning=Image encodée: {}", file_name);
            }
            Err(e) => println!("cargo:warning=Échec lecture image {}: {}", file_name, e),
        }
    }


    // 2. Parse markdown notes
    let matter = Matter::<YAML>::new();
    let mut raw_notes: Vec<(String, String, FrontMatter, String, String, String)> = Vec::new();
    let tag_regex = Regex::new(r"(?:^|\s)#([a-zA-Z0-9_\-]+)").unwrap();
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let wikilink_regex = Regex::new(r"\[\[([^\]\|]+)(?:\|([^\]]+))?\]\]").unwrap();

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

    // Map title/slug lookups
    let mut slug_to_title: HashMap<String, String> = HashMap::new();
    let mut stem_to_slug: HashMap<String, String> = HashMap::new();

    for (stem, slug, fm, _, _, _) in &raw_notes {
        let title = fm.title.clone().unwrap_or_else(|| stem.clone());
        slug_to_title.insert(slug.clone(), title);
        stem_to_slug.insert(stem.to_lowercase(), slug.clone());
    }

    // Process links and backlinks
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

    // Build final NoteMetaData map
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

        // 0. Remove inline hashtags from body so they only appear in the header tags list
        let body_no_tags = tag_regex.replace_all(&body, "");

        // 1. Process Obsidian Callouts > [!NOTE] Title
        let body_callouts = render_callouts(&body_no_tags);

        // 2. Process Obsidian Image embeds ![[image.png]] FIRST before wikilinks
        let embed_img_regex = Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();
        let processed_images = embed_img_regex.replace_all(&body_callouts, |caps: &regex::Captures| {
            let img_raw = caps[1].trim();
            let img_name = img_raw.to_lowercase();
            let url = image_name_to_url.get(&img_name).cloned().unwrap_or_else(|| {
                println!("cargo:warning=Image introuvable dans le vault: {}", img_raw);
                format!("data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7") // pixel transparent
            });
            format!("<img src=\"{}\" alt=\"{}\" class=\"rounded-lg my-4 shadow\" />", url, img_raw)
        });

        // 3. Process Wikilinks [[target|label]] or [[target]]
        let processed_body = wikilink_regex.replace_all(&processed_images, |caps: &regex::Captures| {
            let target = caps[1].trim();
            let label = caps.get(2).map(|m| m.as_str()).unwrap_or(target);
            let target_slug = stem_to_slug.get(&target.to_lowercase()).cloned().unwrap_or_else(|| slug::slugify(target));
            format!("<a href=\"/notes/{}\" class=\"wikilink\">{}</a>", target_slug, label)
        });

        // 4. Convert Markdown to HTML with pulldown-cmark
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(&processed_body, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // 5. Rewrite external links to open in new tab
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
                backlinks,
            },
        );
    }

    let json_output = serde_json::to_string_pretty(&notes_map).expect("Failed to serialize index.json");
    let target_file = Path::new(&out_dir).join("index.json");
    fs::write(target_file, json_output).expect("Failed to write index.json");
}