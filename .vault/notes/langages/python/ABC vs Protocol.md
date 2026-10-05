---
title: ABC vs Protocol
tags:
  - python
  - code
date: 2026-10-04
publish: true
---
# ABC ou Protocol en Python : quelle différence ?

_Deux outils pour définir une interface, deux philosophies opposées._

Python propose deux manières de décrire « ce qu'une classe doit savoir faire » : les classes abstraites (`ABC`) et les protocoles (`Protocol`). Les deux servent le même objectif, mais l'une repose sur l'**héritage** (typage nominal), l'autre sur la **forme** des objets (typage structurel).

---

## ABC : le contrat par héritage

Avec une ABC, une classe doit **hériter explicitement** de la classe abstraite pour en faire partie.

```python
from abc import ABC, abstractmethod

class Forme(ABC):
    @abstractmethod
    def aire(self) -> float: ...

class Cercle(Forme):          # héritage obligatoire
    def __init__(self, r): self.r = r
    def aire(self): return 3.14159 * self.r ** 2

Forme()   # TypeError : impossible d'instancier une classe abstraite
```

**Ce qu'il faut retenir**

- La vérification se fait **à l'exécution** : une sous-classe qui n'implémente pas toutes les méthodes abstraites ne peut pas être instanciée.
- Une ABC peut embarquer du **code partagé** (méthodes concrètes, attributs, `__init__`).
- `isinstance(obj, Forme)` fonctionne naturellement.
- Une classe tierce peut être rattachée après coup avec `Forme.register(MaClasse)`.

---

## Protocol : le duck typing, version statique

Avec un Protocol, une classe respecte le contrat **dès lors qu'elle possède les bonnes méthodes**, sans rien déclarer.

```python
from typing import Protocol

class Forme(Protocol):
    def aire(self) -> float: ...

class Carre:                   # aucun héritage
    def __init__(self, c): self.c = c
    def aire(self): return self.c ** 2

def afficher(f: Forme) -> None:
    print(f.aire())

afficher(Carre(3))   # accepté par mypy / pyright
```

**Ce qu'il faut retenir**

- La vérification est faite par les **outils de typage statique** (mypy, pyright), pas à l'exécution.
- Le Protocol est particulièrement adapté pour typer du code que l'on ne maîtrise pas, comme les classes de bibliothèques tierces, sans toucher à leur hiérarchie.
- Pour utiliser `isinstance()`, il faut ajouter `@runtime_checkable`. Attention : seule la **présence** des méthodes est vérifiée, pas leurs signatures.

---

## Comparaison rapide

||ABC|Protocol|
|---|---|---|
|Relation|Héritage explicite|Compatibilité de structure|
|Vérification|À l'exécution (instanciation)|Statique (mypy / pyright)|
|Code partagé|Oui|Non (interface seulement)|
|Classes tierces|Doivent hériter (ou `register`)|Fonctionne directement|
|Couplage|Plus fort|Plus faible|

---

## Lequel choisir ?

- **ABC** : lorsque la hiérarchie de classes est maîtrisée, que le contrat doit être imposé à l'exécution, ou qu'une implémentation de base commune est utile (pattern _template method_, par exemple).
- **Protocol** : lorsqu'il s'agit simplement de décrire ce qu'un objet doit savoir faire pour typer des paramètres, surtout si les types proviennent de sources différentes.

Les deux approches se combinent : une classe peut hériter explicitement d'un Protocol pour profiter de ses éventuelles implémentations par défaut, tout en documentant son intention.

---

## Et pour reproduire un « trait » ?

Les développeurs venant de Rust, Scala ou PHP se demandent souvent quel est l'équivalent Python d'un trait. La réponse dépend de ce que l'on entend par là.

**Un trait « interface avec méthodes par défaut » (Rust, Scala)** L'ABC est le candidat le plus naturel : elle déclare un contrat, fournit des méthodes concrètes réutilisables et vérifie le tout à l'exécution. Un Protocol auquel on ajoute des méthodes par défaut, puis dont les classes héritent explicitement, donne un résultat proche, avec en prime la vérification statique.

**Un trait « mixin » (PHP, Scala)** Si l'objectif est d'injecter du comportement réutilisable dans plusieurs classes, la solution idiomatique en Python est un **mixin** : une simple classe, sans ABC, combinée par héritage multiple.

```python
class JsonMixin:
    def to_json(self) -> str:
        import json
        return json.dumps(self.__dict__)

class Utilisateur(JsonMixin):
    def __init__(self, nom): self.nom = nom
```

**Un trait Rust « implémentable sur un type existant »** Rust permet d'implémenter un trait pour un type qu'on n'a pas écrit. Python n'a pas d'équivalent exact, mais le Protocol s'en rapproche le plus : tout type externe qui a les bonnes méthodes est accepté, sans modification.

|Besoin|Outil Python|
|---|---|
|Contrat + méthodes par défaut + contrôle à l'exécution|`ABC`|
|Contrat léger, vérifié statiquement, compatible avec des types tiers|`Protocol`|
|Injecter du comportement réutilisable|Mixin (héritage multiple)|

---

## Vérifier qu'une classe respecte bien son contrat

Une question revient souvent, surtout chez les développeurs habitués à Rust : avec un Protocol, comment s'assurer qu'une classe implémente bien toutes les méthodes requises ? Avec une ABC, l'erreur est immédiate à l'instanciation. Avec un Protocol, la vérification existe, mais elle est assurée par **l'outil de typage** (mypy, pyright), et non par Python lui-même.

### Ce que détecte mypy

```python
class Forme(Protocol):
    def aire(self) -> float: ...

class Carre:                  # oubli de aire()
    pass

def afficher(f: Forme) -> None: ...

afficher(Carre())   # mypy : Argument 1 has incompatible type "Carre"; expected "Forme"
```

À l'exécution, Python ne signale rien. Sans passage de mypy, l'erreur n'apparaît qu'à l'appel de `f.aire()`, sous la forme d'un `AttributeError`.

### Retrouver le comportement d'un `impl Trait for Type`

En Rust, l'intention est déclarée (`impl Trait for Type`) et le compilateur vérifie la conformité. Deux approches s'en rapprochent en Python.

**1. Hériter explicitement du Protocol, avec `@abstractmethod`**

```python
from typing import Protocol
from abc import abstractmethod

class Forme(Protocol):
    @abstractmethod
    def aire(self) -> float: ...

class Carre(Forme):           # déclaration explicite
    pass

Carre()   # TypeError à l'exécution + erreur mypy dès la définition
```

Le comportement est alors identique à celui d'une ABC (erreur à l'instanciation), tout en conservant la compatibilité structurelle pour les classes qui n'héritent pas du protocole.

**2. Vérifier la conformité sans héritage**

```python
_check: Forme = Carre()   # mypy signale si Carre ne respecte pas Forme
```

Cette assignation typée, placée dans un test ou à la fin d'un module, valide qu'une classe respecte un protocole sans en hériter.

### Bilan

|Besoin|Solution|
|---|---|
|Erreur à l'instanciation si une méthode manque|ABC, ou Protocol + héritage explicite + `@abstractmethod`|
|Erreur dès l'écriture du code (éditeur, CI)|mypy / pyright, avec les deux approches|
|Aucune déclaration, vérification uniquement à l'usage|Protocol « pur »|

Un Protocol pur ne garantit donc rien à l'exécution, mais l'héritage explicite d'un Protocol comble l'écart. C'est la combinaison la plus proche d'un `impl Trait for Type` de Rust.