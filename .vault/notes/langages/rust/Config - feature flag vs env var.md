---
title: Config - feature flag vs env var
tags:
  - rust
  - code
  - configuration
date: 2026-09-21
publish: false
---

# Le problème

On a un trait et plusieurs implémentations :

```rust
pub trait Backend {
    fn call(&self, input: &str) -> String;
}

pub struct BackendA;
pub struct BackendB;
```

Question : **quand** décide-t-on quelle implémentation utiliser ?

- **À l'exécution** : lecture d'une variable d'environnement, d'un fichier de config, d'un argument CLI.
- **À la compilation** : choix via les _features_ Cargo.

---

# Approche A : choix à l'exécution (dispatch dynamique)

Le type concret n'est connu qu'au démarrage, donc on passe par un trait object.

```rust
use std::sync::Arc;

pub struct Service {
    backend: Arc<dyn Backend + Send + Sync>,
}

pub fn build_service() -> Service {
    let backend: Arc<dyn Backend + Send + Sync> =
        match std::env::var("BACKEND").as_deref() {
            Ok("a") => Arc::new(BackendA),
            Ok("b") => Arc::new(BackendB),
            _ => panic!("BACKEND invalide"),
        };
    Service { backend }
}
```

Si le service est générique (`Service<R>`), il faut relâcher la contrainte implicite `Sized` pour accepter `dyn Trait` :

```rust
pub struct Service<R: ?Sized + Backend> {
    backend: Arc<R>,
}
```

**Points clés**

- Appels de méthodes via une **vtable** (indirection, pas d'inlining possible).
- Un seul binaire, comportement modifiable sans recompiler.
- `?Sized` est nécessaire dès qu'on manipule `dyn Trait` dans un type générique.
- Les erreurs de configuration apparaissent au **runtime**.

> Les notions de `dyn Trait`, `Sized` et `?Sized` sont détaillées en **annexe** (section 10).

---

# Approche B : choix à la compilation (feature flags)

Cargo.toml :

```toml
[features]
backend-a = []
backend-b = []
```

Code :

```rust
#[cfg(feature = "backend-a")]
pub fn build_service() -> Service<BackendA> {
    Service::new(Arc::new(BackendA))
}

#[cfg(feature = "backend-b")]
pub fn build_service() -> Service<BackendB> {
    Service::new(Arc::new(BackendB))
}
```

Build :

```bash
cargo build --features backend-a
cargo test  --features backend-b
```

**Points clés**

- Le compilateur connaît le type concret : **monomorphisation**, appels directs, inlining possible, pas de vtable.
- `Sized` reste valable, pas besoin de `?Sized`.
- Le code des implémentations non sélectionnées peut être **exclu du binaire** (avec `#[cfg]` sur les modules et dépendances optionnelles).
- Changer d'implémentation demande une **recompilation**.
- Les erreurs de configuration apparaissent au **build**.

---

#  Comparatif

|Critère|Config runtime|Feature flag|
|---|---|---|
|Moment du choix|Démarrage du programme|Compilation|
|Dispatch|`dyn Trait` (vtable)|Statique (monomorphisation)|
|Surcoût à l'appel|Faible mais réel|Nul|
|Changer sans rebuild|Oui|Non|
|`?Sized` nécessaire|Oui|Non|
|Nombre de binaires|1|1 par configuration|
|Code mort dans le binaire|Oui (tout est compilé)|Non (si bien isolé)|
|Erreur de config détectée|À l'exécution|À la compilation|
|Risque d'activer le mauvais mode par erreur d'env|Présent|Réduit|

---

# Quand choisir quoi

**Feature flag** quand :

- les variantes correspondent à des **contextes de build distincts** (production / test / développement) ;
- on veut empêcher qu'une implémentation sensible ou factice (mock, bypass, version de debug) se retrouve en production par simple erreur de configuration ;
- on veut des binaires plus petits et sans indirection ;
- une variante tire des dépendances lourdes qu'on ne veut pas toujours embarquer.

**Config runtime** quand :

- le **même artefact** doit être promu d'un environnement à l'autre (staging → prod) ;
- le choix dépend de l'infrastructure ou du client, pas du build ;
- on veut pouvoir changer sans redéployer un nouveau binaire.

**Approche hybride** possible : feature flags pour exclure ce qui ne doit jamais exister en production, config runtime pour choisir parmi ce qui reste.

---

# Features par défaut

Cargo.toml :

```toml
[features]
default = ["backend-a"]
backend-a = []
backend-b = []
```

Effets :

```bash
cargo build                                              # backend-a (default)
cargo build --features backend-b                         # backend-b + backend-a !
cargo build --no-default-features --features backend-b   # backend-b seul
```

> **Piège classique** : `--features` **s'ajoute** aux features par défaut. Pour les remplacer, il faut passer `--no-default-features` en même temps.

Workflow typique :

```bash
# Build normal : utilise la valeur par défaut
cargo build --release

# Variante alternative (tests, dev)
cargo test --no-default-features --features backend-b
```

Bon réflexe : mettre en `default` l'option **sûre** (celle qu'on veut en production) et rendre les options risquées **explicitement opt-in**.

