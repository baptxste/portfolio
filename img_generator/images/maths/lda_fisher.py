import numpy as np
from matplotlib.patches import Rectangle

from styles import Colors, Fonts, save, styled_figure, halo

# Les deux classes reprennent la palette du site : une teinte saturée et
# une teinte claire de la même famille. Le liseré blanc (Colors.HALO) autour
# des marqueurs les détache du fond sombre ; sur fond clair il ne se voit pas.
CLASS_1 = Colors.BAND_1   # cercles / μ1
CLASS_2 = Colors.BAND_3   # carrés  / μ2
EDGE = dict(edgecolors=Colors.HALO, linewidths=1.5)

# Coordonnées en pixels de l'image d'origine (1398 x 898), y vers le bas
CLASS_1_2D = [(736, 126), (808, 148), (897, 143), (651, 166),
          (602, 188), (852, 191), (691, 219), (780, 219)]
CLASS_2_2D = [(985, 283), (1075, 292), (802, 339), (895, 347),
            (834, 376), (1075, 368), (1156, 347)]

# Projections sur l'axe horizontal (bas)
CLASS_1_X = [535, 585, 625, 668, 740, 787, 853]
CLASS_2_X = [701, 815, 918, 1008, 1090]
Y_BOTTOM = 800

# Projections sur l'axe vertical (gauche)
X_LEFT = 52

# Axes principaux
X_ORIGIN, Y_ORIGIN = 450, 637

# Moyennes (μ1, μ2 sur l'axe x ; μ̃1, μ̃2 sur l'axe y)
MU1 = (735, 188)
MU2 = (1012, 328)


def arrow(ax, start, end):
    ann = ax.annotate(
        "", xy=end, xytext=start,
        arrowprops=dict(arrowstyle="-|>", color=Colors.AXIS, linewidth=2,
                        shrinkA=0, shrinkB=0, mutation_scale=34),
    )
    ann.arrow_patch.set_path_effects(halo(linewidth=4))


def dotted(ax, p, q):
    ax.plot([p[0], q[0]], [p[1], q[1]], color=Colors.AXIS, linewidth=3,
            linestyle=(0, (1, 2)), path_effects=halo(linewidth=5), zorder=2)


def label(ax, x, y, text):
    ax.text(x, y, text, ha="center", va="center", color=Colors.TEXT,
            fontsize=Fonts.SIZE_TITLE * 2, style="italic",
            path_effects=halo(linewidth=4), zorder=6)


def generate(output_path):
    fig, ax = styled_figure(figsize=(11, 7), spines=())
    ax.set_axis_off()
    ax.set_xlim(0, 1398)
    ax.set_ylim(898, 0)  # axe y inversé (repère image)
    ax.set_aspect("equal")

    # --- Axes ---
    arrow(ax, (X_ORIGIN, 655), (X_ORIGIN, 62))          # axe y du nuage 2D
    arrow(ax, (362, Y_ORIGIN), (1255, Y_ORIGIN))        # axe x du nuage 2D
    arrow(ax, (X_LEFT, 750), (X_LEFT, 160))             # projection gauche
    arrow(ax, (293, Y_BOTTOM - 5), (1190, Y_BOTTOM - 5))  # projection bas

    # --- Nuage 2D ---
    ax.scatter(*zip(*CLASS_1_2D), s=520, color=CLASS_1, zorder=4, **EDGE)
    ax.scatter(*zip(*CLASS_2_2D), s=520, marker="s", color=CLASS_2, zorder=4, **EDGE)

    # --- Pointillés vers les moyennes ---
    dotted(ax, (X_ORIGIN, MU1[1]), MU1)
    dotted(ax, MU1, (MU1[0], 622))
    dotted(ax, (X_ORIGIN, MU2[1]), (960, MU2[1]))
    dotted(ax, (MU2[0], 370), (MU2[0], 620))

    # --- Triangles des moyennes (sur les axes) ---
    ax.scatter([X_ORIGIN], [MU1[1]], s=420, marker="^", color=CLASS_1, zorder=5, **EDGE)
    ax.scatter([X_ORIGIN], [MU2[1]], s=420, marker="^", color=CLASS_2, zorder=5, **EDGE)
    ax.scatter([MU1[0]], [642], s=420, marker="^", color=CLASS_1, zorder=5, **EDGE)
    ax.scatter([MU2[0]], [637], s=420, marker="^", color=CLASS_2, zorder=5, **EDGE)

    # --- Projection sur l'axe x (bas) ---
    ax.scatter(CLASS_1_X, [Y_BOTTOM] * len(CLASS_1_X), s=520, color=CLASS_1, zorder=4, **EDGE)
    ax.scatter(CLASS_2_X, [Y_BOTTOM] * len(CLASS_2_X), s=520, marker="s",
               color=CLASS_2, zorder=5, **EDGE)

    # --- Projection sur l'axe y (gauche) : points qui se chevauchent ---
    ax.scatter([X_LEFT] * 6, np.linspace(268, 358, 6), s=520, color=CLASS_1,
               zorder=4, **EDGE)
    ax.add_patch(Rectangle((X_LEFT - 20, 402), 40, 50, facecolor=CLASS_2, zorder=4,
                           edgecolor=Colors.HALO, linewidth=1.5))
    ax.add_patch(Rectangle((X_LEFT - 20, 460), 40, 76, facecolor=CLASS_2, zorder=4,
                           edgecolor=Colors.HALO, linewidth=1.5))

    # --- Étiquettes ---
    label(ax, 372, 185, r"$\tilde{\mu}_1$")
    label(ax, 366, 358, r"$\tilde{\mu}_2$")
    label(ax, 745, 180, r"$\mu_1$")
    label(ax, 1020, 332, r"$\mu_2$")
    label(ax, 732, 722, r"$\hat{\mu}_1$")
    label(ax, 1018, 735, r"$\hat{\mu}_2$")

    return save(fig, output_path)


if __name__ == "__main__":
    from pathlib import Path

    default_out = (
        Path(__file__).resolve().parent.parent.parent.parent
        / ".vault"
        / "notes"
        / "assets"
        / "maths"
        / "lda_fisher.png"
    )
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")