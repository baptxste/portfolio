---
title: Linux terminal sensible à la case
tags:
  - linux
  - zsh
date: 2026-10-28
publish: true
---
Pour avoir pareil que sur windows et mac au niveau des cd qui bloquent pas avec les majuscules on peut utiliser **zsh avec completion insensible à la casse** :

### Activer :
```
autoload -Uz compinitcompinit
```

Puis ajoute dans `~/.zshrc` :
```
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
```

Résultat :
- `cd proje` → trouve `Project`
- ignore les majuscules/minuscules