---
title: Compose
tags:
  - compose
  - docker
  - todo
date: 2026-11-03
publish: true
---

# Composition avec Include

`include` permet d'assembler plusieurs projets Compose dans un compose principal : leurs services, réseaux et volumes deviennent ceux du projet. Chaque fichier inclus reste un compose complet, qui continue de fonctionner seul dans son propre repo.

Cas typique : un repo `main` qui contient (ou référence) un repo `back` et un repo `front`, chacun avec son compose.

```
main/
├── compose.yaml
├── back/compose.yaml
└── front/compose.yaml
```

```yaml
# main/compose.yaml
include:
  - path: ./back/compose.yaml
  - path: ./front/compose.yaml
    env_file: ./front/.env      # optionnel : variables d'interpolation du fichier inclus

services:
  proxy:                        # services propres à main
    image: caddy:2
    ports:
      - "80:80"
```

Syntaxe courte : `include: [./back/compose.yaml, ./front/compose.yaml]`. Syntaxe longue :

|Clé|Rôle|
|---|---|
|`path`|Un fichier, ou une **liste** de fichiers fusionnés dans l'ordre (le dernier gagne)|
|`project_directory`|Dossier de référence pour les chemins relatifs (par défaut : le dossier du fichier inclus)|
|`env_file`|Fichier d'environnement utilisé pour l'interpolation des variables du fichier inclus|

**Surcharger un fichier inclus.** `include` n'écrase rien : redéfinir dans `main` un service déjà présent dans un fichier inclus provoque une **erreur de conflit**. Pour le modifier, on passe une liste à `path` (le premier est l'original, les suivants sont des overrides, mêmes règles de fusion que cf Override compose) :

```yaml
include:
  - path:
      - ./front/compose.yaml
      - ./overrides/front.yaml
```

```yaml
# main/overrides/front.yaml
services:
  front:
    environment:
      API_URL: http://back:8080
```

**Ce que ça change**

|Point|Comportement|
|---|---|
|Projet|Tout tourne sous le **nom de projet de main** (le nom du projet inclus est ignoré)|
|Réseau|Réseau par défaut partagé : le front joint le back par son nom de service (`http://back:8080`)|
|Chemins relatifs|Résolus par rapport au dossier du **fichier inclus** (build context, volumes)|
|Profiles|Ceux des fichiers inclus restent actifs|
|Imbrication|Un fichier inclus peut lui-même utiliser `include`|

**Pièges**

- **Conflits de noms** : deux fichiers qui déclarent le même service, réseau ou volume provoquent une erreur.
- **Ports en double** : back et front qui publient le même port hôte ne démarrent pas ensemble.
- **`.env`** : sans `env_file`, l'interpolation d'un fichier inclus dépend de son `.env` (à vérifier avec `docker compose config`).
- **Différence avec `-f`** : avec `-f`, les chemins relatifs sont résolus à partir du **premier** fichier. Avec `include`, fichier par fichier.
- **Version** : en dessous de Compose 2.20, la clé `include` n'est pas reconnue.

Vérifier le résultat : `docker compose config`.

### Extends

`extends` permet à un service de reprendre la définition d'un autre service (du même fichier ou d'un autre) et d'en modifier des morceaux. Adapté à une variante ponctuelle d'un service, moins à l'assemblage de projets.

### Profiles

Les profiles permettent de définir tous les services dans un même fichier et de n'activer que ceux voulus (`--profile`).
## Dérivés

### Ancres

Une ancre permet de faire hériter plusieurs services d'une base commune. cf [[Ancres Compose]]

### Override

L'override permet de créer un fichier compose.override.yaml qui vient redéfinir des services ou morceaux de services dans un compose. cf [[Override compose]]



### Quel mécanisme choisir

|Besoin|Mécanisme|Portée|
|---|---|---|
|Factoriser une config commune entre quelques services d'un même fichier|Ancres (`x-` + `&` / `*` / `<<`)|Un fichier|
|Variantes par environnement ou réglages locaux d'une machine|Override (`compose.override.yaml`, `-f`)|Fichiers fusionnés|
|Assembler plusieurs repos ou projets, chacun avec son compose|`include`|Projet entier|
|Reprendre un seul service d'un autre fichier avec des modifications|`extends`|Un service|
|Activer ou désactiver des services selon le contexte|`profiles`|Services d'un même projet|

Ils se combinent : un main peut inclure back et front (`include`), les surcharger au passage (liste de fichiers dans `path`), et chaque projet peut utiliser des ancres en interne.