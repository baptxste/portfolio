import numpy as np

from styles import Colors, Fonts, save, styled_figure, halo

# Les deux classes reprennent la palette du site : une teinte saturée et
# une teinte claire de la même famille. Le liseré blanc (Colors.HALO) autour
# des points les détache du fond sombre ; sur fond clair il ne se voit pas.
CLASS_1 = Colors.BAND_1
CLASS_2 = Colors.BAND_3
EDGE = dict(edgecolors=Colors.HALO, linewidths=1.5)

# Coordonnées en pixels de l'image d'origine (1354 x 454), y vers le bas
CLASS_1_PTS = np.array([
    (123, 105), (193, 92), (234, 78), (277, 92),
    (165, 132), (220, 118), (193, 159), (123, 174),
], dtype=float)
CLASS_2_PTS = np.array([
    (265, 192), (348, 207), (307, 220), (252, 232),
    (195, 234), (154, 262), (195, 288), (154, 301),
], dtype=float)

# Séparateurs verticaux entre les trois panneaux
SEPARATORS = [447, 900]
# Décalages horizontaux des panneaux 2 et 3 par rapport au panneau 1
OFFSET_2 = 411
OFFSET_3 = 948


def project(points, a, b):
    """Projection orthogonale de `points` sur la droite (a -> b)."""
    d = (b - a) / np.linalg.norm(b - a)
    return a + np.outer((points - a) @ d, d)


def draw_arrow(ax, start, end):
    ann = ax.annotate(
        "", xy=end, xytext=start,
        arrowprops=dict(arrowstyle="->", color=Colors.AXIS,
                        linewidth=1.8, shrinkA=0, shrinkB=0,
                        mutation_scale=28),
    )
    ann.arrow_patch.set_path_effects(halo(linewidth=4))


def draw_panel(ax, red, blue, a=None, b=None):
    """Dessine un panneau ; si (a, b) est donné, ajoute la droite de projection."""
    if a is not None:
        a, b = np.array(a, dtype=float), np.array(b, dtype=float)
        for pts, color in ((red, CLASS_1), (blue, CLASS_2)):
            proj = project(pts, a, b)
            for p, q in zip(pts, proj):
                ax.plot([p[0], q[0]], [p[1], q[1]], color=Colors.AXIS,
                        linewidth=1.2, linestyle=(0, (4, 2)),
                        path_effects=halo(linewidth=2.5), zorder=1)
            ax.scatter(proj[:, 0], proj[:, 1], s=40, color=color, zorder=6, **EDGE)
        draw_arrow(ax, a, b)
    ax.scatter(red[:, 0], red[:, 1], s=210, color=CLASS_1, zorder=4, **EDGE)
    ax.scatter(blue[:, 0], blue[:, 1], s=210, color=CLASS_2, zorder=4, **EDGE)


def generate(output_path):
    fig, ax = styled_figure(figsize=(12, 4), spines=())
    ax.set_axis_off()
    ax.set_xlim(0, 1354)
    ax.set_ylim(454, 0)  # axe y inversé (repère image)
    ax.set_aspect("equal")

    # Séparateurs entre panneaux
    for x in SEPARATORS:
        ax.plot([x, x], [0, 454], color=Colors.GRID, linewidth=2.5,
                path_effects=halo(linewidth=4))

    # Panneau 1 : nuage de points brut
    draw_panel(ax, CLASS_1_PTS, CLASS_2_PTS)

    # Panneau 2 : projection sur un axe qui mélange les deux classes
    shift2 = np.array([OFFSET_2, 0])
    draw_panel(ax, CLASS_1_PTS + shift2, CLASS_2_PTS + shift2,
               a=(515, 381), b=(865, 247))

    # Panneau 3 : projection sur un axe qui sépare bien les deux classes
    shift3 = np.array([OFFSET_3, 0])
    draw_panel(ax, CLASS_1_PTS + shift3, CLASS_2_PTS + shift3,
               a=(1101, 386), b=(980, 93))

    return save(fig, output_path)


if __name__ == "__main__":
    from pathlib import Path

    default_out = (
        Path(__file__).resolve().parent.parent.parent.parent
        / ".vault"
        / "notes"
        / "assets"
        / "maths"
        / "lda_projection.png"
    )
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")