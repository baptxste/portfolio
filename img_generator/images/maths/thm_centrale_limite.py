import numpy as np
from matplotlib.ticker import FuncFormatter

from styles import Colors, Fonts, save, styled_figure


def normal_pdf(x, mu=0.0, sigma=1.0):
    return (1 / (sigma * np.sqrt(2 * np.pi))) * np.exp(-0.5 * ((x - mu) / sigma) ** 2)


def generate(output_path):
    fig, ax = styled_figure(figsize=(11, 7), spines=("left", "bottom"))

    x = np.linspace(-4, 4, 2000)
    y = normal_pdf(x)

    # Bandes remplies (du centre vers l'extérieur), symétriques
    bands = [
        (-1, 1, Colors.BAND_1),
        (-2, -1, Colors.BAND_2),
        (1, 2, Colors.BAND_2),
        (-3, -2, Colors.BAND_3),
        (2, 3, Colors.BAND_3),
        (-4, -3, Colors.BAND_4),
        (3, 4, Colors.BAND_4),
    ]
    for lo, hi, color in bands:
        mask = (x >= lo) & (x <= hi)
        ax.fill_between(x[mask], y[mask], color=color, linewidth=0)

    # Courbe blanche par-dessus les zones
    ax.plot(x, y, color=Colors.CURVE, linewidth=1.8)

    # Lignes verticales aux frontières d'écart-type (+ au centre)
    for s in [-3, -2, -1, 0, 1, 2, 3]:
        ax.plot([s, s], [0, normal_pdf(s)], color=Colors.CURVE, linewidth=1)

    # Étiquettes de pourcentage (virgule décimale à la française)
    labels = [
        (-3.5, 0.014, "0,1%"),
        (-2.5, 0.032, "2,1%"),
        (-1.5, 0.078, "13,6%"),
        (-0.5, 0.17, "34,1%"),
        (0.5, 0.17, "34,1%"),
        (1.5, 0.078, "13,6%"),
        (2.5, 0.032, "2,1%"),
        (3.5, 0.014, "0,1%"),
    ]
    for xpos, ypos, text in labels:
        ax.text(
            xpos,
            ypos,
            text,
            ha="center",
            va="center",
            color=Colors.TEXT,
            fontsize=Fonts.SIZE_ANNOTATION,
        )

    # Axe X en unités d'écart-type (sigma)
    ax.set_xticks(range(-3, 4))
    ax.set_xticklabels(
        [f"{s}σ" if s != 0 else "0" for s in range(-3, 4)],
        fontsize=Fonts.SIZE_TICK,
    )
    ax.set_xlim(-4, 4)

    # Axe Y avec point décimal classique
    ax.set_ylim(0, 0.45)
    ax.set_yticks(np.arange(0, 0.46, 0.05))
    ax.yaxis.set_major_formatter(FuncFormatter(lambda v, _: f"{v:.2f}"))

    return save(fig, output_path)


if __name__ == "__main__":
    from pathlib import Path

    default_out = (
        Path(__file__).resolve().parent.parent.parent.parent
        / ".vault"
        / "notes"
        / "assets"
        / "maths"
        / "thm_centrale_limite.png"
    )
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")
