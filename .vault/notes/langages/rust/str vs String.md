---
title: str vs String
tags:
  - rust
  - todo
  - code
date: 2026-10-11
publish: true
---

`String` est une chaîne UTF-8 **possédée**, extensible et allouée sur le tas. `&str` est une **vue empruntée** (pointeur + longueur) sur des données UTF-8 qui vivent ailleurs. **Mémo :** `String` est à `&str` ce que `Vec<u8>` est à `&[u8]`, avec en plus la garantie que les octets forment de l'UTF-8 valide.

---

# 1. Les bases

| |`String`|`&str`|
|---|---|---|
|Propriété|Possède ses données|Emprunte|
|Modifiable|Oui (`push_str`, `push`, `insert`...)|Non (lecture seule)|
|Allocation|Tas|Aucune (pointe vers l'existant)|
|Taille de la valeur (64 bits)|24 octets (ptr, len, cap)|16 octets (ptr, len)|
|Durée de vie|Indépendante|Liée à la source empruntée|

```rust
let literal: &str = "bonjour";               // &'static str, intégré au binaire
let owned: String = String::from("bonjour"); // allocation sur le tas
let borrowed: &str = &owned;                 // coercion par déréférencement : &String -> &str
let slice: &str = &owned[1..3];              // "on" (indices en octets, aux frontières de caractères)

let mut s = owned;
s.push_str(" le monde");                     // seule String peut grandir
```

**Conversions**

- `&str` → `String` : `.to_string()`, `.to_owned()`, `String::from(s)`, `s.into()` (alloue et copie)
- `String` → `&str` : `&s`, `s.as_str()` (gratuit)

---

# 2. Où vivent les données en mémoire

```
String                         Tas
┌─────────┐                    ┌───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr ────┼──────────────────► │ b │ o │ n │ j │ o │ u │ r │ ? │
│ len = 7 │                    └───┴───┴───┴───┴───┴───┴───┴───┘
│ cap = 8 │                      ▲
└─────────┘                      │
&str (sous-tranche)              │
┌─────────┐                      │
│ ptr ────┼──────────────────────┘ (ou n'importe où dans la source)
│ len     │
└─────────┘
```

- Un **littéral** (`"abc"`) est stocké dans la section en lecture seule du binaire (`.rodata`). Aucune allocation à l'exécution, durée de vie `'static`, et il est chargé gratuitement avec le programme.
- Un **`&str` sur une `String`** pointe vers le tampon du tas : aucune copie.
- Une **sous-tranche** `&s[a..b]` ne copie rien : on obtient simplement un nouveau couple (ptr, len).

---

# 3. Coût des opérations

### Ce qui est gratuit ou en O(1)

- `String` → `&str` (`as_str`, `&s`) : simple réinterprétation, zéro coût.
- `.len()` : O(1), la longueur en **octets** est stockée.
- Découper `&s[a..b]` : O(1) (avec une vérification que `a` et `b` tombent sur des frontières de caractères).
- Passer un `&str` à une fonction : on copie 16 octets, jamais le contenu.
- **Déplacer** (`move`) une `String` : on copie 24 octets, le tampon du tas n'est pas touché.

### Ce qui coûte

- `&str` → `String` (`to_string`, `to_owned`, `String::from`, `into`) : **une allocation + une copie** en O(n). Ces quatre variantes sont équivalentes en performance.
- `.clone()` d'une `String` : même coût, allocation + copie.
- `.chars().count()` : O(n), il faut parcourir l'UTF-8. À ne pas confondre avec `.len()`.
- `String::from_utf8(vec)` : validation UTF-8 en O(n) (mais sans copie, le `Vec` est réutilisé). `from_utf8_unchecked` évite la validation mais est `unsafe`.
- `format!` : alloue une nouvelle `String` à chaque appel.

### Croissance d'une `String`

`push` / `push_str` sont en **O(1) amorti** : quand la capacité est dépassée, le tampon est réalloué avec une capacité plus grande (croissance géométrique, comme `Vec`), puis les données sont copiées. Chaque réallocation peut déplacer tout le contenu.

---

# 4. Optimisations concrètes

## 4.1 Préallouer avec `with_capacity`

```rust
// Mauvais : plusieurs réallocations pendant la croissance
let mut s = String::new();
for part in &parts {
    s.push_str(part);
}

// Meilleur : une seule allocation si on connaît (ou estime) la taille
let total: usize = parts.iter().map(|p| p.len()).sum();
let mut s = String::with_capacity(total);
for part in &parts {
    s.push_str(part);
}
```

Voir aussi `reserve` / `reserve_exact`, et `shrink_to_fit` pour rendre la mémoire inutilisée quand la chaîne est figée.

## 4.2 Prendre `&str` en paramètre, pas `String`

```rust
// Force l'appelant à allouer ou à céder la propriété
fn compter(s: String) -> usize { s.len() }

// Accepte un littéral, une &String, une tranche : aucune allocation imposée
fn compter(s: &str) -> usize { s.len() }
```

`&String` en paramètre est presque toujours inférieur à `&str` (une indirection de plus, API moins souple). Clippy le signale (`ptr_arg`).

## 4.3 Éviter les allocations inutiles : le parsing zéro-copie

```rust
// Alloue une String par morceau
let mots: Vec<String> = texte.split_whitespace().map(String::from).collect();

// Zéro allocation par morceau : on ne garde que des tranches du texte original
let mots: Vec<&str> = texte.split_whitespace().collect();
```

Comparaison pour N mots :

| |`Vec<String>`|`Vec<&str>`|
|---|---|---|
|Taille par élément|24 octets|16 octets|
|Allocations tas|N (une par chaîne) + 1 (le vecteur)|1 (le vecteur)|
|Localité cache|Données dispersées sur le tas|Éléments contigus, données dans la source|
|Contrainte|Aucune|La source doit survivre aux tranches|

Les itérateurs de `str` (`split`, `lines`, `trim`, `split_whitespace`, `char_indices`...) retournent des `&str` **sans rien allouer** : à privilégier tant que la source reste en vie.

## 4.4 Concaténer sans allocations superflues

```rust
// `a + &b` réutilise le tampon de `a` (qui est consommée) : pas de nouvelle allocation si la capacité suffit
let s = a + &b;

// format! alloue toujours une nouvelle String
let s = format!("{a}{b}");

// Écrire dans un tampon existant : zéro allocation supplémentaire
use std::fmt::Write;
let mut buf = String::with_capacity(64);
write!(buf, "{a}-{b}").unwrap();
```

Dans une boucle chaude, réutiliser un seul tampon (`buf.clear()` puis réécriture) évite de réallouer à chaque itération : `clear()` garde la capacité.

## 4.5 Retourner une référence plutôt qu'un clone quand c'est possible

```rust
// Alloue à chaque appel
fn premier_mot(s: &str) -> String {
    s.split(' ').next().unwrap_or("").to_string()
}

// Zéro allocation, la durée de vie est liée à `s`
fn premier_mot(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}
```

## 4.6 Structs qui empruntent

```rust
// Possède : allocation par champ, indépendante de la source
struct Personne { nom: String }

// Emprunte : aucune allocation, mais porte un lifetime
struct PersonneRef<'a> { nom: &'a str }
```

C'est le modèle des parseurs zéro-copie. Contrepartie : la struct ne peut pas survivre à la source, et le lifetime se propage dans les types qui la contiennent. Si la struct doit voyager entre threads, être stockée longtemps ou retournée librement, `String` est plus simple et le coût est souvent négligeable.

---

# 5. Alternatives selon le besoin mémoire/perf

|Type|Taille de la valeur (64 bits)|Quand l'utiliser|
|---|---|---|
|`String`|24 octets|Texte modifiable, propriété exclusive|
|`&str`|16 octets|Lecture seule, emprunt|
|`Box<str>`|16 octets|Texte **immuable** possédé : économise le champ `capacity` et la capacité excédentaire (`into_boxed_str()`)|
|`Rc<str>` / `Arc<str>`|16 octets|Texte immuable **partagé** : clone en O(1) (incrément du compteur), sans copie des données|
|`Cow<'a, str>`|24 octets|Emprunte par défaut, alloue seulement si une modification est nécessaire|
|`&'static str`|16 octets|Constantes et littéraux : aucune allocation|

