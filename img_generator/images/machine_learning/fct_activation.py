"""Trace ReLU, sigmoïde et tanh côte à côte, dans la charte graphique du site.

Usage :
    python fct_activation.py
"""
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from styles import (Colors, Fonts, apply_global_style, french_number, halo,
                    save)

# Une teinte de la palette par fonction
CURVE_RELU = Colors.BAND_1
CURVE_SIGMOID = Colors.BAND_2
CURVE_TANH = Colors.BAND_3

X_MAX = 5          # les courbes sont tracées sur [-X_MAX, X_MAX]


def relu(z):
    """max(0, z)."""
    return np.maximum(0, z)


def sigmoid(z):
    """1 / (1 + e^-z) : sortie dans ]0, 1[."""
    return 1 / (1 + np.exp(-z))


def tanh(z):
    """Tangente hyperbolique : sortie dans ]-1, 1[."""
    return np.tanh(z)


def style_axes(ax):
    """Même traitement que styled_figure(), mais pour un axe d'une grille :
    fond transparent, axes fins avec halo, croisés à l'origine."""
    ax.patch.set_alpha(0)
    for name, spine in ax.spines.items():
        if name in ("left", "bottom"):
            spine.set_position("zero")
            spine.set_color(Colors.AXIS)
            spine.set_linewidth(1.4)
            spine.set_path_effects(halo(linewidth=4))
        else:
            spine.set_visible(False)
    ax.tick_params(colors=Colors.AXIS, labelsize=Fonts.SIZE_TICK)


def draw_panel(ax, f, color, title, yticks, ylim):
    """Dessine une fonction d'activation dans un axe."""
    style_axes(ax)

    z = np.linspace(-X_MAX, X_MAX, 400)
    ax.plot(z, f(z), color=color, linewidth=2.8,
            path_effects=halo(linewidth=6))

    ax.set_xlim(-X_MAX, X_MAX)
    ax.set_ylim(*ylim)
    ax.set_xticks([-4, -2, 2, 4])
    ax.set_yticks(yticks)
    # Vrai signe moins (U+2212), comme sur l'axe x
    ax.set_yticklabels([french_number(t, 0 if t == int(t) else 1)
                        .replace("-", "−") for t in yticks],
                       fontsize=Fonts.SIZE_TICK)
    ax.set_title(title, color=Colors.TEXT, fontsize=Fonts.SIZE_TITLE,
                 fontweight="bold", pad=14, path_effects=halo())

    # Halo sur les graduations (les labels n'existent qu'après set_ticks)
    for label in ax.get_xticklabels() + ax.get_yticklabels():
        label.set_path_effects(halo())


def generate(output_path):
    apply_global_style()
    fig, axes = plt.subplots(1, 3, figsize=(16, 5.5))
    fig.patch.set_alpha(0)

    draw_panel(axes[0], relu, CURVE_RELU, "ReLU",
               yticks=[1, 2, 3, 4, 5], ylim=(-0.6, 5.3))
    draw_panel(axes[1], sigmoid, CURVE_SIGMOID, "Sigmoïde",
               yticks=[0.5, 1], ylim=(-0.1, 1.1))
    draw_panel(axes[2], tanh, CURVE_TANH, "Tanh",
               yticks=[-1, 1], ylim=(-1.2, 1.2))

    fig.subplots_adjust(wspace=0.25)
    return save(fig, output_path)


if __name__ == "__main__":
    default_out = Path(__file__).resolve().parent / "activations.png"
    default_out.parent.mkdir(parents=True, exist_ok=True)
    path = generate(default_out)
    print(f"Image générée : {path}")