"""
style.py — Charte graphique commune du générateur d'images.

Tous les scripts de génération de visuels doivent importer ce module et
utiliser Colors / Fonts / styled_figure() plutôt que de redéfinir leurs
propres couleurs ou réglages matplotlib. Ça garantit que chaque image
produite (courbes, diagrammes, graphiques…) a le même rendu : fond bleu
nuit, texte blanc, dégradé de bleus pour les zones remplies.

Utilisation dans un script d'image :

    from style import Colors, Fonts, styled_figure

    fig, ax = styled_figure(figsize=(11, 7))
    ax.plot(x, y, color=Colors.CURVE)
    ...
    fig.savefig("sortie.png", dpi=150, facecolor=Colors.BACKGROUND)
"""

import matplotlib as mpl
import matplotlib.pyplot as plt


# ----------------------------------------------------------------------
# Palette de couleurs
# ----------------------------------------------------------------------
class Colors:
    BACKGROUND = "#1b2030"   # fond bleu nuit
    CURVE = "#ffffff"        # ligne de courbe / contours
    TEXT = "#ffffff"         # texte et annotations
    AXIS = "#ffffff"         # axes et ticks
    GRID = "#3a4256"         # grille discrète (si besoin)

    # Dégradé de bleus utilisé pour les zones remplies, du centre (plus
    # saturé) vers la périphérie (plus clair). Réutilisable pour tout
    # graphique à bandes/zones (distributions, intervalles, etc.)
    BAND_1 = "#2e6090"   # zone centrale
    BAND_2 = "#3f83b4"   # zone intermédiaire
    BAND_3 = "#8fc6e8"   # zone externe
    BAND_4 = "#bcdff0"   # zone la plus externe / queue


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
# Réglages globaux matplotlib
# ----------------------------------------------------------------------
def apply_global_style():
    """Applique les rcParams communs à tout le générateur d'images."""
    mpl.rcParams.update({
        "figure.facecolor": Colors.BACKGROUND,
        "axes.facecolor": Colors.BACKGROUND,
        "savefig.facecolor": Colors.BACKGROUND,
        "text.color": Colors.TEXT,
        "axes.edgecolor": Colors.AXIS,
        "axes.labelcolor": Colors.TEXT,
        "xtick.color": Colors.TEXT,
        "ytick.color": Colors.TEXT,
        "font.family": Fonts.FAMILY,
        "font.size": Fonts.SIZE_TICK,
        "axes.linewidth": 1.2,
    })


def styled_figure(figsize=(11, 7), spines=("left", "bottom")):
    """
    Crée une figure + un axe déjà stylés : fond sombre, cadre blanc fin
    uniquement sur les côtés indiqués, pas de grille par défaut.
    C'est le point d'entrée standard pour tout nouveau script d'image.
    """
    apply_global_style()
    fig, ax = plt.subplots(figsize=figsize)
    ax.set_facecolor(Colors.BACKGROUND)
    fig.patch.set_facecolor(Colors.BACKGROUND)

    for spine_name, spine in ax.spines.items():
        if spine_name in spines:
            spine.set_color(Colors.AXIS)
            spine.set_linewidth(1.2)
        else:
            spine.set_visible(False)

    ax.tick_params(colors=Colors.TEXT, labelsize=Fonts.SIZE_TICK)
    return fig, ax


def french_number(value, decimals=1):
    """Formate un nombre avec une virgule décimale (ex: 34.1 -> '34,1')."""
    return f"{value:.{decimals}f}".replace(".", ",")


def save(fig, output_path, dpi=150):
    """Sauvegarde une figure en respectant le fond de la charte graphique."""
    fig.tight_layout()
    fig.savefig(output_path, dpi=dpi, facecolor=Colors.BACKGROUND)
    return output_path