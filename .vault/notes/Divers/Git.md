---
title: Git
tags:
  - code
  - git
date: 2026-09-29
publish: true
---
# Git sibmodules

Permet de gérer un git dans un git, un peu comme un équivalent d'un repo meta. Pratique pour des applications multi composants où chaque composant a son propre repo git.

```bash
# Ajouter un submodule
git submodule add <url-du-depot> <chemin>

# Cloner un dépôt avec ses submodules
git clone --recurse-submodules <url-du-depot>

# Si le dépot est déjà cloné 
git submodule update --init --recursive

# Update
git submodule update --recursive # ça point vers le commit enregistrer dans le dépot source 

```

# Rebase
`git rebase` rejoue les commits d'une branche sur une nouvelle base afin de conserver un historique linéaire.
### Exemple

```text
main
A---B---C---D

feature
     \
      E---F
```

```bash
git checkout feature
git rebase main
```

Résultat :
```text
main
A---B---C---D

feature
             \
              E'---F'
```

## Rebase --onto

`git rebase --onto <nouvelle-base> <ancienne-base> <branche>`
Rejoue les commits de `<branche>` situés après `<ancienne-base>` sur `<nouvelle-base>`.

### Exemple

```text
A---B---C---D---E---F---G
            ^       ^
         feature1 feature2
```

```bash
git rebase --onto main feature1 feature2
```

Les commits `F` et `G` sont déplacés sur `main` :
```text
main
A---B---C

feature1
         \
          D---E

feature2
         \
          F'---G'
```

### Cas pratique

Supprimer les commits `C` et `D` :
```text
A---B---C---D---E---F
```

```bash
git rebase --onto B D
```

Résultat :
```text
A---B---E'---F'
```