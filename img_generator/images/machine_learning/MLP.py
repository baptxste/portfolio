"""Dessine un perceptron multicouche (MLP) dans la charte graphique du site.

Usage :
    python mlp.py                 # architecture par défaut (4-6-6-3)
    python mlp.py 4 8 8 3         # tailles de couches personnalisées
"""
import sys
from pathlib import Path

from matplotlib.patches import Circle

from styles import Colors, Fonts, save, styled_figure, halo

# Les trois types de couches reprennent la palette du site : teintes
# claire (entrée), saturée (cachées) et intermédiaire (sortie).
COLOR_INPUT = Colors.BAND_3
COLOR_HIDDEN = Colors.BAND_1
COLOR_OUTPUT = Colors.BAND_2

COLOR_LINKS = Colors.GRID   # connexions entre neurones

H_SPACING = 3.0    # espace horizontal entre deux couches
V_SPACING = 1.2    # espace vertical entre deux neurones
RADIUS = 0.4       # rayon d'un neurone
MAX_NODES = 10     # neurones affichés par couche (le reste -> "⋮")


def layer_color(i, n_layers):
    """Couleur d'aplat d'une couche selon son rôle."""
    if i == 0:
        return COLOR_INPUT
    if i == n_layers - 1:
        return COLOR_OUTPUT
    return COLOR_HIDDEN


def layer_title(i, n_layers, size):
    """Libellé affiché au-dessus d'une couche."""
    if i == 0:
        return f"Entrée\n({size})"
    if i == n_layers - 1:
        return f"Sortie\n({size})"
    return f"Cachée {i}\n({size})"


def generate(output_path, layer_sizes=(4, 6, 6, 3)):
    n_layers = len(layer_sizes)
    fig, ax = styled_figure(figsize=(2.8 * n_layers, 7), spines=())

    # Positions des neurones (couches tronquées si trop grandes)
    positions = []
    for i, size in enumerate(layer_sizes):
        n = min(size, MAX_NODES)
        x = i * H_SPACING
        top = (n - 1) * V_SPACING / 2
        positions.append([(x, top - j * V_SPACING) for j in range(n)])

    # Connexions : traits fins, dessinés sous les neurones
    for i in range(n_layers - 1):
        for (x1, y1) in positions[i]:
            for (x2, y2) in positions[i + 1]:
                ax.plot([x1, x2], [y1, y2], color=COLOR_LINKS,
                        linewidth=0.9, alpha=0.8, zorder=1)

    # Neurones : aplats opaques (pas de halo nécessaire)
    for i, layer in enumerate(positions):
        for (x, y) in layer:
            ax.add_patch(Circle((x, y), RADIUS,
                                facecolor=layer_color(i, n_layers),
                                edgecolor=Colors.ACCENT, linewidth=1.6,
                                zorder=2))
        # Points de suspension si la couche est tronquée
        if layer_sizes[i] > MAX_NODES:
            ax.text(layer[0][0], layer[-1][1] - V_SPACING * 0.9, "⋮",
                    color=Colors.TEXT, ha="center", va="center",
                    fontsize=Fonts.SIZE_TITLE + 4,
                    path_effects=halo())

    # Titres des couches : texte avec halo
    y_top = max(p[0][1] for p in positions) + 1.2
    for i, size in enumerate(layer_sizes):
        ax.text(i * H_SPACING, y_top, layer_title(i, n_layers, size),
                color=Colors.TEXT, ha="center", va="bottom",
                fontsize=Fonts.SIZE_LABEL, fontweight="bold",
                path_effects=halo())

    # Cadrage (axes masqués : seul le schéma est visible)
    n_max = min(max(layer_sizes), MAX_NODES)
    half = (n_max - 1) * V_SPACING / 2
    ax.set_aspect("equal")
    ax.set_xlim(-1, (n_layers - 1) * H_SPACING + 1)
    ax.set_ylim(-half - 1.5, y_top + 1.5)
    ax.axis("off")

    return save(fig, output_path)


if __name__ == "__main__":
    sizes = tuple(int(a) for a in sys.argv[1:]) or (4, 6, 6, 3)

    default_out = Path(__file__).resolve().parent / "mlp.png"
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out, sizes)
    print(f"Image générée : {path}")