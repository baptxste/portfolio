"""
schema_renderer.py — Rendu épuré, flat et arrondi (sans gradients ni angles droits rigides).

Principes graphiques :
- Zéro dégradé : aplats de couleurs purs et lisibles.
- Formes fluides et douces : coins très arrondis (rx: 16-20px) et pills (rx: 9999px).
- Traits nets et arrondis : stroke-linecap: round, stroke-linejoin: round.
- Fond transparent avec halo de protection pour lisibilité Light / Dark mode.
"""

import json
import re
from pathlib import Path
import resvg_py
import styles


def get_default_defs() -> str:
    """Définitions SVG système épurées : marqueurs de flèches nets et arrondis."""
    return f"""
  <defs id="system-defs">
    <!-- Pointes de flèches soignées sans agressivité -->
    <marker id="arrow" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="{styles.Colors.ACCENT}" />
    </marker>
    <marker id="arrow-soft" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="{styles.Colors.BAND_2}" />
    </marker>
    <marker id="dot" markerWidth="6" markerHeight="6" refX="3" refY="3">
      <circle cx="3" cy="3" r="2.5" fill="{styles.Colors.ACCENT}" />
    </marker>
  </defs>
"""


def build_common_stylesheet() -> str:
    """Feuille de style CSS épurée : aplats plats, arrondis généreux, typographie nette."""
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

    /* Typographie moderne et lisible (Plus Jakarta Sans / Inter) */
    text {{
        font-family: 'Plus Jakarta Sans', system-ui, -apple-system, sans-serif;
        fill: {styles.Colors.TEXT};
        letter-spacing: -0.1px;
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

    /* Halo pour le Dark Mode : liseré blanc net */
    .halo {{
        paint-order: stroke fill;
        stroke: {styles.Colors.HALO};
        stroke-width: 3.5px;
        stroke-linecap: round;
        stroke-linejoin: round;
    }}

    /* Conteneurs très arrondis et plats (zéro dégradé, zéro ombre floue) */
    .card {{
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 18px;
        ry: 18px;
    }}

    .card-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 18px;
        ry: 18px;
    }}

    .card-soft {{
        fill: rgba(195, 221, 210, 0.45);
        stroke: {styles.Colors.BAND_2};
        stroke-width: 1.5px;
        rx: 16px;
        ry: 16px;
    }}

    .card-white {{
        fill: #ffffff;
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 18px;
        ry: 18px;
    }}

    .pill {{
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 9999px;
        ry: 9999px;
    }}

    .pill-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 9999px;
        ry: 9999px;
    }}

    .badge {{
        fill: {styles.Colors.BAND_1};
        stroke: {styles.Colors.HALO};
        stroke-width: 1.5px;
        rx: 8px;
        ry: 8px;
    }}

    .badge-text {{
        fill: #ffffff;
        font-size: 11px;
        font-weight: 600;
        font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    }}

    .node {{
        fill: {styles.Colors.BAND_3};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 2.2px;
    }}

    .container-dashed {{
        fill: rgba(195, 221, 210, 0.12);
        stroke: {styles.Colors.GRID};
        stroke-width: 1.5px;
        stroke-dasharray: 6 5;
        rx: 22px;
        ry: 22px;
    }}

    /* Connecteurs et flèches fluides */
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
        fill: {styles.Colors.BAND_4};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 16px;
        ry: 16px;
    }}
    .box-accent {{
        fill: {styles.Colors.ACCENT};
        stroke: {styles.Colors.ACCENT};
        stroke-width: 1.8px;
        rx: 16px;
        ry: 16px;
    }}
    .box-subtle {{
        fill: rgba(195, 221, 210, 0.45);
        stroke: {styles.Colors.GRID};
        stroke-width: 1.5px;
        rx: 14px;
        ry: 14px;
    }}
    .subtext {{
        fill: {styles.Colors.BAND_2};
        font-size: 11px;
    }}
    """


def extract_and_prepare_svg(content: str) -> str:
    """Extrait le SVG et y injecte les <defs> système si nécessaires."""
    match = re.search(r"(<svg[\s\S]*?</svg>)", content, re.IGNORECASE)
    svg = match.group(1) if match else content

    if 'id="system-defs"' not in svg:
        defs = get_default_defs()
        svg = re.sub(r"(<svg[^>]*>)", r"\1\n" + defs, svg, count=1, flags=re.IGNORECASE)

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
