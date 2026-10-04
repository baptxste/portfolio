import numpy as np

from styles import Colors, Fonts, save, styled_figure, halo

# Teintes de la palette du site : axes et points en vert de marque,
# directions principales en vert clair, vecteur v en vert intermédiaire.
COLOR_PRINCIPAL = Colors.BAND_3
COLOR_V = Colors.BAND_2

# Coordonnées en pixels de l'image d'origine (1388 x 800), y vers le bas
POINTS = np.array([
    (545, 444), (597, 386), (673, 360), (751, 288), (830, 267),
    (902, 255), (993, 233), (751, 346), (815, 336), (903, 328),
    (993, 311), (931, 387), (672, 456), (592, 498), (537, 557),
    (586, 581), (712, 524), (787, 532), (844, 456), (684, 606),
    (589, 656), (765, 413),
], dtype=float)

ORIGIN = (190, 741)
CENTER = (765, 413)          # point d'application des vecteurs
LINE_START = (500, 619)      # début de la droite principale
LINE_END = (1065, 182)       # pointe de la flèche principale
PERP_END = (682, 308)        # pointe de la direction orthogonale
V_END = (848, 348)           # pointe du vecteur v


def arrow(ax, start, end, color, linewidth, halo_width=None, scale=34):
    """Flèche à pointe ouverte, avec liseré clair pour fonds sombres."""
    ann = ax.annotate(
        "", xy=end, xytext=start,
        arrowprops=dict(arrowstyle="->", color=color, linewidth=linewidth,
                        shrinkA=0, shrinkB=0, mutation_scale=scale),
    )
    ann.arrow_patch.set_path_effects(
        halo(linewidth=halo_width or linewidth + 2.5)
    )
    return ann


def segment(ax, p, q, color, linewidth):
    ax.plot([p[0], q[0]], [p[1], q[1]], color=color, linewidth=linewidth,
            solid_capstyle="butt", path_effects=halo(linewidth=linewidth + 2.5))


def generate(output_path):
    fig, ax = styled_figure(figsize=(11, 6.3), spines=())
    ax.set_axis_off()
    ax.set_xlim(0, 1388)
    ax.set_ylim(800, 0)  # axe y inversé (repère image)
    ax.set_aspect("equal")

    # --- Axes X et Y ---
    arrow(ax, ORIGIN, (190, 45), Colors.AXIS, 3.2, scale=42)
    arrow(ax, ORIGIN, (1210, 741), Colors.AXIS, 3.2, scale=42)
    ax.text(1242, 762, "X", ha="center", va="center", color=Colors.TEXT,
            fontsize=Fonts.SIZE_TITLE * 1.5, path_effects=halo(linewidth=4))
    ax.text(148, 48, "Y", ha="center", va="center", color=Colors.TEXT,
            fontsize=Fonts.SIZE_TITLE * 1.5, path_effects=halo(linewidth=4))

    # --- Nuage de points (croix) ---
    ax.scatter(POINTS[:, 0], POINTS[:, 1], marker="x", s=130,
               linewidths=2.8, color=Colors.AXIS, zorder=3,
               path_effects=halo(linewidth=5))

    # --- Droite principale (fine) puis tronçon épais jusqu'à la flèche ---
    segment(ax, LINE_START, CENTER, COLOR_PRINCIPAL, 3.2)
    segment(ax, CENTER, LINE_END, COLOR_PRINCIPAL, 5)
    arrow(ax, (1030, 205), LINE_END, COLOR_PRINCIPAL, 3.5)

    # --- Direction orthogonale ---
    segment(ax, CENTER, PERP_END, COLOR_PRINCIPAL, 5)
    arrow(ax, (700, 330), PERP_END, COLOR_PRINCIPAL, 3.5)

    # --- Vecteur propre v ---
    arrow(ax, CENTER, V_END, COLOR_V, 3.5)
    ax.text(812, 410, r"$v$", ha="center", va="center", color=COLOR_V,
            fontsize=Fonts.SIZE_TITLE * 1.4, path_effects=halo(linewidth=4))

    return save(fig, output_path)


if __name__ == "__main__":
    from pathlib import Path

    default_out = (
        Path(__file__).resolve().parent.parent.parent.parent
        / ".vault"
        / "notes"
        / "assets"
        / "maths"
        / "pca_axe_principal.png"
    )
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")