---

# Attention : features mutuellement exclusives

Cargo considère les features comme **additives** : si plusieurs crates du graphe de dépendances demandent des features différentes, Cargo les **unifie** (union). Deux features censées être exclusives peuvent donc se retrouver activées ensemble.

Conséquence avec l'exemple ci-dessus : si `backend-a` et `backend-b` sont actives en même temps, on obtient deux fonctions `build_service` identiques et une erreur de compilation peu lisible.

Garde-fou recommandé :

```rust
#[cfg(all(feature = "backend-a", feature = "backend-b"))]
compile_error!("Activer une seule feature parmi `backend-a` et `backend-b`.");

#[cfg(not(any(feature = "backend-a", feature = "backend-b")))]
compile_error!("Activer au moins une feature parmi `backend-a` et `backend-b`.");
```

Cela transforme une erreur obscure en message clair dès la compilation.

Alternative plus robuste si les features doivent coexister : exposer **les deux implémentations** et choisir à l'exécution, ou utiliser un type `enum` qui regroupe les variantes (dispatch statique sans trait object).

---

# Variante : enum dispatch

Quand les implémentations sont connues et en nombre fini, un `enum` permet un choix à l'exécution **sans** `dyn` :

```rust
pub enum AnyBackend {
    A(BackendA),
    B(BackendB),
}

impl Backend for AnyBackend {
    fn call(&self, input: &str) -> String {
        match self {
            AnyBackend::A(b) => b.call(input),
            AnyBackend::B(b) => b.call(input),
        }
    }
}
```

- Pas de vtable, pas de `?Sized`, pas d'allocation obligatoire.
- Tous les variants sont compilés dans le binaire.
- Ajouter une implémentation oblige à modifier l'enum (pas extensible depuis l'extérieur).

---

# Résumé

- **Runtime + `dyn`** : souple, un seul binaire, léger surcoût, nécessite `?Sized` dans les types génériques.
- **Compile-time + features** : zéro surcoût, binaires dédiés, plus sûr pour isoler du code sensible, rebuild nécessaire.
- **Toujours** : `--no-default-features` pour remplacer les defaults, `compile_error!` pour verrouiller les combinaisons invalides.
- **Option intermédiaire** : enum dispatch pour un choix runtime sans trait object.

---

# Annexe : comprendre `Sized`, `dyn Trait` et `?Sized`

## `Sized` : « la taille est connue à la compilation »

Un type est `Sized` quand le compilateur connaît son nombre d'octets au moment de compiler.

