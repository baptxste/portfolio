---
title: Cargo
tags:
  - cargo
  - rust
  - code
date: 2026-09-21
publish: true
---
# Update

Mettre à jour le cargo.lock :  (modifie uniquement les contraintes)
```bash
cargo update                       # toutes les dépendances
cargo update -p tokio              # une seule
cargo update -p tokio --precise 1.40.0   # une version précise
cargo update --dry-run             # voir ce qui changerait, sans rien écrire
```

Mettre à jour les version : 
```bash
cargo install cargo-edit
cargo upgrade                      # versions compatibles (semver)
cargo upgrade --incompatible       # inclut les changements majeurs
```

auditer les dépendances:
```bash
cargo install cargo-outdated
cargo outdated
```