---
title: Tmux
tags:
  - terminal
date: 2026-10-04
publish: true
---

# Configuration


dans `~/.tmux.conf` : 

```bash
# autorise le scroll à la sourie
set -g mouse on 

# raccourcis vim
setw -g mode-keys vi

# Pour qu'une sessions ne soit pas tuée si on tue le terminal 
set -g destroy-unattached off
set -g exit-unattached off
```
# Raccourcis
**Préfixe :** `Ctrl+b`
## Sessions

|Action|Raccourci / Commande|
|---|---|
|Nouvelle session|`tmux`|
|Nouvelle session nommée|`tmux new -s <name>`|
|Lister les sessions|`tmux ls`|
|Attacher la dernière session|`tmux a`|
|Attacher une session nommée|`tmux a -t <name>`|
|Détacher la session|`Prefix d`|
|Renommer la session|`Prefix $`|
|Liste interactive des sessions|`Prefix s`|
|Supprimer une session|`tmux kill-session -t <name>`|
|Supprimer toutes les sessions|`tmux kill-server`|

## Fenêtres (Windows)

|Action|Raccourci|
|---|---|
|Créer une fenêtre|`Prefix c`|
|Renommer une fenêtre|`Prefix ,`|
|Fenêtre suivante|`Prefix n`|
|Fenêtre précédente|`Prefix p`|
|Dernière fenêtre utilisée|`Prefix l`|
|Sélection par numéro|`Prefix 0-9`|
|Liste des fenêtres|`Prefix w`|
|Rechercher une fenêtre|`Prefix f`|
|Fermer une fenêtre|`Prefix &`|
|Déplacer une fenêtre|`Prefix .`|

## Panneaux (Panes)

|Action|Raccourci|
|---|---|
|Split horizontal|`Prefix %`|
|Split vertical|`Prefix "`|
|Panneau suivant|`Prefix o`|
|Navigation entre panneaux|`Prefix ← ↑ ↓ →`|
|Afficher les numéros|`Prefix q`|
|Zoom / Dézoom|`Prefix z`|
|Fermer un panneau|`Prefix x`|
|Échanger vers l'avant|`Prefix }`|
|Échanger vers l'arrière|`Prefix {`|
|Convertir en fenêtre|`Prefix !`|
|Redimensionner|`Prefix Ctrl+← ↑ ↓ →`|
|Changer de layout|`Prefix Space`|

## Mode Copie (Copy Mode)

|Action|Raccourci|
|---|---|
|Entrer en mode copie|`Prefix [`|
|Défilement haut / bas|`PgUp / PgDn`|
|Coller le buffer|`Prefix ]`|
|Lister les buffers|`Prefix =`|
|Navigation (mode vi)|`h`, `j`, `k`, `l`|
|Début / fin de ligne|`0` / `$`|
|Recherche avant|`/`|
|Recherche arrière|`?`|
|Début de sélection|`v` (vi) / `Space`|
|Copier la sélection|`y` / `Enter`|
|Quitter|`q` / `Esc`|

## Mode Commande

|Action|Raccourci|
|---|---|
|Ouvrir l'invite de commande|`Prefix :`|
|Afficher les raccourcis|`Prefix ?`|
|Recharger la configuration|`Prefix : source-file ~/.tmux.conf`|

## Divers

|Action|Raccourci / Commande|
|---|---|
|Mode horloge|`Prefix t`|
|Détacher les autres clients|`Prefix D`|
|Afficher la version|`tmux -V`|
|Informations serveur|`tmux info`|