|Type|Taille|`Sized` ?|
|---|---|---|
|`u32`|4 octets|oui|
|`struct P { a: u64, b: u64 }`|16 octets|oui|
|`enum` simple (sans données)|1 octet|oui|
|`Vec<T>`|24 octets (pointeur + capacité + longueur)|oui|
|`Box<T>`, `Arc<T>`, `&T`|taille d'un pointeur|oui|
|`str`|inconnue (texte de longueur variable)|**non**|
|`[T]` (slice)|inconnue (nombre d'éléments variable)|**non**|
|`dyn Trait`|inconnue (type concret inconnu)|**non**|

Remarque : `Vec<T>` est `Sized` parce qu'il ne contient que trois champs de taille fixe. Les données elles-mêmes sont sur le tas, et leur quantité n'influence pas la taille du `Vec`.

Les types non `Sized` s'appellent **DST** (_dynamically sized types_).

## Pourquoi un type non `Sized` doit rester derrière un pointeur

Pour créer une variable locale, passer un argument par valeur ou stocker un champ, le compilateur doit savoir combien d'octets réserver. Pour un DST, il ne le sait pas, donc :

```rust
let s: str = ...;          // impossible
let x: dyn Backend = ...;  // impossible
fn f(x: dyn Backend) {}    // impossible
```

On les manipule **uniquement derrière un pointeur** : `&str`, `&[T]`, `&dyn Backend`, `Box<dyn Backend>`, `Arc<dyn Backend>`, `Rc<dyn Backend>`.

## Pointeurs larges (_fat pointers_)

Un pointeur vers un type `Sized` contient une seule adresse (8 octets sur 64 bits). Un pointeur vers un DST contient **l'adresse + une métadonnée** (16 octets) :

|Pointeur|Contenu|
|---|---|
|`&T` (T `Sized`)|adresse|
|`&[T]`|adresse + **longueur**|
|`&str`|adresse + **longueur**|
|`&dyn Trait` / `Box<dyn Trait>` / `Arc<dyn Trait>`|adresse + **pointeur vers la vtable**|

Schéma pour `Arc<dyn Backend>` :

```
Arc<dyn Backend>  (16 octets)
┌──────────────────┬──────────────────┐
│ adresse données  │ adresse vtable   │
└────────┬─────────┴────────┬─────────┘
         │                  │
         ▼                  ▼
   [ BackendA {...} ]   vtable de BackendA pour le trait Backend
   (sur le tas)         ┌──────────────────────────┐
                        │ fonction drop            │
                        │ taille / alignement      │
                        │ pointeur vers call()     │
                        └──────────────────────────┘
```

## `dyn Trait` : dispatch dynamique

Il y a deux façons d'écrire une fonction qui accepte « n'importe quel `Backend` ».

**Dispatch statique (génériques)**

```rust
fn run<T: Backend>(b: &T) {
    b.call("x");
}
```

Le compilateur génère **une copie de `run` par type concret** utilisé (`run::<BackendA>`, `run::<BackendB>`). C'est la **monomorphisation**. Les appels sont directs, donc inlinables, mais le type doit être connu à la compilation.

**Dispatch dynamique (`dyn`)**

```rust
fn run(b: &dyn Backend) {
    b.call("x");
}
```

Une seule copie de `run`. À chaque appel, le programme lit l'adresse de `call` dans la **vtable** puis saute dessus. Le type concret n'a pas besoin d'être connu à la compilation, ce qui permet par exemple de choisir `BackendA` ou `BackendB` selon une variable d'environnement.

| |Générique `T: Trait`|`dyn Trait`|
|---|---|---|
|Type connu à la compilation|oui|non|
|Copies de code|une par type|une seule|
|Appel de méthode|direct|via vtable|
|Inlining|possible|impossible|
|Collection hétérogène (`Vec<Box<dyn Trait>>`)|impossible|possible|
|Taille du pointeur|8 octets|16 octets|

## La contrainte `Sized` implicite des génériques

Détail important : en Rust, **tout paramètre générique est implicitement `Sized`**. Ces deux écritures sont équivalentes :

```rust
struct Service<R> { ... }
struct Service<R: Sized> { ... }
```

Donc `Service<dyn Backend>` est **refusé**, car `dyn Backend` n'est pas `Sized` :

```
error[E0277]: the size for values of type `dyn Backend` cannot be known
              at compilation time
```

## `?Sized` : relâcher la contrainte

`?Sized` se lit « peut être `Sized`, mais pas obligatoirement ». C'est la seule contrainte en Rust qui **retire** une exigence au lieu d'en ajouter une.

```rust
use std::sync::Arc;

pub struct Service<R: ?Sized + Backend> {
    backend: Arc<R>,   // OK : Arc<T: ?Sized> accepte un DST
}

// L'impl doit aussi relâcher la contrainte, sinon elle ne s'applique
// qu'aux R de taille connue.
impl<R: ?Sized + Backend> Service<R> {
    pub fn new(backend: Arc<R>) -> Self {
        Self { backend }
    }
}
```

Avec cela, les deux usages fonctionnent :

```rust
let s1: Service<BackendA>     = Service::new(Arc::new(BackendA));  // statique
let s2: Service<dyn Backend>  = Service::new(arc_dyn_backend);     // dynamique
```

Règles pratiques :

- `?Sized` n'a de sens que si le type est utilisé **derrière un pointeur** (`Arc<R>`, `Box<R>`, `&R`).
- Si `R` était stocké directement comme champ, `R: ?Sized` ne servirait à rien (et ne pourrait être que le dernier champ).
- Il faut penser à écrire `?Sized` aussi sur les blocs `impl`, sinon l'erreur apparaît à l'usage et pas à la définition.
- Quand on n'utilise **jamais** `dyn`, on n'a pas besoin de `?Sized`. C'est pour cela que l'approche par feature flags peut s'en passer.

## Limite : tous les traits ne sont pas utilisables avec `dyn`

Un trait doit être _dyn-compatible_ (anciennement « object safe ») pour servir de `dyn Trait`. Les cas qui l'empêchent le plus souvent :

- une méthode générique (`fn foo<T>(&self, x: T)`) ;
- une méthode qui retourne `Self` (`fn clone_me(&self) -> Self`) ;
- une méthode sans `self` qui n'a pas `where Self: Sized` ;
- le trait exige `Sized` comme supertrait (`trait Backend: Sized`).

Les fonctions `async` dans les traits demandent aussi un traitement particulier pour être utilisées avec `dyn` (par exemple le crate `async-trait`, qui boxe le futur retourné).

## Mémo

- `Sized` = taille connue à la compilation. Par défaut, tout générique l'exige.
- `dyn Trait` = type concret inconnu à la compilation, donc **non `Sized`**, donc **toujours derrière un pointeur** (`Box`, `Arc`, `&`).
- Pointeur vers `dyn Trait` = adresse + vtable (16 octets).
- `?Sized` = « j'accepte aussi les types dont la taille est inconnue », indispensable pour écrire `Service<dyn Trait>`.
- Pas de `dyn` → pas de `?Sized` → monomorphisation et zéro surcoût.