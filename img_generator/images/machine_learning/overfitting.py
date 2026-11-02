"""Illustre bon ajustement vs sur-apprentissage, dans la charte du site.

Deux graphes côte à côte, mêmes points d'entraînement :
  - un modèle de capacité adaptée, qui suit la tendance ;
  - un modèle trop flexible, qui passe par tous les points (erreur
    d'entraînement nulle) mais oscille entre eux.

Usage :
    python overfitting.py
"""
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from styles import Colors, Fonts, apply_global_style, halo, save

CURVE_MODEL = Colors.BAND_1       # courbe apprise par le modèle
POINTS = Colors.BAND_3            # points d'entraînement (aplats)
# Points de test : orange, hors palette verte, pour bien les distinguer
# des points d'entraînement (à déplacer dans styles.py si réutilisé).
POINTS_TEST = "#d9822b"
CURVE_TRUE = Colors.GRID          # fonction réelle (pointillés)

N_POINTS = 11      # points d'entraînement
NOISE = 0.35       # écart-type du bruit sur les observations
DEG_BON = 3        # degré du modèle adapté
DEG_OVERFIT = 10   # degré du modèle trop flexible (= N_POINTS - 1)
# Points de test (jamais vus pendant l'ajustement) : aux bords et entre
# les points d'entraînement, là où un modèle trop flexible oscille.
X_TEST = (0.02, 0.095, 0.275, 0.455, 0.635, 0.815, 0.905, 0.98)
SEED = 3


def true_function(x):
    """Fonction que l'on cherche à apprendre."""
    return np.sin(2 * np.pi * x)


def make_data():
    """Jeux d'entraînement et de test : points bruités tirés de la
    fonction réelle. Seul le jeu d'entraînement sert à l'ajustement."""
    rng = np.random.default_rng(SEED)
    x_train = np.linspace(0.05, 0.95, N_POINTS)
    y_train = true_function(x_train) + rng.normal(0, NOISE, N_POINTS)

    x_test = np.array(X_TEST)
    y_test = true_function(x_test) + rng.normal(0, NOISE, len(x_test))
    return (x_train, y_train), (x_test, y_test)


def style_axes(ax):
    """Même traitement que styled_figure(), pour un axe d'une grille :
    fond transparent, axes fins avec halo, sans graduations."""
    ax.patch.set_alpha(0)
    for name, spine in ax.spines.items():
        if name in ("left", "bottom"):
            spine.set_color(Colors.AXIS)
            spine.set_linewidth(1.4)
            spine.set_path_effects(halo(linewidth=4))
        else:
            spine.set_visible(False)
    ax.set_xticks([])
    ax.set_yticks([])


def draw_panel(ax, train, test, degree, title, legend=False):
    """Ajuste un polynôme de degré `degree` sur le jeu d'entraînement,
    puis trace la courbe avec les points d'entraînement et de test."""
    style_axes(ax)
    x, y = train
    x_test, y_test = test

    xs = np.linspace(0, 1, 400)
    coeffs = np.polyfit(x, y, degree)

    ax.plot(xs, true_function(xs), color=CURVE_TRUE, linewidth=2.2,
            linestyle="--", label="fonction réelle", zorder=1)
    ax.plot(xs, np.polyval(coeffs, xs), color=CURVE_MODEL, linewidth=2.8,
            path_effects=halo(linewidth=6), label="modèle", zorder=2)
    ax.scatter(x, y, s=110, color=POINTS, edgecolor=Colors.ACCENT,
               linewidth=1.6, zorder=3, label="données d'entraînement")
    ax.scatter(x_test, y_test, s=110, marker="D", color=POINTS_TEST,
               edgecolor=Colors.ACCENT, linewidth=1.6, zorder=4,
               label="données de test")

    ax.set_xlim(-0.03, 1.03)
    ax.set_ylim(-1.7, 1.7)
    ax.set_xlabel("x", color=Colors.TEXT, fontsize=Fonts.SIZE_LABEL,
                  path_effects=halo())
    ax.set_ylabel("y", color=Colors.TEXT, fontsize=Fonts.SIZE_LABEL,
                  rotation=0, labelpad=12, path_effects=halo())
    ax.set_title(title, color=Colors.TEXT, fontsize=Fonts.SIZE_TITLE,
                 fontweight="bold", pad=14, path_effects=halo())

    if legend:
        leg = ax.legend(loc="lower left", frameon=False,
                        fontsize=Fonts.SIZE_TICK)
        for text in leg.get_texts():
            text.set_color(Colors.TEXT)
            text.set_path_effects(halo())


def generate(output_path):
    apply_global_style()
    fig, axes = plt.subplots(1, 2, figsize=(13, 5.5))
    fig.patch.set_alpha(0)

    train, test = make_data()
    draw_panel(axes[0], train, test, DEG_BON, "Bon ajustement", legend=True)
    draw_panel(axes[1], train, test, DEG_OVERFIT, "Sur-apprentissage")

    fig.subplots_adjust(wspace=0.15)
    return save(fig, output_path)


if __name__ == "__main__":
    default_out = Path(__file__).resolve().parent / "overfitting.png"
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")