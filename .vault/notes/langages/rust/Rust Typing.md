---
title: Rust Typing
tags:
  - rust
  - code
date: 2026-09-23
publish: true
---
# Rust Newtypes — Guide de bonnes pratiques

> Résumé basé sur [The Ultimate Guide to Rust Newtypes](https://www.howtocodeit.com/guides/ultimate-guide-rust-newtypes)

## 1. Pourquoi utiliser des newtypes ?

Un newtype (`struct EmailAddress(String)`) sert à bien plus qu'à contourner l'_Orphan Rule_ (implémenter un trait externe sur un type externe). Son vrai pouvoir : rendre **impossible la représentation d'un état invalide**.

- Sans newtype, les fonctions métier reçoivent des `&str`, `f64`, etc. génériques → elles doivent revalider les données à chaque appel, ce qui pollue la logique métier avec des vérifications, complexifie les types d'erreurs, et multiplie les cas de test (y compris des combinaisons de validations).
- Avec des newtypes, la validation est faite **une seule fois, à la construction**. Une fois l'objet créé, on sait qu'il est valide — plus besoin de le revérifier.

**Principe clé : "Parse, don't validate"** (Alexis King) — ne validez pas des données brutes profondément dans votre code, transformez-les (parse) en un type qui _garantit_ leur validité dès l'entrée dans le système.

## 2. Les fondamentaux du newtype

- Ne rendez **jamais** le champ interne `pub` (`pub struct EmailAddress(pub String)`) : ça permettrait de construire l'objet sans passer par la validation, annulant tout l'intérêt du pattern.
- Le **constructeur est la seule source de vérité**. Toute logique de validation doit vivre dans `new()` (ou équivalent), jamais dispersée dans le code métier.
- Un constructeur fallible retourne un `Result<Self, MonErreurType>` avec un type d'erreur dédié et simple.
- **Mutabilité** : si le newtype expose des méthodes `&mut self`, chacune doit préserver les invariants du type (ex. un `NonEmptyVec` ne doit jamais pouvoir devenir vide via `pop`). En contrepartie, d'autres méthodes deviennent plus simples/infaillibles (plus besoin de retourner `Option`).

## 3. Implémentations de traits à connaître

### `derive` les traits standards quand ils ont du sens

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(String);
```

- Faites-en une habitude systématique, même sans besoin immédiat — c'est particulièrement important pour du code de librairie (à cause de l'Orphan Rule, vos utilisateurs ne pourront pas les ajouter eux-mêmes).
- Ne dérivez pas `Default` si un "valeur par défaut" n'a pas de sens métier (une adresse email par défaut, par exemple).
- `Display` n'a pas de macro `derive` : implémentez-le à la main. Bonus : ça donne `ToString` gratuitement — donc n'implémentez jamais `to_string` manuellement (Clippy le signale comme une erreur).

### Cas particuliers à implémenter manuellement

- Si le type interne n'a pas d'égalité/ordre total (ex. `f64` à cause de `NAN`) mais que votre newtype, lui, en a un (ex. `Subsecond` borné à `0.0..1.0`), implémentez `Eq`/`Ord` manuellement.
- Règle Clippy importante : si vous implémentez `Ord` manuellement, `PartialOrd::partial_cmp` doit simplement déléguer à `Ord::cmp` (`Some(self.cmp(other))`) pour éviter toute divergence entre les deux traits.
- Inversement, si `PartialEq` est correct et que l'égalité est totale, ajoutez `impl Eq for MonType {}` sans le dériver.

## 4. Constructeurs ergonomiques : `From` / `TryFrom`

- **Conversion infaillible → `From`** (donne `Into` gratuitement via l'implémentation blanket de la std).
- **Conversion faillible → `TryFrom`** (avec `type Error = ...`).
- **Règle d'or : un seul constructeur canonique.** `TryFrom::try_from` doit simplement appeler votre `new()` — n'écrivez jamais deux implémentations concurrentes de la même validation (double maintenance, double tests, pour rien).
- `FromStr` est un "relique" antérieure à `TryFrom` : à implémenter seulement si vous interfacez du code legacy qui l'exige, ou si vous voulez profiter de `str::parse` / `serde_with::DeserializeFromStr`. Sinon, évitez-le (une chose de moins à tester) — mais si votre équipe l'adopte, soyez cohérents et documentez le choix.

## 5. Revenir au type primitif : `AsRef`, `Deref`, `Borrow`

### Getters explicites

Commencez par des méthodes nommées clairement (`into_string`, `as_str`, etc.) suivant les [conventions de l'API Guidelines Rust](https://rust-lang.github.io/api-guidelines/naming.html) (`as_`, `to_`, `into_`).

### `AsRef<T>`

Le moyen le plus simple et sûr d'exposer une référence vers le type interne (ex. `AsRef<str>` pour interfacer avec du code qui attend `&str`).

### `Deref` — à utiliser avec prudence ⚠️

- Permet la _deref coercion_ : `&EmailAddress` devient utilisable là où un `&str` est attendu, et donne accès à toutes les méthodes `&self` du type sous-jacent (façon héritage).
- **Danger** : ça élargit énormément la surface publique de votre type (ex. `EmailAddress::is_empty()` n'a pas de sens métier mais apparaîtrait quand même).
- Pour un wrapper générique (`struct SmartBox<T>(T)`), préférez des **fonctions associées** plutôt que des **méthodes inhérentes** : une méthode inhérente sur le wrapper peut masquer silencieusement une méthode de même nom sur le type interne via `Deref`.
- Rappel subtil : `*valeur` désucre en `*Deref::deref(&valeur)`, donc il déréférence "jusqu'au bout" — ce n'est pas juste équivalent à `deref()`.

### `Borrow<T>` — le plus dangereux du lot ⚠️⚠️

- Contrat implicite (non vérifié par le compilateur) : un type `Borrow<T>` doit avoir **exactement** le même comportement de `Eq`, `Ord` et `Hash` que `T`.
- Violer ce contrat casse silencieusement les `HashMap`/`HashSet` (une clé insérée avec le newtype devient introuvable via le type emprunté, ou l'inverse).
- Exemple classique : si `EmailAddress` implémente une égalité insensible à la casse mais que `Borrow<str>` compare la casse — bug garanti.
- **Solution** : normalisez (ex. passage en minuscules) directement dans le constructeur, pour que le newtype et le type interne soient réellement équivalents pour le hachage/l'égalité/l'ordre.
- Scrutez systématiquement toute implémentation de `Borrow` en revue de code.

## 6. Contourner la validation (volontairement)

Parfois, revalider une donnée déjà connue comme valide est un coût inutile (ex. relire une adresse email déjà validée depuis la base de données).

Deux approches :

1. **Types marqueurs** (pattern plus avancé, façon "provenance" au niveau du système de types) — pas détaillé dans cet article.
2. **`unsafe` + convention `_unchecked`**, à la manière de `String::from_utf8_unchecked` :

```rust
impl EmailAddress {
    pub fn new(raw: &str) -> Result<Self, EmailAddressError> { /* validation */ }

    /// # Safety
    /// L'appelant doit garantir que `raw` est déjà une adresse email valide.
    pub unsafe fn new_unchecked(raw: &str) -> Self {
        Self(raw.to_string())
    }
}
```

`unsafe` ici est un **signal humain** : "il existe un invariant non vérifié par le compilateur, sois prudent". À traiter avec la même vigilance en revue de code qu'un vrai `unsafe` mémoire.

## 7. Réduire le boilerplate

Écrire ces newtypes à la main d'abord (pour bien comprendre le pattern), puis, si besoin, utiliser :

- **[`derive_more`](https://docs.rs/derive_more)** : ajoute des macros `derive` pour `From`, `AsRef`, `Deref`, opérateurs arithmétiques, etc. Bon compromis, peu de "magie".
    
    ```rust
    #[derive(Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, AsRef, Deref)]struct EmailAddress(String);
    ```
    
- **[`nutype`](https://docs.rs/nutype)** : macro procédurale complète qui génère sanitation + validation + types d'erreurs dédiés. Très puissant mais :
    - messages d'erreur générés assez vagues, non personnalisables ;
    - devient vite une dépendance structurante difficile à retirer une fois adoptée.
    - À évaluer sérieusement avant d'engager tout un projet dessus.

## 8. Checklist résumée

- [ ] Le champ interne du newtype n'est **jamais** `pub`
- [ ] Toute la validation vit dans un constructeur unique (`new`, faillible ou non)
- [ ] Pas de constructeurs concurrents qui dupliquent la même logique
- [ ] Traits standards dérivés systématiquement quand ils ont un sens métier
- [ ] `Display` implémenté à la main (pas de `to_string` manuel)
- [ ] `Ord`/`PartialOrd` et `Eq`/`PartialEq` cohérents entre eux si implémentés manuellement
- [ ] `From`/`TryFrom` définis en appelant le constructeur canonique
- [ ] `Deref` utilisé avec parcimonie ; fonctions associées plutôt que méthodes inhérentes sur les wrappers génériques
- [ ] `Borrow` implémenté seulement si `Eq`/`Ord`/`Hash` sont réellement identiques au type interne
- [ ] Toute bascule `_unchecked` documentée et passée en `unsafe`
- [ ] Considérer `derive_more`/`nutype` seulement après avoir maîtrisé le pattern à la main