## `Cow<str>` : n'allouer que si nécessaire

```rust
use std::borrow::Cow;

fn normaliser(s: &str) -> Cow<str> {
    if s.contains('\t') {
        Cow::Owned(s.replace('\t', "    "))   // alloue seulement dans ce cas
    } else {
        Cow::Borrowed(s)                       // zéro allocation dans le cas courant
    }
}
```

## `Box<str>` pour de nombreuses chaînes figées

Pour des millions de petites chaînes qui ne changent plus, passer de `String` à `Box<str>` économise 8 octets par valeur et supprime la capacité excédentaire du tampon.

## `Arc<str>` pour partager sans copier

```rust
use std::sync::Arc;
let nom: Arc<str> = Arc::from("config");
let nom2 = Arc::clone(&nom);   // O(1), pas de copie du texte
```

### Petites chaînes

La bibliothèque standard n'a **pas** d'optimisation pour les petites chaînes : même `"a"` dans une `String` coûte une allocation sur le tas. Si votre profil montre beaucoup de petites chaînes, des crates comme `compact_str`, `smol_str` ou `smallstr` stockent les courtes chaînes directement dans la valeur (sans tas). À valider par mesure.

---

# 6. Indexation, UTF-8 et coût caché

- `s[0]` est interdit : un caractère UTF-8 fait de 1 à 4 octets, donc l'indexation par position de caractère ne serait pas O(1).
- `&s[a..b]` est en O(1) mais **panique** si `a` ou `b` coupe un caractère en deux. Vérifier avec `s.is_char_boundary(i)` ou utiliser `s.get(a..b)` qui retourne un `Option`.
- Accès à la n-ième lettre : `s.chars().nth(n)` est en **O(n)**. Dans une boucle, `for i in 0..s.chars().count() { s.chars().nth(i) }` devient quadratique.
- Si la donnée est de l'ASCII pur et que vous avez besoin d'accès aléatoire, travailler sur `s.as_bytes()` (`&[u8]`) est en O(1) et sans surcoût.

