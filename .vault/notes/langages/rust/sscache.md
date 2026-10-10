---
title: sscache
tags:
  - code
  - rust
date: 2026-10-10
publish: true
---
```bash
# Installer le cache de compilation (une seule fois) 
cargo install sccache --locked 

# Vérifier qu'il sert bien (taux de hits) 
sccache --show-stats
```