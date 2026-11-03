---
title: Ancres Compose
tags:
  - docker
  - compose
  - yaml
  - anchor
date: 2026-11-03
publish: true
---
# Ancres YAML et champs `x-` dans Docker Compose

## Objectif

Éviter de dupliquer la configuration commune de plusieurs services (ici, deux modes du même front Angular + BFF sur `:4200`) en la définissant une seule fois et en la réutilisant.

## Les ingrédients

|Élément|Syntaxe|Origine|Rôle|
|---|---|---|---|
|Champ d'extension|`x-nom:`|Compose Specification|Clé de premier niveau préfixée par `x-` : ignorée par Compose (ni service, ni réseau, ni volume). Sert d'espace de stockage pour des blocs réutilisables.|
|Ancre|`&nom`|YAML|Donne un nom au bloc YAML qui suit.|
|Alias|`*nom`|YAML|Réutilise le bloc ancré.|
|Fusion (_merge key_)|`<<: *nom`|YAML 1.1|Insère le contenu de l'alias dans le mapping courant, puis on peut écraser ou ajouter des clés.|

## Norme ou convention ?

- **`&`, `*` : norme YAML.** Ancres et alias font partie de la spécification YAML elle-même (yaml.org), donc ils marchent dans tout fichier YAML (GitHub Actions, GitLab CI, Kubernetes...), pas seulement dans Docker.
- **`<<` : norme YAML 1.1**, absente du cœur de YAML 1.2, mais gérée par presque tous les parseurs, dont celui de Compose.
- **`x-` : convention de la Compose Specification** (compose-spec.io, ouverte, utilisée par Docker Compose). Un outil qui lit le fichier doit ignorer ces clés, ce qui évite une erreur de validation sur une clé inconnue. Ce n'est pas du YAML mais le même principe existe ailleurs (par exemple `x-...` dans OpenAPI).
- Les deux sont **indépendants** : on peut ancrer un bloc sans `x-`, mais à la racine du fichier le préfixe `x-` est nécessaire, sinon Compose rejette la clé comme inconnue. La combinaison donne le motif : `x-front` stocke le bloc sans le déclarer comme service, `&front` permet de le réutiliser.

## Exemple

```yaml
x-front: &front
  build:
    context: ./prometeo-front
    dockerfile: .docker/Dockerfile-dev-front
  restart: unless-stopped
  working_dir: /var/www
  command: sh -c "npm i --prefix ./bff && npm i --prefix ./ui && ./.docker/start-dev.sh"
  env_file:
    - prometeo-front/.env.local
    - prometeo-front/.env.secret.local
  volumes:
    - ./prometeo-front:/var/www
  ports:
    - "4200:4200"
    - "9222:9222"
    - "9229:9229"
  networks:
    - prometeo_shared_net

services:
  front:                      # front branché sur le back de dev
    <<: *front
    environment:
      API_URLS: https://back-dev.example.com

  front-local-back:           # front branché sur un back local
    <<: *front
    profiles: ["local-back"]
    environment:
      API_URLS: http://backend:8000
```

Chaque service hérite de tout le bloc `x-front` et ne redéfinit que ce qui change (ici `API_URLS`).

## Points d'attention

- **La fusion est superficielle** : redéfinir une clé de l'ancre (`environment`, `ports`, `volumes`...) remplace sa valeur en entier, sans fusion des sous-clés.
- **L'ancre doit être définie avant son alias**, dans le même fichier.
- Les deux services publient les mêmes ports, ils ne peuvent pas tourner ensemble. Les `profiles` permettent d'activer l'un ou l'autre : `docker compose --profile local-back up`.
- Vérifier le résultat de la fusion avec `docker compose config`.

## Alternatives

- **`extends`** : un service hérite d'un autre, éventuellement d'un autre fichier. Plus explicite, fusion plus fine, plus verbeux.
- **`include`** ou plusieurs fichiers (`-f` répété) : découper la configuration en fichiers.

## Quand l'utiliser

Peu de variantes, dans un seul fichier, avec une base largement commune : les ancres restent le choix le plus simple.