---

# 7. Règles pratiques

**Pour les API**

- Paramètre en lecture seule : `&str`.
- Retourner `String` quand on **crée** du nouveau texte, `&str` quand on renvoie une portion de ce qu'on a emprunté.
- Champ de struct : `String` par défaut, `&'a str` si on veut du zéro-copie et qu'on accepte le lifetime.
- Besoin d'emprunter parfois et de posséder parfois : `Cow<'a, str>`.

**Pour les performances**

1. Éviter `to_string()` / `clone()` / `format!` dans les boucles chaudes.
2. `String::with_capacity` quand la taille finale est connue ou estimable.
3. Réutiliser un tampon (`clear()` conserve la capacité).
4. Préférer les itérateurs de `&str` aux collections de `String`.
5. Pour du texte immuable et nombreux : `Box<str>`. Pour du texte immuable partagé : `Arc<str>`.
6. **Mesurer avant d'optimiser** : `criterion` ou `divan` pour le temps, `dhat` ou `heaptrack` pour les allocations, `cargo flamegraph` pour le profil. L'allocation n'est pas toujours le goulot, et un code plus simple avec `String` est souvent assez rapide.

---

# 8. Pièges

- `&String` comme type de paramètre : presque toujours moins bon que `&str`.
- Cloner par réflexe pour faire taire le borrow checker : chaque `.clone()` sur une `String` est une allocation + copie.
- Confondre `.len()` (octets, O(1)) et `.chars().count()` (caractères, O(n)).
- Découper un `&str` à un indice arbitraire : panique si ce n'est pas une frontière de caractère.
- Supposer que les petites chaînes sont « gratuites » : elles passent quand même par l'allocateur.