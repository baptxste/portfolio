---
title: Fonctions d'activation
tags:
  - todo
date: 2026-09-21
publish: true
---
|                  |                                                       |                   |                                                              |
| ---------------- | ----------------------------------------------------- | ----------------- | ------------------------------------------------------------ |
| **Sigmoïde**     | σ(z) = 1 / (1 + e⁻ᶻ)                                  | ]0, 1[            | Sortie binaire (probabilité) ; historique en couches cachées |
| **Tanh**         | tanh(z) = (eᶻ − e⁻ᶻ) / (eᶻ + e⁻ᶻ)                     | ]−1, 1[           | Couches cachées (anciennes), réseaux récurrents              |
| **ReLU**         | max(0, z)                                             | [0, +∞[           | Défaut des couches cachées                                   |
| **Leaky ReLU**   | max(αz, z), α ≈ 0,01                                  | ℝ                 | Variante de ReLU contre les neurones « morts »               |
| **ELU**          | z si z > 0, sinon α(eᶻ − 1)                           | ]−α, +∞[          | Variante lisse de ReLU                                       |
| **GELU**         | z·Φ(z), Φ = fonction de répartition de la loi normale | ≈ ]−0,17, +∞[     | Transformers (BERT, GPT)                                     |
| **SiLU / Swish** | z·σ(z)                                                | ≈ ]−0,28, +∞[     | Réseaux modernes (EfficientNet, LLM)                         |
| **Softmax**      | softmax(z)ᵢ = eᶻⁱ / Σⱼ eᶻʲ                            | ]0, 1[, somme = 1 | Sortie multiclasse                                           |
| **Identité**     | z                                                     | ℝ                 | Sortie en régression                                         |

Une fonction d'activation doit etre non linéraire (sinon pas d'intéret ), différentiable, de préférence monotone (la monotonie garantie la convexité de l'erreur ) et surtout doit etre facile à calculer et a différencier ( efficcité du reseau)