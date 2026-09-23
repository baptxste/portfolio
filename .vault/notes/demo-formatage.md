---
title: Note Démo Formatage & Math
tags:
  - demo
  - latex
  - formatage
date: 2026-09-21
publish: true
---

# Démo complète de formatage Obsidian

Voici une note démontrant la prise en charge enrichie du Markdown, des images, des formules mathématiques et des callouts.

## 1. Formules Mathématiques (LaTeX)

Équation en ligne : la célèbre relation $E = mc^2$ d'Einstein.

Équation en bloc :
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

## 2. Callouts Obsidian

> [!NOTE] Remarque importante
> Les callouts Obsidian sont automatiquement stylisés avec une bordure et un fond adapté !

> [!WARNING] Attention
> Pensez à bien vérifier vos liens externes.

## 3. Liens externes & Wikilinks

- Lien interne vers [[bienvenue|la note de bienvenue]]
- Lien externe vers [le site officiel Rust](https://www.rust-lang.org) (s'ouvre automatiquement dans un nouvel onglet avec l'icône ↗)

## 4. Images & Fichiers joints

Voici une image de démonstration :

![[image_demo.png]]

## 5. Tableaux & Listes de tâches

| Fonctionnalité | Supporté | Note |
|---|---|---|
| LaTeX KaTeX | ✅ Oui | $f(x) = x^2$ |
| Callouts | ✅ Oui | Note, Warning, Info |
| Images local | ✅ Oui | Dossier `attachments/` |

- [x] Implémenter le parsing Markdown complet
- [x] Supporter LaTeX avec KaTeX
- [x] Gérer les images et callouts
