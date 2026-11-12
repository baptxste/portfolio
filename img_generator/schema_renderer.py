"""
schema_renderer.py — Rendu épuré, flat et garanti 100% lisible.

Règles de rendu :
- Formes douces : arrondis explicites (rx/ry) injectés automatiquement si omis.
- Lisibilité maximale : fort contraste texte/fond. Ne jamais forcer fill sur 'text'
  afin de préserver les attributs fill="#ffffff" et classes de contraste.
- Zéro dégradé : aplats nets et lisibles sur fond clair et sombre.
"""

import json
import re
from pathlib import Path
import resvg_py
import styles


def get_default_defs() -> str:
    """Définitions SVG système : marqueurs de flèches épurés."""
    return f"""
  <defs id="system-defs">
    <marker id="arrow" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="{styles.Colors.ACCENT}" />
    </marker>
    <marker id="arrow-soft" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="{styles.Colors.BAND_2}" />
    </marker>
    <marker id="arrow-gold" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="#cba258" />
    </marker>
    <marker id="dot" markerWidth="6" markerHeight="6" refX="3" refY="3">
      <circle cx="3" cy="3" r="2.5" fill="{styles.Colors.ACCENT}" />
    </marker>
  </defs>
"""


def build_common_stylesheet() -> str:
    """Feuille de style CSS garantissant un contraste et une lisibilité parfaits."""
    return f"""
    :root {{
        --color-accent: {styles.Colors.ACCENT};
        --color-halo: {styles.Colors.HALO};
        --color-band-1: {styles.Colors.BAND_1};
        --color-band-2: {styles.Colors.BAND_2};
        --color-band-3: {styles.Colors.BAND_3};
        --color-band-4: {styles.Colors.BAND_4};
        --color-grid: {styles.Colors.GRID};
    }}

    /* Typographie : couleur de marque par défaut pour éviter tout texte noir */
    text {{
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", "Helvetica Neue", sans-serif;
        letter-spacing: -0.1px;
        fill: {styles.Colors.TEXT};
    }}

    .title {{
        font-size: 14px;
        font-weight: 700;
        fill: {styles.Colors.ACCENT};
    }}

    .subtitle {{
        font-size: 11px;
        font-weight: 500;
        fill: {styles.Colors.BAND_2};
    }}

    .text-white, text[fill="#ffffff"] {{
        fill: #ffffff !important;
    }}

    .text-accent {{
        fill: {styles.Colors.ACCENT} !important;
    }}

    /* Halo fin pour isoler le texte sur le fond sans noyer les petites lettres */
    .halo {{
        paint-order: stroke fill;
        stroke: {styles.Colors.HALO};
        stroke-width: 2px;
        stroke-linecap: round;
        stroke-linejoin: round;
    }}

    /* Cartes & Blocs par défaut : fond blanc pur et contour vert sapin */
    .card {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}
    .card-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}
    .card-soft {{
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.5px;
    }}

    /* Formes spécifiques aux bases de données */
    .db-body {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.6px;
    }}
    .db-top {{
        fill: #d4e9e2;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.6px;
    }}

    /* Pastilles d'état (vert vif, or, bleu) */
    .dot-live {{
        fill: #10b981;
    }}
    .dot-gold {{
        fill: #cba258;
    }}
    .dot-blue {{
        fill: #3b82f6;
    }}

    .card-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}

    .card-soft {{
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.5px;
    }}

    /* Capsules / Pilules douces inspirées du design system du site */
    .pill {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}

    .pill-accent {{
        fill: #d4e9e2;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.6px;
    }}

    .pill-soft {{
        fill: rgba(212, 233, 226, 0.45);
        stroke: {styles.Colors.BAND_2};
        stroke-width: 1.4px;
    }}

    /* Texte dans les pilules accentuées : vert sapin foncé bien net sur fond vert pastel doux */
    .pill-accent text, .card-accent text, g.pill-accent text, g.card-accent text {{
        fill: {styles.Colors.ACCENT};
        font-weight: 600;
    }}

    .badge {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.HALO};
        stroke-width: 1.2px;
    }}

    .badge-text {{
        fill: #ffffff;
        font-size: 11px;
        font-weight: 600;
        font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    }}

    .node {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 2.2px;
    }}

    .container-dashed {{
        fill: none;
        stroke: {styles.Colors.GRID};
        stroke-width: 1.5px;
        stroke-dasharray: 6 5;
    }}

    /* Connecteurs */
    .arrow {{
        stroke: {styles.Colors.ACCENT};
        stroke-width: 2px;
        fill: none;
        stroke-linecap: round;
        stroke-linejoin: round;
    }}

    .arrow-dashed {{
        stroke: {styles.Colors.BAND_2};
        stroke-width: 1.8px;
        fill: none;
        stroke-dasharray: 5 4;
        stroke-linecap: round;
        stroke-linejoin: round;
    }}

    .grid-line {{
        stroke: {styles.Colors.GRID};
        stroke-width: 1.2px;
        stroke-dasharray: 4 4;
    }}

    /* Rétrocompatibilité */
    .box {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}
    .box-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
    }}
    .box-subtle {{
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.BAND_2};
        stroke-width: 1.5px;
    }}
    .subtext {{
        fill: {styles.Colors.BAND_2};
        font-size: 11px;
    }}
    """


