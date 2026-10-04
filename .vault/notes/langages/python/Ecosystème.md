---
title: Ecosystème
tags:
  - python
  - code
date: 2026-09-21
publish: true
---
# Outils de dev
- [UV](https://docs.astral.sh/uv/) gestionnaire de dépendances largement supérieur à pip, similaire à poetry mais plus bien plus rapide. 
- [Ty](https://docs.astral.sh/ty/) Type checker pour python qui permet une gestion du typage un peu similaire à Rust



## UV 
Par défaut uv utilise la dernière version compatible des dépendances ce qui peut causer des erreurs entre différents environnements et laisse un flou sur quelle version d'une dépendance est utilisée, sur un projet déployé/ partagé il est préférable de fixer la version exact des dépendances avec : 
```toml
[tool.uv] 
add-bounds = "exact"
```

Pour vérifier les mise à jour des dépendances : `uv tree --outdated --depth 1`