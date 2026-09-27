"""
style.py — Charte graphique commune du générateur d'images.

Le site a un thème clair et un thème sombre, mais chaque graphique n'est
généré qu'UNE seule fois (un seul fichier PNG, pas de variante par
thème). Pour que cette image reste lisible sur les deux fonds, le style
repose sur deux idées :

  - fond TRANSPARENT : l'image n'impose plus son propre fond (fini le
    bloc bleu marine qui jurait aussi bien sur fond blanc que sur fond
    sombre) — elle se pose directement sur la page, quel que soit le
    thème actif.
  - halo clair autour des traits et du texte : la courbe, les axes et
    les libellés sont dessinés dans le vert de marque du site
    (#286044), avec un léger liseré blanc. Sur fond clair, ce halo est
    quasiment invisible et seul le vert ressort (comme les titres du
    site). Sur fond sombre, ce même halo fait ressortir le trait / le
    texte, qui sans lui se fondrait dans le fond vert nuit du site.

Les zones remplies (bandes de la courbe, aires…) restent des aplats
opaques : elles n'ont pas besoin de halo, elles se voient sur n'importe
quel fond par construction — seuls les TRAITS FINS (courbe, axes,
texte) ont besoin de ce traitement.

Utilisation dans un script d'image :

    from style import Colors, Fonts, styled_figure, halo, save

    fig, ax = styled_figure(figsize=(11, 7))
    ax.plot(x, y, color=Colors.CURVE, linewidth=2.5, path_effects=halo())
    ax.fill_between(x, y, color=Colors.BAND_1)
    ax.text(0, 0.1, "34,1%", color=Colors.TEXT,
            ha="center", path_effects=halo())
    ...
    save(fig, "sortie.png")
"""

import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.patheffects as pe


# ----------------------------------------------------------------------
# Palette de couleurs
# ----------------------------------------------------------------------
class Colors:
    # Vert sapin — couleur de marque du site (logo "BC", titres, onglet
    # actif). Sert pour la courbe, les axes, les textes.
    ACCENT = "#286044"
    CURVE = ACCENT
    TEXT = ACCENT
    AXIS = ACCENT

    # Liseré clair posé derrière traits et textes (cf. halo() plus bas)
    # pour rester lisible sur le fond sombre du site.
    HALO = "#ffffff"

    # Dégradé du centre (saturé) vers la périphérie (clair), dans la
    # même famille que ACCENT. Aplats opaques : pas besoin de halo.
    BAND_1 = "#286044"  # zone centrale (= ACCENT)
    BAND_2 = "#4a8a6e"
    BAND_3 = "#7fb39a"
    BAND_4 = "#c3ddd2"

    GRID = "#a9c2b8"


# ----------------------------------------------------------------------
# Typographie
# ----------------------------------------------------------------------
class Fonts:
    FAMILY = "DejaVu Sans"
    SIZE_TITLE = 16
    SIZE_LABEL = 13
    SIZE_TICK = 12
    SIZE_ANNOTATION = 13


# ----------------------------------------------------------------------
# Halo : liseré clair posé derrière un trait ou un texte, pour rester
# lisible qu'on soit sur fond clair ou sur fond sombre.
# ----------------------------------------------------------------------
def halo(linewidth=3, color=None):
    """
    À passer en path_effects=... sur ax.plot(), ax.text(), les tick
    labels, les spines, etc. Dessine un liseré `color` (blanc par
    défaut) autour du trait, avec le trait lui-même par-dessus.
    """
    color = color or Colors.HALO
    return [pe.withStroke(linewidth=linewidth, foreground=color), pe.Normal()]


# ----------------------------------------------------------------------
# Réglages globaux matplotlib
# ----------------------------------------------------------------------
def apply_global_style():
    """Applique les rcParams communs à tout le générateur d'images."""
    mpl.rcParams.update({
        "figure.facecolor": "none",
        "axes.facecolor": "none",
        "savefig.facecolor": "none",
        "savefig.transparent": True,
        "text.color": Colors.TEXT,
        "axes.edgecolor": Colors.AXIS,
        "axes.labelcolor": Colors.TEXT,
        "xtick.color": Colors.AXIS,
        "ytick.color": Colors.AXIS,
        "font.family": Fonts.FAMILY,
        "font.size": Fonts.SIZE_TICK,
        "axes.linewidth": 1.4,
    })


def styled_figure(figsize=(11, 7), spines=("left", "bottom")):
    """
    Crée une figure + un axe déjà stylés : fond transparent, cadre fin
    (avec halo) uniquement sur les côtés indiqués, pas de grille par
    défaut. Point d'entrée standard pour tout nouveau script d'image.
    """
    apply_global_style()
    fig, ax = plt.subplots(figsize=figsize)
    ax.patch.set_alpha(0)
    fig.patch.set_alpha(0)

    for spine_name, spine in ax.spines.items():
        if spine_name in spines:
            spine.set_color(Colors.AXIS)
            spine.set_linewidth(1.4)
            spine.set_path_effects(halo(linewidth=4))
        else:
            spine.set_visible(False)

    ax.tick_params(colors=Colors.AXIS, labelsize=Fonts.SIZE_TICK)
    for label in ax.get_xticklabels() + ax.get_yticklabels():
        label.set_path_effects(halo())

    return fig, ax


def french_number(value, decimals=1):
    """Formate un nombre avec une virgule décimale (ex: 34.1 -> '34,1')."""
    return f"{value:.{decimals}f}".replace(".", ",")


def save(fig, output_path, dpi=150):
    """Sauvegarde une figure en PNG à fond transparent."""
    fig.tight_layout()
    fig.savefig(output_path, dpi=dpi, transparent=True)
    return output_path