def inject_missing_rx(svg: str) -> str:
    """Injecte automatiquement rx/ry sur les <rect> de classe pill, card, box s'ils manquent."""
    def add_rx_pill(match):
        attrs = match.group(0)
        if "rx=" not in attrs:
            return attrs.replace("<rect", '<rect rx="24" ry="24"')
        return attrs

    def add_rx_card(match):
        attrs = match.group(0)
        if "rx=" not in attrs:
            return attrs.replace("<rect", '<rect rx="18" ry="18"')
        return attrs

    svg = re.sub(r'<rect[^>]*class=["\'][^"\']*pill[^"\']*["\'][^>]*>', add_rx_pill, svg)
    svg = re.sub(r'<rect[^>]*class=["\'][^"\']*(?:card|box|container)[^"\']*["\'][^>]*>', add_rx_card, svg)
    return svg


def extract_and_prepare_svg(content: str) -> str:
    """Extrait le SVG, injecte les defs et s'assure des arrondis."""
    match = re.search(r"(<svg[\s\S]*?</svg>)", content, re.IGNORECASE)
    svg = match.group(1) if match else content

    if 'id="system-defs"' not in svg:
        defs = get_default_defs()
        svg = re.sub(r"(<svg[^>]*>)", r"\1\n" + defs, svg, count=1, flags=re.IGNORECASE)

    svg = inject_missing_rx(svg)
    return svg


def render_schema(schema_file: Path, schemas_dir: Path, assets_dir: Path) -> bool:
    """Rend un unique fichier .html ou .svg en .png."""
    rel_path = schema_file.relative_to(schemas_dir)
    target_png = assets_dir / rel_path.with_suffix(".png")
    target_png.parent.mkdir(parents=True, exist_ok=True)

    json_path = schema_file.with_suffix(".json")
    config = {}
    if json_path.exists():
        try:
            with open(json_path, "r", encoding="utf-8") as f:
                config = json.load(f)
        except Exception as e:
            print(f"⚠️  [WARN] Erreur de lecture de {json_path.name}: {e}")

    try:
        raw_content = schema_file.read_text(encoding="utf-8")
        svg_content = extract_and_prepare_svg(raw_content)

        css = build_common_stylesheet()
        if "extra_css" in config:
            css += f"\n{config['extra_css']}"

        png_bytes = resvg_py.svg_to_bytes(
            svg_string=svg_content,
            width=config.get("width"),
            height=config.get("height"),
            zoom=config.get("zoom"),
            dpi=config.get("dpi", 0.0),
            style_sheet=css,
        )

        target_png.write_bytes(png_bytes)
        print(f"✅ [SUCCESS] {rel_path} -> {target_png.name}")
        return True

    except Exception as e:
        print(f"❌ [FAILED] {rel_path}: {e}")
        return False


def find_schemas(schemas_dir: Path) -> list[Path]:
    """Trouve tous les fichiers .html et .svg dans le dossier des schémas."""
    if not schemas_dir.exists():
        return []
    schemas = []
    for path in sorted(schemas_dir.rglob("*")):
        if path.is_file() and path.suffix.lower() in {".html", ".svg"}:
            if path.name.startswith((".", "_")):
                continue
            schemas.append(path)
    return schemas
