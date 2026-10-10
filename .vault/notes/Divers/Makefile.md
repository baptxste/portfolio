---
title: Makefile
tags:
  - code
date: 2026-10-10
publish: true
---

# Structure : 

```makefile
.DEFAULT_GOAL := help
.PHONY: help build test lint clean

help: ## Affiche l'aide
	@awk ' \
		/^##/ { \
			sub(/^## ?/, ""); \
			print; \
			next; \
		} \
		/^[a-zA-Z0-9_-]+:/ { \
			name = $$0; \
			sub(/:.*/, "", name); \
			if (name == "help") next; \
			desc = ""; \
			if (match($$0, /##.*/)) { \
				desc = substr($$0, RSTART + 2); \
				sub(/^ */, "", desc); \
			} \
			printf " \033[36m%-15s\033[0m %s\n", name, desc; \
		} \
	' $(MAKEFILE_LIST)

## TITLE OF THE BUILD SECTION

build: ## Comment of the command
	cargo build

....
```

# Phony 

Le fichier doit commencer par : 
```makefile
.PHONY: help build test lint clean
```
où help, build ... sont des commandes du makefile, si cette ligne n'est pas présente le makefile fonctionne correctement tant qu'aucun fichier dans le dépôt ne s'appelle pareil, si cela ce produit et que phony n'est renseigné alors le makefile essaye de lancer le fichier.