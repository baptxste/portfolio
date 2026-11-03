---
title: Override compose
tags:
  - docker
  - compose
  - override
date: 2026-11-03
publish: true
---
# Override dans Docker Compose

## Principe

Compose peut charger **plusieurs fichiers** et les fusionner : le premier sert de base, les suivants viennent **par-dessus** et modifient ou complètent ce qui existe. Les services sont repérés par leur **nom**.

Intérêt : garder un fichier de base commun à tous, et isoler dans d'autres fichiers ce qui dépend de l'environnement (dev, prod, CI) ou de la machine de chacun.

## Deux façons de l'utiliser

### 1. Le fichier d'override implicite

Sans option, `docker compose up` lit automatiquement :

1. `compose.yaml` (ou `compose.yml`, `docker-compose.yml`) : la base
2. `compose.override.yaml` (ou `compose.override.yml`, `docker-compose.override.yml`) : appliqué par-dessus, s'il existe

C'est pratique pour des réglages **locaux**, souvent non versionnés (`compose.override.yaml` dans le `.gitignore`).

### 2. Les fichiers explicites avec `-f`

```bash
docker compose -f compose.yml -f compose.prod.yml up -d
```

- **L'ordre compte** : un fichier écrase ce qui vient avant lui.
- Dès qu'on passe `-f`, le fichier `compose.override.yaml` **n'est plus chargé automatiquement** : il faut l'ajouter à la main si on le veut.
- Équivalent par variable d'environnement : `COMPOSE_FILE=compose.yml:compose.prod.yml` (séparateur `:` sous Linux/macOS, `;` sous Windows).

## Exemple

```yaml
# compose.yml  (base)
services:
  front:
    build: ./front
    command: npm run dev
    environment:
      LOG_LEVEL: debug
      API_URL: http://api-dev:8000
    ports:
      - "4200:4200"
    volumes:
      - ./front:/app
```

```yaml
# compose.prod.yml  (par-dessus)
services:
  front:
    command: npm run start
    environment:
      LOG_LEVEL: warn
    ports:
      - "80:4200"
    volumes: []
```

Résultat fusionné pour `front` :

|Clé|Résultat|Pourquoi|
|---|---|---|
|`command`|`npm run start`|remplacée|
|`environment.LOG_LEVEL`|`warn`|la variable est écrasée|
|`environment.API_URL`|`http://api-dev:8000`|conservée (fusion clé par clé)|
|`ports`|`4200:4200` **et** `80:4200`|liste : les valeurs s'ajoutent|
|`volumes`|`./front:/app` conservé|`volumes: []` n'efface rien|

Le dernier point est un piège classique : pour **retirer** une valeur, un override vide ne suffit pas, voir plus bas.

## Règles de fusion

|Type de clé|Comportement|
|---|---|
|Valeur simple (`image`, `restart`, ...)|Remplacée|
|Mapping (`environment`, `labels`, `build`, ...)|Fusionné clé par clé|
|Liste (`ports`, `expose`, `dns`, ...)|Les valeurs de l'override sont **ajoutées**|
|`volumes`|Fusionnés selon le point de montage dans le conteneur|
|`command`, `entrypoint`|**Remplacés** en entier|

On peut aussi **ajouter un service** qui n'existe que dans l'override (par exemple un outil de debug).

## Retirer ou remplacer plutôt que fusionner

Compose récent (v2.24 et suivants) propose deux tags YAML :

```yaml
services:
  front:
    ports: !override
      - "80:4200"        # remplace la liste au lieu de s'y ajouter
    volumes: !reset []   # supprime complètement la valeur de la base
```

## Vérifier le résultat

```bash
docker compose -f compose.yml -f compose.prod.yml config
```

Affiche le fichier final après fusion, avec les variables substituées. C'est le réflexe à avoir avant de lancer.

## Pièges

- **Chemins relatifs** : ils sont interprétés par rapport au dossier du **premier** fichier (le dossier du projet), pas à celui de l'override.
- **Ordre des `-f`** : inverser deux fichiers change le résultat.
- **Listes qui s'accumulent** : deux fichiers qui publient chacun un port s'additionnent, ce qui peut créer un conflit de ports.
- **Override local oublié** : un `compose.override.yaml` non versionné peut expliquer qu'un projet marche différemment sur deux machines.

## Cas d'usage typiques

- Base commune + `compose.dev.yml` / `compose.prod.yml`
- Réglages personnels (ports, volumes de debug) dans un `compose.override.yaml` ignoré par git
- Surcharge pour la CI (images pré-construites, pas de volumes de code)

## Alternatives

- **`extends`** : un service hérite d'un autre, éventuellement d'un autre fichier.
- **`include`** : inclure un autre fichier Compose entier dans le projet.
- **Profiles** : définir tous les services dans un même fichier et n'activer que ceux voulus (`--profile`).