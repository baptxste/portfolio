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

Fixer la version exact des dépendances: 
```toml
[tool.uv] 
add-bounds = "exact"
```