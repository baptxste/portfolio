---
title: Tmux
tags:
  - terminal
date: 2026-10-13
publish: true
---

# Configuration

dans `~/.tmux.conf` :

```bash
# autorise le scroll à la souris
set -g mouse on

# raccourcis vim
setw -g mode-keys vi
bind -T copy-mode-vi v send -X begin-selection   # v pour démarrer la sélection
bind -T copy-mode-vi y send -X copy-selection-and-cancel   # y pour copier

# Pour qu'une session ne soit pas tuée si on tue le terminal
set -g destroy-unattached off
set -g exit-unattached off

# --- Plugins (TPM) : survivre aux redémarrages ---
set -g @plugin 'tmux-plugins/tpm'
set -g @plugin 'tmux-plugins/tmux-resurrect'
set -g @plugin 'tmux-plugins/tmux-continuum'

set -g @continuum-restore 'on'

# Toujours en toute dernière ligne
run '~/.tmux/plugins/tpm/tpm'
```

## Installation des plugins (survie aux reboots)

Les sessions tmux survivent à la fermeture du terminal, mais **pas à un redémarrage** de la machine. Pour ça, on restaure la configuration et les commandes (pas l'état exact des processus) :

- **tmux-resurrect** : sauvegarde / restaure sessions, fenêtres, panes et commandes
- **tmux-continuum** : automatise les sauvegardes (toutes les 15 min par défaut) et la restauration au démarrage de tmux

1. Installer TPM (le gestionnaire de plugins) :

```bash
git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
```

2. Ajouter le bloc plugins ci-dessus dans `~/.tmux.conf`
3. Recharger la config :

```bash
tmux source-file ~/.tmux.conf
```

4. Dans tmux, installer les plugins : `Prefix I` (i majuscule)

|Action|Raccourci|
|---|---|
|Installer les plugins|`Prefix I`|
|Mettre à jour les plugins|`Prefix U`|
|Sauvegarder manuellement (resurrect)|`Prefix Ctrl+s`|
|Restaurer manuellement (resurrect)|`Prefix Ctrl+r`|

> [!note] Un serveur, un `vim` ou une commande longue ne reprend pas toujours exactement au même point après un reboot, mais `tmux-resurrect` recrée les panes et relance généralement les commandes. La restauration automatique se déclenche au démarrage du serveur tmux (première commande `tmux` après le reboot).

# Raccourcis

**Préfixe :** `Ctrl+b`

## Sessions

|Action|Raccourci / Commande|
|---|---|
|Nouvelle session|`tmux`|
|Nouvelle session nommée|`tmux new -s <name>`|
|Lister les sessions|`tmux ls`|
|Attacher la dernière session|`tmux a`|
|Attacher une session nommée|`tmux a -t <name>` (ou `tmux attach -t <name>`)|
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
|Navigation (mode vi)|`h`, `j`, `k`, `l`|
|Début / fin de ligne|`0` / `$`|
|Recherche avant|`/`|
|Recherche arrière|`?`|
|Début de sélection|`Space` (ou `v` avec le bind de la config)|
|Copier la sélection|`Enter` (ou `y` avec le bind de la config)|
|Quitter|`q` / `Esc`|

## Mode Commande

|Action|Raccourci|
|---|---|
|Ouvrir l'invite de commande|`Prefix :`|
|Afficher les raccourcis|`Prefix ?`|
|Recharger la configuration|`Prefix : source-file ~/.tmux.conf` ou `tmux source-file ~/.tmux.conf`|

## Divers

| Action                      | Raccourci / Commande |
| --------------------------- | -------------------- |
| Mode horloge                | `Prefix t`           |
| Détacher les autres clients | `Prefix D`           |
| Afficher la version         | `tmux -V`            |
| Informations serveur        | `tmux info`          |