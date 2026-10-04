import numpy as np

from styles import Colors, Fonts, french_number, save, styled_figure, halo

# Les deux courbes reprennent la palette du site : une teinte saturée et
# une teinte claire de la même famille.
CURVE_CONCENTREE = Colors.BAND_1  # variance concentrée sur peu d'axes
CURVE_DIFFUSE = Colors.BAND_3     # variance répartie sur beaucoup d'axes

N_AXES = 12.5      # nombre d'axes pour atteindre 100 % (courbe diffuse)
SEUIL = 0.9        # seuil de variance conservée repéré sur l'axe y


def concentree(k):
    """Rapport cumulé qui monte vite puis se stabilise vers 1."""
    return 1 - np.exp(-(k - 1) / 1.1)


def diffuse(k):
    """Rapport cumulé quasi linéaire (légèrement en S) : variance répartie."""
    t = np.clip((k - 1) / (N_AXES - 1), 0, 1)
    return t - 0.07 * np.sin(2 * np.pi * t)


def generate(output_path):
    fig, ax = styled_figure(figsize=(11, 7), spines=("left", "bottom"))

    k = np.linspace(1, 13, 400)

    # Les deux courbes partent de l'origine (k = 1) et finissent à 1
    ax.plot(k, concentree(k), color=CURVE_CONCENTREE, linewidth=2.8,
            path_effects=halo(linewidth=6))
    ax.plot(k, diffuse(k), color=CURVE_DIFFUSE, linewidth=2.8,
            path_effects=halo(linewidth=6))

    # Axe X : numéro de l'axe principal (1, 2, 3, 4, ...)
    ax.set_xlim(0, 13.5)
    ax.set_xticks([1, 2, 3, 4])
    ax.set_xticklabels(["1", "2", "3", "4"], fontsize=Fonts.SIZE_TICK)

    # Axe Y : rapport cumulé, avec le seuil de 0,9 repéré (virgule française)
    ax.set_ylim(0, 1.05)
    ax.set_yticks([0, SEUIL])
    ax.set_yticklabels(["0", french_number(SEUIL)], fontsize=Fonts.SIZE_TICK)

    # Halo sur les graduations (les labels n'existent qu'après set_ticklabels)
    for label in ax.get_xticklabels() + ax.get_yticklabels():
        label.set_path_effects(halo())

    return save(fig, output_path)


if __name__ == "__main__":
    from pathlib import Path

    default_out = (
        Path(__file__).resolve().parent.parent.parent.parent
        / ".vault"
        / "notes"
        / "assets"
        / "maths"
        / "pca_var_cumulee.png"
    )
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")