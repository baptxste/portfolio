# Cookbook — Schémas « HTML/SVG → PNG »

Document **autonome** : il contient tout ce qu'il faut pour créer un nouveau schéma de ce type sans rien d'autre sous la main (règles de rendu, palette, classes CSS, composants, icônes, patrons complets, checklist). Toutes les règles marquées *(testé)* ont été vérifiées en exécutant le vrai moteur de rendu (`resvg_py` 0.5.0) avec le vrai `schema_renderer.py`.

---

## 1. Ce qu'est ce type de fichier

Un schéma est un fichier `nom.html` (ou `nom.svg`) qui contient **un unique élément `<svg>…</svg>`**, accompagné optionnellement d'un `nom.json` de même nom. Le générateur le transforme en `nom.png` **à fond transparent**.

- Le fichier s'appelle « html » mais le moteur ne lit que le SVG : **pas de `div`, pas de flexbox, pas de CSS de mise en page, pas de JavaScript, pas de `foreignObject`** *(testé : `foreignObject` ne rend rien)*.
- Tout est positionné **à la main en coordonnées absolues** (x, y, width, height, `transform="translate(...)"`).
- Le PNG est posé tel quel sur un site qui a un **thème clair et un thème sombre** : il doit rester lisible sur les deux fonds (voir §6).

### Chaîne de traitement (dans l'ordre)

1. Découverte : tous les fichiers `.html` et `.svg` du dossier des schémas, récursivement. Les noms qui commencent par `.` ou `_` sont ignorés (pratique pour des brouillons : `_brouillon.html`).
2. Lecture du `.json` de même nom (même dossier, même nom de base). S'il est absent ou illisible, config vide (avertissement affiché si illisible).
3. Extraction par regex du **premier** `<svg … </svg>` du fichier (tout ce qui est avant/après est jeté).
4. Injection automatique des `<defs id="system-defs">` (marqueurs de flèches) juste après la balise `<svg>` ouvrante, sauf si le SVG contient déjà `id="system-defs"`.
5. Injection automatique de `rx`/`ry` sur les `<rect>` concernés (voir §4).
6. Construction de la feuille de style commune (tous les `.card`, `.pill`, `.arrow`…), à laquelle s'ajoute `extra_css` du JSON s'il existe.
7. Rendu PNG par resvg avec les options du JSON (`width`, `height`, `zoom`, `dpi`).
8. Sortie : même chemin relatif que le fichier source, extension `.png`, dans le dossier d'assets (`schemas/archi/x.html` → `<assets>/archi/x.png`).

Lancement : le script principal découvre et rend à la fois les scripts matplotlib et les schémas ; l'option `--filter <texte>` ne garde que les fichiers dont le chemin contient ce texte (insensible à la casse) ; `--schemas-dir` et `--assets-dir` changent les dossiers d'entrée/sortie. Code de sortie 1 si au moins un rendu échoue.

---

## 2. Le fichier JSON (toutes les options)

Les **seules clés lues** sont les cinq ci-dessous. Toute autre clé est ignorée.

| Clé | Type | Effet | Défaut |
|---|---|---|---|
| `width` | entier (px) | Largeur du PNG. La hauteur suit le ratio du SVG. | taille du SVG |
| `height` | entier (px) | Hauteur du PNG. La largeur suit le ratio. | taille du SVG |
| `zoom` | nombre | Multiplicateur de la taille d'origine (`2` → PNG 2× plus grand, net sur écrans retina). | 1 |
| `dpi` | nombre | Résolution pour les unités physiques (mm, pt). **Sans effet sur un SVG en pixels** *(testé)*. | 0 |
| `extra_css` | chaîne | CSS ajouté **après** la feuille commune (donc prioritaire à spécificité égale). Sert à définir des classes propres au schéma ou à surcharger une classe commune. | aucun |

Comportements de taille *(testés sur un SVG 400×200)* :

| JSON | PNG obtenu |
|---|---|
| `{}` | 400×200 |
| `{"width": 800}` | 800×400 |
| `{"height": 100}` | 200×100 |
| `{"width": 800, "height": 800}` | 800×400 (ratio conservé : le SVG est **inscrit** dans la boîte) |
| `{"zoom": 2}` | 800×400 |
| `{"width": 800, "zoom": 2}` | 800×400 (`width` l'emporte sur `zoom`) |
| `{"dpi": 192}` | 400×200 (aucun effet) |

Conseils :
- Toujours mettre `width` et `height` égaux aux attributs `width`/`height` de la balise `<svg>` (ils documentent la taille nominale).
- Pour un rendu net, ajouter `"zoom": 2`. Sans zoom, le PNG fait exactement la taille du SVG (ex. 1200×460), ce qui est pixelisé sur écran haute densité.
- Un `viewBox` seul (sans `width`/`height`) fonctionne aussi, mais garder les trois pour la cohérence.

Exemple minimal :

```json
{ "width": 1200, "height": 460, "zoom": 2 }
```

Exemple avec CSS propre au schéma :

```json
{
  "width": 900,
  "height": 380,
  "zoom": 2,
  "extra_css": ".card-bronze { fill: #fbf6f1; stroke: #a87954; stroke-width: 1.8px; } .halo-thin { paint-order: stroke fill; stroke: #ffffff; stroke-width: 1px; stroke-linejoin: round; }"
}
```

Autre option équivalente pour du CSS propre au schéma : un bloc `<style>…</style>` **à l'intérieur du `<svg>`** *(testé, fonctionne)*. C'est le choix le plus robuste car le fichier reste autonome, sans dépendre du JSON. Les deux peuvent coexister.

> Options resvg qui existent mais que le script **n'expose pas** (nécessiteraient de modifier le renderer) : `background`, `font_files`, `font_dirs`, `font_family`, `skip_system_fonts`, `font_size`, `shape_rendering`, `text_rendering`, `image_rendering`, `resources_dir`.

---

## 3. Règles techniques dures *(toutes testées)*

### 3.1 Priorité des styles — le piège n° 1

Du plus fort au plus faible :

1. `style="fill:#xxx"` **inline** : gagne toujours.
2. Règles CSS de classe (`.title`, `.card`…), à spécificité égale la **dernière** règle gagne (`extra_css` et `<style>` interne passent après la feuille commune).
3. Règle d'élément `text { fill: #286044 }` de la feuille commune.
4. **Attribut de présentation** `fill="#xxx"` : le plus faible.

Conséquences :

| Situation | Résultat |
|---|---|
| `<text fill="#a87954">` | **Ignoré** : le texte sort en vert `#286044`. |
| `<text style="fill:#a87954">` | Brun, OK. |
| `<text class="t-bronze-1">` avec `.t-bronze-1 { fill:#784f32 }` défini | Brun foncé, OK. |
| `<text fill="#ffffff">` | Blanc, OK (règle spéciale `text[fill="#ffffff"]`). |
| `<text class="text-white">` | Blanc, OK. |
| `<rect fill="#d4e9e2">` / `<path fill=…>` / `<circle fill=…>` **sans classe** | L'attribut fonctionne (aucune règle globale sur les formes). |
| `<rect class="card" fill="#fdf6ee">` | `fill` **ignoré** (la classe `.card` impose son fill). Utiliser `style="fill:#fdf6ee"`. |
| `<rect class="card" stroke-width="2.2">` | `stroke-width` **ignoré** (la classe impose 1.8px). Utiliser `style="stroke-width:2.2px"`. |

Règle pratique : **pour toute couleur de texte autre que vert sapin ou blanc, définir une classe (dans `<style>` ou `extra_css`) ou utiliser `style="fill:…"`. Jamais l'attribut `fill` sur un `<text>`.**

### 3.2 Classes inconnues et formes sans style

- Une forme **sans fill défini** (aucune classe qui en donne un, aucun attribut) est **remplie en noir**. Une classe inconnue (ex. `card-bronze` non définie) ne donne rien → noir. *(C'est exactement le défaut de l'exemple d'origine : les cartes bronze/argent/or sortaient noires tant que ces classes n'étaient pas définies.)*
- **Toute classe utilisée doit exister** : soit dans le catalogue du §5, soit définie dans le `<style>` interne / `extra_css`.
- Un `<text>` sans `font-size` est rendu à **16px** (défaut resvg). Toujours préciser `font-size` ou utiliser `.title` (14px) / `.subtitle` (11px).

### 3.3 Ne jamais poser une classe de forme sur un `<g>`

`.pill-accent`, `.card`, `.card-accent`… posent un `stroke`. Sur un `<g>`, le trait est **hérité par les `<text>` enfants** : le texte devient gras et baveux *(testé)*. Pire, la règle `.pill-accent text, .card-accent text` force le texte en vert sapin (illisible sur `.card-accent` qui est lui-même vert sapin) et ne peut pas être contournée, même par `.text-white`.

✅ `<g transform="…"><rect class="pill-accent" … /><text …>…</text></g>`
❌ `<g class="pill-accent"><rect … /><text …>…</text></g>`

### 3.4 Interdits / non supportés

| Interdit | Pourquoi |
|---|---|
| **`<svg>` imbriqué** | La regex d'extraction s'arrête au premier `</svg>` : erreur « the root node was opened but never closed » *(testé)*. Pour réutiliser une icône, utiliser `<g transform>` ou `<symbol>` + `<use>`. |
| `foreignObject` / HTML / `div` | Rien n'est rendu. |
| `var(--…)` | Non résolu : la forme sort noire. Les `--color-*` du `:root` existent mais **ne sont pas utilisables**. Écrire les hex en dur. |
| `!important` dans ses propres règles | Non fiable : ne pas compter dessus (les seules règles `!important` existantes sont `.text-white` et `.text-accent`). |
| Redéfinir les ids `arrow`, `arrow-soft`, `arrow-gold`, `dot` | Ces ids sont injectés automatiquement ; créer ses propres marqueurs avec **d'autres ids** (voir §6). |
| Entités XML manquantes | `&` doit être écrit `&amp;`, `<` → `&lt;`. Un `&` nu casse le parsing. |
| Texte qui dépasse | Aucun retour à la ligne automatique : tout `<text>` est une ligne unique (voir §7). |

### 3.5 Supporté techniquement *(testé)*, à utiliser avec parcimonie

La charte est « flat » (aplats, zéro dégradé). Techniquement, le moteur rend aussi : `linearGradient`, `filter` avec `feDropShadow`, `opacity` / `fill-opacity`, `clip-path`, `<symbol>` + `<use href>`, `<textPath>` (texte sur courbe), `dominant-baseline` (`middle`, `hanging`), `transform` (`rotate`, `scale`, `translate`), `currentColor`, `marker-start`/`marker-end`/`marker-mid`, `stroke-dasharray`, `stroke-linecap`, emojis et symboles Unicode (π, ≥, ∑, →, ✓).
Dans la charte : **pas de dégradé, pas d'ombre** par défaut. Un aplat avec `opacity` est acceptable pour un fond doux.

### 3.6 Polices

La pile `font-family` est : `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", "Helvetica Neue", sans-serif`. Le rendu prend la **première police réellement installée** sur la machine qui génère. Les largeurs de texte changent d'une police à l'autre : prévoir **~15 % de marge** sur toute largeur de texte (§7). Les graphiques matplotlib utilisent `DejaVu Sans` (police différente, plus large) : ne pas s'attendre à des largeurs identiques entre schémas et graphiques.

---

## 4. Ce que le renderer fait tout seul

### 4.1 `rx`/`ry` injectés sur les `<rect>`

Si un `<rect>` **n'a pas d'attribut `rx`** :
- et que sa classe contient `pill` → `rx="24" ry="24"` ;
- sinon, si sa classe contient `card`, `box` ou `container` → `rx="18" ry="18"`.

La détection est une **sous-chaîne** du nom de classe : `card-bronze`, `card-gold`, `pill-soft`, `container-dashed`, `box-subtle` sont tous concernés. Un `rx="0"` explicite est respecté (rect carré). **Toujours écrire `rx` et `ry` explicitement** pour maîtriser l'arrondi ; l'injection n'est qu'un filet de sécurité.

### 4.2 Marqueurs de flèches

Quatre marqueurs sont toujours disponibles (voir §6 pour en créer d'autres) :

| id | Couleur | Usage |
|---|---|---|
| `arrow` | `#286044` (ACCENT) | Flèche standard (`marker-end="url(#arrow)"`). |
| `arrow-soft` | `#4a8a6e` (BAND_2) | Flèche secondaire, pointillés, skip-connection. |
| `arrow-gold` | `#cba258` | Flèche or (palier Gold). |
| `dot` | `#286044`, rond r=2.5 | Point de départ d'une ligne (`marker-start="url(#dot)"`). |

Les trois flèches ont `markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto"` : **la pointe tombe exactement sur l'extrémité du chemin**. Terminer le tracé 4 à 5 px avant le bord de la cible.

---

## 5. Palette et catalogue complet des classes

### 5.1 Palette

| Nom | Hex | Usage |
|---|---|---|
| ACCENT / TEXT / AXIS / CURVE | `#286044` | Vert sapin de marque : contours, texte, flèches, titres. |
| BAND_1 | `#286044` | = ACCENT. |
| BAND_2 | `#4a8a6e` | Sous-titres, flèches secondaires. |
| BAND_3 | `#7fb39a` | Aplats moyens (face haute du cube, liens de réseau). |
| BAND_4 | `#c3ddd2` | Fond doux (`.card-soft`). |
| HALO | `#ffffff` | Liseré clair derrière le texte nu. |
| GRID | `#a9c2b8` | Pointillés, lignes guides, conteneurs. |
| Pastel | `#d4e9e2` | Fond de `.pill-accent`, faces d'icônes. |
| Vert vif | `#10b981` | Pastille « live » / nœud actif. |
| Bleu | `#3b82f6` | Pastille bleue (`.dot-blue`). |

Palette médaillon (clé : `bandeau` = header et pastille, `fond` = corps de carte, `t1` = texte fort, `t2` = texte secondaire, `trait` = pointillé interne) :

| Palier | bandeau / contour | fond | t1 | t2 | trait | flèche vers le suivant |
|---|---|---|---|---|---|---|
| Bronze | `#a87954` | `#fbf6f1` | `#784f32` | `#a87954` | `#d9beab` | `#a87954` |
| Argent | `#5a7d8c` | `#f3f7f9` | `#3c5966` | `#5a7d8c` | `#b8ccd4` | `#5a7d8c` |
| Or | `#cba258` | `#fcf8ef` | `#8c6b2d` | `#b08a3c` | `#dfc49d` | `#cba258` |

Règle de composition : une **famille de teintes par rôle** (vert = neutre/flux, brun/bleu-gris/or = paliers). Au-delà de ces familles, rester dans des teintes pastel désaturées et garder le texte à fort contraste (le plus foncé de la famille).

### 5.2 Classes texte

| Classe | Propriétés |
|---|---|
| *(règle `text`)* | `font-family` (pile §3.6), `letter-spacing:-0.1px`, `fill:#286044` |
| `.title` | 14px, graisse 700, `fill:#286044` |
| `.subtitle` | 11px, graisse 500, `fill:#4a8a6e` |
| `.subtext` | 11px, `fill:#4a8a6e` (ancienne forme, compat.) |
| `.text-white` | `fill:#ffffff !important` (aussi appliqué à tout `text[fill="#ffffff"]`) |
| `.text-accent` | `fill:#286044 !important` |
| `.halo` | `paint-order:stroke fill; stroke:#ffffff; stroke-width:2px; stroke-linecap:round; stroke-linejoin:round` |
| `.badge-text` | `fill:#ffffff`, 11px, graisse 600, police monospace (`ui-monospace, SFMono-Regular, Menlo, monospace`) |

Les classes se combinent : `class="title halo"` (titre 14px avec halo) fonctionne.

### 5.3 Classes de formes

| Classe | Fill | Stroke | Épaisseur | Usage |
|---|---|---|---|---|
| `.card` | `#ffffff` | `#286044` | 1.8px | Carte standard blanche. |
| `.card-accent` | `#286044` | `#286044` | 1.8px | Carte pleine vert sapin (texte **blanc** obligatoire). |
| `.card-soft` | `#c3ddd2` | `#286044` | 1.5px | Carte à fond vert doux. |
| `.pill` | `#ffffff` | `#286044` | 1.8px | Capsule blanche. |
| `.pill-accent` | `#d4e9e2` | `#286044` | 1.6px | Capsule pastel mise en avant. |
| `.pill-soft` | `rgba(212,233,226,0.45)` | `#4a8a6e` | 1.4px | Puce légère (translucide). |
| `.badge` | `#286044` | `#ffffff` | 1.2px | Étiquette pleine (avec `.badge-text`). |
| `.node` | `#ffffff` | `#286044` | 2.2px | Nœud circulaire (états, neurones). |
| `.db-body` | `#ffffff` | `#286044` | 1.6px | Corps de cylindre de base de données. |
| `.db-top` | `#d4e9e2` | `#286044` | 1.6px | Disque supérieur de cylindre. |
| `.container-dashed` | `none` | `#a9c2b8` | 1.5px | Cadre de groupement, `stroke-dasharray: 6 5`. |
| `.box` | `#ffffff` | `#286044` | 1.8px | Compat. (équivalent `.card`). |
| `.box-accent` | `#286044` | `#286044` | 1.8px | Compat. |
| `.box-subtle` | `#c3ddd2` | `#4a8a6e` | 1.5px | Compat. |
| `.dot-live` | `#10b981` | — | — | Pastille verte. |
| `.dot-gold` | `#cba258` | — | — | Pastille or. |
| `.dot-blue` | `#3b82f6` | — | — | Pastille bleue. |

### 5.4 Classes de traits

| Classe | Propriétés |
|---|---|
| `.arrow` | `stroke:#286044; stroke-width:2px; fill:none; stroke-linecap:round; stroke-linejoin:round` |
| `.arrow-dashed` | `stroke:#4a8a6e; stroke-width:1.8px; fill:none; stroke-dasharray:5 4; round caps/joins` |
| `.grid-line` | `stroke:#a9c2b8; stroke-width:1.2px; stroke-dasharray:4 4` |

### 5.5 Classes de paliers à définir soi-même (dans `<style>` ou `extra_css`)

Ces classes **n'existent pas** dans la feuille commune. À copier dans le schéma qui les utilise :

```css
.card-bronze { fill: #fbf6f1; stroke: #a87954; stroke-width: 1.8px; }
.card-silver { fill: #f3f7f9; stroke: #5a7d8c; stroke-width: 1.8px; }
.card-gold   { fill: #fcf8ef; stroke: #cba258; stroke-width: 2.2px; }
.t-bronze-1 { fill: #784f32; } .t-bronze-2 { fill: #a87954; }
.t-silver-1 { fill: #3c5966; } .t-silver-2 { fill: #5a7d8c; }
.t-gold-1   { fill: #8c6b2d; } .t-gold-2   { fill: #b08a3c; }
.t-link-bronze { fill: #8f6542; } .t-link-silver { fill: #46687a; } .t-link-gold { fill: #9a7430; }
.halo-thin { paint-order: stroke fill; stroke: #ffffff; stroke-width: 1px; stroke-linejoin: round; }
```

---

## 6. Lisibilité sur fond clair ET sombre

Le PNG est **transparent** et se pose sur la page quel que soit le thème. Deux principes :

1. **Les aplats opaques se voient partout.** Toute information importante vit dans une carte/capsule remplie (blanche, pastel ou colorée) : le texte vert sapin dessus est lisible sur fond clair comme sombre.
2. **Le texte posé directement sur le fond (hors carte) est invisible sur fond sombre** (vert sur vert nuit). Il faut lui ajouter `class="halo"` (liseré blanc). Concerne : titre principal, titres de groupes (« Sources »), labels de flèches, légendes.

Limites du halo *(constaté au rendu)* :
- `.halo` fait 2px : sur du texte **≤ 10px**, il devient pâteux. Pour les petits labels, utiliser `.halo-thin` (1px, §5.5) **et** passer à 10–11px.
- Sur un fond de carte clair, le halo blanc est invisible : inutile de le mettre sur du texte déjà dans une carte.
- Les traits fins hors carte (flèches, lignes guides) restent en vert sapin sans halo : acceptés tels quels.
- Ne jamais utiliser de noir ni de gris très foncé : tout texte est vert sapin, blanc (sur fond coloré) ou dans la famille d'un palier.

### Créer un marqueur de flèche propre à une couleur

Ajouter un `<defs>` dans le SVG avec un **nouvel id** :

```svg
<defs>
  <marker id="arrow-bronze" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
    <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="#a87954" />
  </marker>
</defs>
```

Puis : `<path d="…" stroke="#a87954" stroke-width="2" fill="none" marker-end="url(#arrow-bronze)" />`. Il coexiste sans problème avec les `system-defs` injectés *(testé)*.

---

## 7. Typographie et largeurs de texte

### 7.1 Échelle de tailles

| Rôle | Taille | Graisse |
|---|---|---|
| Titre principal du schéma | 16px (15px pour un petit schéma) | bold + `halo` |
| Titre de carte / de groupe | 13–14px | bold (`.title` = 14px/700) |
| Nom d'élément dans une carte | 11–12px | bold |
| Sous-titre / description | 11px | 500 (`.subtitle`) |
| Puces de liste | 11px | normal (600 pour mise en avant Gold) |
| Label de flèche | 10–11px (9px minimum, déconseillé) | 600 |
| Pastille numérotée / badge | 10–11px | bold |

Graisses réellement distinctes : 400 (défaut), 500, 600, 700 (`bold`).

### 7.2 Estimer la largeur d'un texte (indispensable : pas de retour à la ligne)

Mesuré avec la police Inter sur du texte français :

```
largeur_px ≈ nb_caractères × taille_px × 0,56     (graisse normale)
largeur_px ≈ nb_caractères × taille_px × 0,58     (gras)
```

Valeurs de référence à 11px : ~6,2 px par caractère. Exemples mesurés : « Tables SQL relationnelles » (25 car.) = 131px ; « Collections BSON / NoSQL » (24 car.) = 139px ; « IA & Machine Learning » (21 car., gras) = 120px ; « Applications & Users » (20 car., gras) = 112px.

Procédure : `largeur disponible = largeur de la carte − position x du texte − 8px de marge droite`. Si `largeur estimée > largeur disponible`, **raccourcir le texte ou élargir la carte** (jamais réduire la police sous 10px). Pour un texte centré (`text-anchor="middle"`), la largeur disponible est `largeur de la carte − 24px`.

Tableau rapide, nombre de caractères maximum par largeur disponible :

| Largeur dispo. | 11px | 12px | 13px |
|---|---|---|---|
| 100px | 16 | 14 | 13 |
| 120px | 19 | 17 | 16 |
| 140px | 22 | 20 | 19 |
| 180px | 29 | 27 | 24 |

### 7.3 Ancrage et position verticale

- Texte centré : `text-anchor="middle"` avec `x` = milieu de la carte (largeur/2).
- Texte aligné à gauche : `x` = début (pas de `text-anchor`) ; c'est le cas des cartes à icône (icône à gauche, texte à `x=48…55`).
- `y` d'un `<text>` = **ligne de base**. Dans une carte de hauteur 62 : nom à `y=28`, sous-titre à `y=44`.
- Centrer verticalement dans une capsule de hauteur H avec une taille de police S : `y ≈ H/2 + 0,35 × S`.
- `dominant-baseline="middle"` fonctionne si on préfère centrer sur `y` *(testé)*.

---

## 8. Grille et géométrie

| Élément | Valeur usuelle |
|---|---|
| Marge extérieure gauche/droite du schéma | 30px |
| Titre principal | `y=30–32`, centré sur `x = largeur/2` |
| Haut du contenu | `y=55–60` sous un titre |
| Rayon cartes de niveau 1 (paliers, conteneurs) | 16–20 |
| Rayon cartes de niveau 2 (sources, consommateurs) | 12–14 |
| Rayon capsule | demi-hauteur (hauteur 70 → `rx=35`) |
| Rayon écran / moniteur | 8 |
| Rayon badge | 6 |
| Écart entre cartes d'une même colonne | 16–20px (cartes 62px de haut espacées de 78px) |
| Écart horizontal entre deux paliers consécutifs | 80–85px (reçoit un label de flèche) |
| Écart flèche ↔ carte cible | 4–5px |
| Épaisseur de trait cartes | 1.8px ; palier mis en avant 2.2px |
| Épaisseur flèche | 2px (2.2–2.5px pour les flèches principales) |

Carte « premier plan » (palier final, élément clé) : **légèrement plus grande** que les autres (ex. +10px de haut, +10px de large, décalée de 10px vers le haut pour rester centrée), contour à 2.2px.

### Flèches courbes (aucun angle droit)

Courbe de Bézier cubique `M x1,y1 C cx1,cy1 cx2,cy2 x2,y2` ; ligne droite `M x1,y L x2,y` seulement pour un lien horizontal strict.

- **Convergence (N sources → 1 cible)** : même point d'arrivée pour toutes (centre vertical du palier cible), points de contrôle à mi-chemin horizontal :
  `M xS,yi C xm,yi xm,yC xC,yC` avec `xm = xS + (xC − xS)/2`, `yC` = centre vertical de la cible, `yi` = centre vertical de la source i. Exemple : `M 245,133 C 272,133 272,245 300,245`.
- **Divergence (1 → N)** : symétrique, même point de départ : `M xS,yC C xm,yC xm,yi xC,yi`.
- **Boucle de retour latérale (feedback, skip connection)** : les deux points de contrôle poussés loin vers l'extérieur : `M 165,267 C 60,267 60,97 165,97`.
- **Transition entre deux nœuds** : arc en cloche : `M 110,58 C 145,40 175,40 210,58`.

---

## 9. Composants réutilisables (copier-coller)

Convention : chaque composant est dans un `<g transform="translate(x, y)">` ; ses coordonnées internes partent de (0, 0).

### 9.1 Carte standard + texte

```svg
<g transform="translate(15, 42)">
  <rect rx="14" ry="14" class="card" width="185" height="62" />
  <text x="55" y="28" font-weight="bold" font-size="12" class="text-accent">Nom</text>
  <text x="55" y="44" class="subtitle">Description courte</text>
</g>
```

### 9.2 Capsule (étape de pipeline)

```svg
<g transform="translate(40, 55)">
  <rect rx="35" ry="35" class="pill" width="160" height="70" />
  <text x="80" y="36" text-anchor="middle" class="title">1. Ingestion</text>
  <text x="80" y="52" text-anchor="middle" class="subtitle">API &amp; Webhooks</text>
</g>
```
Variante mise en avant : `class="pill-accent"`.

### 9.3 Conteneur pointillé (groupe)

```svg
<g transform="translate(30, 60)">
  <rect rx="22" ry="22" class="container-dashed" width="215" height="370" />
  <text x="107" y="26" text-anchor="middle" class="title halo">Titre du groupe</text>
  <!-- cartes enfants à l'intérieur, décalées de 15px -->
</g>
```

### 9.4 Carte de palier à header découpé + pastille numérotée

Le header est un `path` dont les deux coins hauts sont arrondis et le bas droit. Pour une carte de largeur `W`, rayon `r` et hauteur de header `H` :

`M 0,r A r,r 0 0,1 r,0 L (W−r),0 A r,r 0 0,1 W,r L W,H L 0,H Z`

Exemple W=155, r=18, H=50 :

```svg
<g transform="translate(305, 105)">
  <rect rx="18" ry="18" class="card-bronze" width="155" height="270" />
  <path d="M 0,18 A 18,18 0 0,1 18,0 L 137,0 A 18,18 0 0,1 155,18 L 155,50 L 0,50 Z" fill="#a87954" />
  <circle cx="28" cy="25" r="11" fill="#ffffff" />
  <text x="28" y="29" text-anchor="middle" font-weight="bold" font-size="10" class="t-bronze-2">01</text>
  <text x="92" y="30" text-anchor="middle" font-weight="bold" font-size="13" class="text-white">BRONZE</text>

  <text x="77" y="78" text-anchor="middle" font-weight="bold" font-size="13" class="t-bronze-1">Titre du contenu</text>
  <text x="77" y="96" text-anchor="middle" font-size="11" class="t-bronze-2">Sous-titre</text>
  <line x1="20" y1="115" x2="135" y2="115" stroke="#d9beab" stroke-width="1.2" stroke-dasharray="3 3" />
  <text x="77" y="145" text-anchor="middle" font-size="11" class="t-bronze-1">• Puce 1</text>
  <text x="77" y="172" text-anchor="middle" font-size="11" class="t-bronze-1">• Puce 2</text>
</g>
```

Règles : texte du bandeau en blanc (`class="text-white"` ou `fill="#ffffff"`) ; pastille blanche r=11 (r=12 pour la carte agrandie) à `cx=28`, `cy` = milieu du bandeau ; puces espacées de 27px (28px pour la carte agrandie) ; séparateur pointillé en `stroke` (attribut) de la teinte claire du palier ; les pointillés `stroke-dasharray="3 3"` sont des attributs sur une `<line>` sans classe : ils fonctionnent.

### 9.5 Label de flèche entre deux paliers

```svg
<g transform="translate(460, 225)">
  <path d="M 4,20 L 74,20" stroke="#a87954" stroke-width="2" fill="none" stroke-linecap="round" marker-end="url(#arrow-bronze)" />
  <text x="39" y="10" text-anchor="middle" font-size="10" font-weight="600" class="t-link-bronze halo-thin">Nettoyage &amp;</text>
  <text x="39" y="38" text-anchor="middle" font-size="10" font-weight="600" class="t-link-bronze halo-thin">validation</text>
</g>
```
Un label au-dessus de la flèche (`y = y_flèche − 10`), le second en dessous (`y = y_flèche + 18`). Largeur du label ≤ longueur de la flèche + 10px.

### 9.6 Badge (étiquette de dimension / valeur)

```svg
<rect rx="6" ry="6" class="badge" x="40" y="-14" width="70" height="18" />
<text x="75" y="0" text-anchor="middle" class="badge-text">(B, T, D)</text>
```
Largeur du badge ≈ `nb_caractères × 6,6 + 14` (monospace 11px).

### 9.7 Nœud circulaire

```svg
<circle cx="90" cy="62" r="22" class="node" />
<text x="90" y="66" text-anchor="middle" font-weight="bold" font-size="10" class="text-accent">s_t</text>
```
Rayon ≥ `largeur du texte / 2 + 4`.

### 9.8 Nœud de décision (losange) et étape (hexagone)

Losange blanc, contour ardoise, point central (centre en (0,0), demi-diagonale 18) :

```svg
<g transform="translate(40, 120)">
  <polygon points="0,-18 18,0 0,18 -18,0" fill="#ffffff" stroke="#5a7d8c" stroke-width="1.8" stroke-linejoin="round" />
  <circle cx="0" cy="0" r="4" fill="#5a7d8c" />
</g>
```

Hexagone or (centre en (0,0), rayon 17) :

```svg
<g transform="translate(120, 120)">
  <polygon points="0,-17 14.7,-8.5 14.7,8.5 0,17 -14.7,8.5 -14.7,-8.5" fill="#fcf8ef" stroke="#cba258" stroke-width="1.8" stroke-linejoin="round" />
  <circle cx="0" cy="0" r="4" fill="#cba258" />
</g>
```
Ces formes se placent **sur** une flèche (au milieu du lien) avec le label au-dessus et en dessous.

### 9.9 Ligne guide en arrière-plan

```svg
<line x1="60" y1="90" x2="660" y2="90" class="grid-line" />
```
À dessiner **avant** les éléments pour qu'elle passe derrière.

### 9.10 Point de bifurcation

```svg
<circle cx="300" cy="335" r="4" fill="#286044" />
```

---

## 10. Catalogue d'icônes dessinées à la main

Toutes sont placées dans la carte à `translate(14…16, …)`, taille ≈ 26–30px, traits `#286044`, fonds `#ffffff` / `#d4e9e2`. Les attributs `fill`/`stroke` sur ces formes (sans classe) fonctionnent.

**Base de données (pile de 3 disques)**, 26×30, à `translate(14, 12)` dans une carte de 62px :

```svg
<g transform="translate(14, 12)">
  <path d="M 0,22 A 13,5 0 0,0 26,22 L 26,30 A 13,5 0 0,1 0,30 Z" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
  <path d="M 0,14 A 13,5 0 0,0 26,14 L 26,22 A 13,5 0 0,1 0,22 Z" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
  <ellipse cx="13" cy="8" rx="13" ry="5" fill="#286044" />
  <path d="M 0,8 A 13,5 0 0,0 26,8 L 26,14 A 13,5 0 0,1 0,14 Z" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
</g>
```

**Document à coin plié**, 26×32, à `translate(16, 15)` :

```svg
<g transform="translate(16, 15)">
  <polygon points="0,0 18,0 26,8 26,32 0,32" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
  <polygon points="18,0 18,8 26,8" fill="#ffffff" stroke="#286044" stroke-width="1.2" />
  <line x1="5" y1="14" x2="18" y2="14" stroke="#286044" stroke-width="1" />
  <line x1="5" y1="20" x2="20" y2="20" stroke="#286044" stroke-width="1" />
  <line x1="5" y1="26" x2="14" y2="26" stroke="#286044" stroke-width="1" />
</g>
```

**Tuyau / flux de messages (capsule avec paquets)**, 30×16, à `translate(14, 20)` :

```svg
<g transform="translate(14, 20)">
  <rect x="0" y="4" width="30" height="16" rx="8" ry="8" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
  <circle cx="8" cy="12" r="3" fill="#286044" />
  <circle cx="15" cy="12" r="3" fill="#4a8a6e" />
  <circle cx="22" cy="12" r="3" fill="#10b981" />
</g>
```

**Cube isométrique (stockage objet)**, 26×26, à `translate(16, 17)` :

```svg
<g transform="translate(16, 17)">
  <polygon points="13,0 26,6 13,12 0,6" fill="#7fb39a" stroke="#286044" stroke-width="1.2" />
  <polygon points="0,6 13,12 13,26 0,20" fill="#d4e9e2" stroke="#286044" stroke-width="1.2" />
  <polygon points="13,12 26,6 26,20 13,26" fill="#4a8a6e" stroke="#286044" stroke-width="1.2" />
</g>
```
Règle de teinte du cube : face haute la plus claire des verts moyens (`#7fb39a`), face gauche pastel, face droite la plus sombre (`#4a8a6e`).

**Moniteur / tableau de bord** : la carte est un `<rect rx="8" class="pill">` de 56px de haut ; barres de graphique à `translate(14, 18)` ; pied sous la carte :

```svg
<g transform="translate(14, 18)">
  <rect x="0" y="14" width="5" height="12" rx="1.5" fill="#cba258" />
  <rect x="8" y="6" width="5" height="20" rx="1.5" fill="#4a8a6e" />
  <rect x="16" y="0" width="5" height="26" rx="1.5" fill="#286044" />
</g>
<line x1="92" y1="56" x2="92" y2="62" stroke="#286044" stroke-width="2" />
<line x1="80" y1="62" x2="105" y2="62" stroke="#286044" stroke-width="2" stroke-linecap="round" />
```
(le pied est centré à `x = largeur de carte / 2`).

**Réseau de neurones / graphe (triangle de nœuds)**, à `translate(12, 16)` :

```svg
<g transform="translate(12, 16)">
  <line x1="4" y1="6" x2="20" y2="12" stroke="#7fb39a" stroke-width="1.2" />
  <line x1="4" y1="18" x2="20" y2="12" stroke="#7fb39a" stroke-width="1.2" />
  <circle cx="4" cy="6" r="3.5" fill="#286044" />
  <circle cx="4" cy="18" r="3.5" fill="#286044" />
  <circle cx="20" cy="12" r="4" fill="#10b981" />
</g>
```

**Prise / connecteur / point d'accès (cible)**, à `translate(14, 20)` :

```svg
<g transform="translate(14, 20)">
  <circle cx="10" cy="8" r="7" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
  <circle cx="10" cy="8" r="3" fill="#286044" />
</g>
```

**Pastille « + »** (addition / résiduel), dans une capsule `.pill-accent` de 40px de haut :

```svg
<circle cx="28" cy="20" r="9" fill="#286044" />
<text x="28" y="24" text-anchor="middle" font-weight="bold" font-size="12" fill="#ffffff">+</text>
```

**Recette pour inventer une nouvelle icône** : tenir dans ~26–30px ; 2 à 3 teintes seulement (blanc, pastel `#d4e9e2`, verts `#4a8a6e`/`#286044`, + un accent `#10b981` ou `#cba258`) ; contours 1.2–1.4px, détails internes 1px ; formes simples (ellipses, polygones, lignes), jamais de dégradé ; un seul point d'accent de couleur vive par icône.

---

## 11. Patrons complets

Chaque patron donne le SVG et son JSON. Tous ont été rendus avec le vrai moteur et sont lisibles sur fond clair et sombre.

### 11.1 Pipeline linéaire par étapes

Pour : ETL, CI/CD, parcours utilisateur, algorithme séquentiel. Capsules, ligne guide, flèches droites.

```svg
<svg width="720" height="180" viewBox="0 0 720 180" xmlns="http://www.w3.org/2000/svg">
  <line x1="60" y1="90" x2="660" y2="90" class="grid-line" />

  <g transform="translate(40, 55)">
    <rect rx="35" ry="35" class="pill" width="160" height="70" />
    <text x="80" y="36" text-anchor="middle" class="title">1. Ingestion</text>
    <text x="80" y="52" text-anchor="middle" class="subtitle">API &amp; Webhooks</text>
  </g>

  <line x1="205" y1="90" x2="265" y2="90" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(270, 55)">
    <rect rx="35" ry="35" class="pill-accent" width="180" height="70" />
    <text x="90" y="36" text-anchor="middle" class="title">2. Transformation</text>
    <text x="90" y="52" text-anchor="middle" class="subtitle">Nettoyage &amp; Validation</text>
  </g>

  <line x1="455" y1="90" x2="515" y2="90" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(520, 55)">
    <rect rx="35" ry="35" class="pill" width="160" height="70" />
    <text x="80" y="36" text-anchor="middle" class="title">3. Destination</text>
    <text x="80" y="52" text-anchor="middle" class="subtitle">PostgreSQL / Gold</text>
  </g>
</svg>
```

```json
{ "width": 720, "height": 180, "zoom": 2 }
```

Remarque : le sous-titre « Nettoyage & Validation » (23 car. à 11px ≈ 140px) tient dans la capsule de 180px ; vérifier chaque libellé avec la formule du §7.2.

### 11.2 Architecture simple multi-sources (version compacte)

Pour : data lake, microservices, architecture hexagonale, systèmes distribués. Version minimale : sources → 3 paliers avec badge de nom.

```svg
<svg width="900" height="380" viewBox="0 0 900 380" xmlns="http://www.w3.org/2000/svg">
  <text x="450" y="32" text-anchor="middle" class="title halo" font-size="16">Architecture Data Lakehouse</text>

  <g transform="translate(30, 60)">
    <rect rx="16" ry="16" class="container-dashed" width="150" height="290" />
    <text x="75" y="24" text-anchor="middle" class="title halo">Sources</text>

    <g transform="translate(15, 45)">
      <rect rx="12" ry="12" class="card" width="120" height="48" />
      <text x="60" y="24" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">PostgreSQL</text>
      <text x="60" y="38" text-anchor="middle" class="subtitle">OLTP Tables</text>
    </g>
    <g transform="translate(15, 115)">
      <rect rx="12" ry="12" class="card" width="120" height="48" />
      <text x="60" y="24" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">Kafka Stream</text>
      <text x="60" y="38" text-anchor="middle" class="subtitle">Événements JSON</text>
    </g>
    <g transform="translate(15, 185)">
      <rect rx="12" ry="12" class="card" width="120" height="48" />
      <text x="60" y="24" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">Fichiers S3</text>
      <text x="60" y="38" text-anchor="middle" class="subtitle">Parquet &amp; CSV</text>
    </g>
  </g>

  <path d="M 165,129 C 195,129 195,205 220,205" class="arrow" marker-end="url(#arrow)" />
  <path d="M 165,199 C 195,199 195,205 220,205" class="arrow" marker-end="url(#arrow)" />
  <path d="M 165,269 C 195,269 195,205 220,205" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(225, 95)">
    <rect rx="16" ry="16" class="card" width="140" height="220" />
    <rect rx="10" ry="10" class="pill-soft" x="25" y="15" width="90" height="22" />
    <text x="70" y="30" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">BRONZE</text>
    <text x="70" y="65" text-anchor="middle" class="title">Brut (Raw)</text>
    <line x1="20" y1="85" x2="120" y2="85" class="grid-line" />
    <text x="70" y="115" text-anchor="middle" class="subtitle">• Immuable</text>
    <text x="70" y="140" text-anchor="middle" class="subtitle">• Append-only</text>
    <text x="70" y="165" text-anchor="middle" class="subtitle">• Format source</text>
  </g>

  <line x1="370" y1="205" x2="420" y2="205" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(425, 95)">
    <rect rx="16" ry="16" class="card" width="140" height="220" />
    <rect rx="10" ry="10" class="pill-soft" x="25" y="15" width="90" height="22" />
    <text x="70" y="30" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">SILVER</text>
    <text x="70" y="65" text-anchor="middle" class="title">Épuré (Clean)</text>
    <line x1="20" y1="85" x2="120" y2="85" class="grid-line" />
    <text x="70" y="115" text-anchor="middle" class="subtitle">• Dédupliqué</text>
    <text x="70" y="140" text-anchor="middle" class="subtitle">• Typage strict</text>
    <text x="70" y="165" text-anchor="middle" class="subtitle">• Tables jointes</text>
  </g>

  <line x1="570" y1="205" x2="620" y2="205" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(625, 85)">
    <rect rx="18" ry="18" class="card" width="150" height="240" style="stroke-width:2.2px" />
    <rect rx="12" ry="12" class="pill-accent" x="25" y="15" width="100" height="26" />
    <text x="75" y="32" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">GOLD</text>
    <text x="75" y="68" text-anchor="middle" class="title">Métier (Star)</text>
    <line x1="20" y1="90" x2="130" y2="90" class="grid-line" />
    <text x="75" y="120" text-anchor="middle" class="subtitle">• Modèles en étoile</text>
    <text x="75" y="145" text-anchor="middle" class="subtitle">• KPI agrégés</text>
    <text x="75" y="170" text-anchor="middle" class="subtitle">• Prêt pour BI/ML</text>
  </g>
</svg>
```

```json
{ "width": 900, "height": 380, "zoom": 2 }
```

### 11.3 Architecture médaillon complète (sources → Bronze → Silver → Gold → consommateurs)

Version riche, rendue et vérifiée : paliers colorés à header découpé, icônes, marqueurs bronze/argent/or, consommateurs. Dimensions : 1200×460. Les classes de paliers sont définies dans le `<style>` interne (aucun JSON requis pour les couleurs).

```svg
<svg width="1200" height="460" viewBox="0 0 1200 460" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow-bronze" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
      <path d="M 0 0.5 L 7.5 3 L 0 5.5 Z" fill="#a87954" />
    </marker>
  </defs>
  <style>
    .card-bronze { fill: #fbf6f1; stroke: #a87954; stroke-width: 1.8px; }
    .card-silver { fill: #f3f7f9; stroke: #5a7d8c; stroke-width: 1.8px; }
    .card-gold   { fill: #fcf8ef; stroke: #cba258; stroke-width: 2.2px; }
    .t-bronze-1 { fill: #784f32; } .t-bronze-2 { fill: #a87954; }
    .t-silver-1 { fill: #3c5966; } .t-silver-2 { fill: #5a7d8c; }
    .t-gold-1   { fill: #8c6b2d; } .t-gold-2   { fill: #b08a3c; }
    .t-link-bronze { fill: #8f6542; } .t-link-gold { fill: #9a7430; }
    .halo-thin { paint-order: stroke fill; stroke: #ffffff; stroke-width: 1px; stroke-linejoin: round; }
  </style>

  <text x="600" y="32" text-anchor="middle" class="title halo" font-size="16">Architecture Data Lakehouse : des sources multiples à la donnée Gold</text>

  <!-- SOURCES -->
  <g transform="translate(30, 60)">
    <rect rx="22" ry="22" class="container-dashed" width="215" height="370" />
    <text x="107" y="26" text-anchor="middle" class="title halo">Sources hétérogènes</text>

    <g transform="translate(15, 42)">
      <rect rx="14" ry="14" class="card" width="185" height="62" />
      <g transform="translate(14, 12)">
        <path d="M 0,22 A 13,5 0 0,0 26,22 L 26,30 A 13,5 0 0,1 0,30 Z" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
        <path d="M 0,14 A 13,5 0 0,0 26,14 L 26,22 A 13,5 0 0,1 0,22 Z" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
        <ellipse cx="13" cy="8" rx="13" ry="5" fill="#286044" />
        <path d="M 0,8 A 13,5 0 0,0 26,8 L 26,14 A 13,5 0 0,1 0,14 Z" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
      </g>
      <text x="55" y="28" font-weight="bold" font-size="12" class="text-accent">PostgreSQL</text>
      <text x="55" y="44" class="subtitle">Tables relationnelles</text>
    </g>

    <g transform="translate(15, 120)">
      <rect rx="14" ry="14" class="card" width="185" height="62" />
      <g transform="translate(16, 15)">
        <polygon points="0,0 18,0 26,8 26,32 0,32" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
        <polygon points="18,0 18,8 26,8" fill="#ffffff" stroke="#286044" stroke-width="1.2" />
        <line x1="5" y1="14" x2="18" y2="14" stroke="#286044" stroke-width="1" />
        <line x1="5" y1="20" x2="20" y2="20" stroke="#286044" stroke-width="1" />
        <line x1="5" y1="26" x2="14" y2="26" stroke="#286044" stroke-width="1" />
      </g>
      <text x="55" y="28" font-weight="bold" font-size="12" class="text-accent">MongoDB</text>
      <text x="55" y="44" class="subtitle">Collections NoSQL</text>
    </g>

    <g transform="translate(15, 198)">
      <rect rx="14" ry="14" class="card" width="185" height="62" />
      <g transform="translate(14, 20)">
        <rect x="0" y="4" width="30" height="16" rx="8" ry="8" fill="#d4e9e2" stroke="#286044" stroke-width="1.4" />
        <circle cx="8" cy="12" r="3" fill="#286044" />
        <circle cx="15" cy="12" r="3" fill="#4a8a6e" />
        <circle cx="22" cy="12" r="3" fill="#10b981" />
      </g>
      <text x="55" y="28" font-weight="bold" font-size="12" class="text-accent">Kafka</text>
      <text x="55" y="44" class="subtitle">Messages &amp; CDC</text>
    </g>

    <g transform="translate(15, 276)">
      <rect rx="14" ry="14" class="card" width="185" height="62" />
      <g transform="translate(16, 17)">
        <polygon points="13,0 26,6 13,12 0,6" fill="#7fb39a" stroke="#286044" stroke-width="1.2" />
        <polygon points="0,6 13,12 13,26 0,20" fill="#d4e9e2" stroke="#286044" stroke-width="1.2" />
        <polygon points="13,12 26,6 26,20 13,26" fill="#4a8a6e" stroke="#286044" stroke-width="1.2" />
      </g>
      <text x="55" y="28" font-weight="bold" font-size="12" class="text-accent">Stockage S3</text>
      <text x="55" y="44" class="subtitle">Parquet, Avro &amp; CSV</text>
    </g>
  </g>

  <!-- Convergence vers Bronze -->
  <path d="M 245,133 C 272,133 272,245 300,245" class="arrow" marker-end="url(#arrow)" />
  <path d="M 245,211 C 270,211 272,245 300,245" class="arrow" marker-end="url(#arrow)" />
  <path d="M 245,289 C 270,289 272,245 300,245" class="arrow" marker-end="url(#arrow)" />
  <path d="M 245,367 C 272,367 272,245 300,245" class="arrow" marker-end="url(#arrow)" />

  <!-- BRONZE -->
  <g transform="translate(305, 105)">
    <rect rx="18" ry="18" class="card-bronze" width="155" height="270" />
    <path d="M 0,18 A 18,18 0 0,1 18,0 L 137,0 A 18,18 0 0,1 155,18 L 155,50 L 0,50 Z" fill="#a87954" />
    <circle cx="28" cy="25" r="11" fill="#ffffff" />
    <text x="28" y="29" text-anchor="middle" font-weight="bold" font-size="10" class="t-bronze-2">01</text>
    <text x="92" y="30" text-anchor="middle" font-weight="bold" font-size="13" class="text-white">BRONZE</text>
    <text x="77" y="78" text-anchor="middle" font-weight="bold" font-size="13" class="t-bronze-1">Données brutes</text>
    <text x="77" y="96" text-anchor="middle" font-size="11" class="t-bronze-2">Ingestion source</text>
    <line x1="20" y1="115" x2="135" y2="115" stroke="#d9beab" stroke-width="1.2" stroke-dasharray="3 3" />
    <text x="77" y="145" text-anchor="middle" font-size="11" class="t-bronze-1">• Format brut natif</text>
    <text x="77" y="172" text-anchor="middle" font-size="11" class="t-bronze-1">• Immuable / append</text>
    <text x="77" y="199" text-anchor="middle" font-size="11" class="t-bronze-1">• Historique complet</text>
    <text x="77" y="226" text-anchor="middle" font-size="11" class="t-bronze-1">• Stockage illimité</text>
  </g>

  <g transform="translate(460, 225)">
    <path d="M 4,20 L 74,20" stroke="#a87954" stroke-width="2" fill="none" stroke-linecap="round" marker-end="url(#arrow-bronze)" />
    <text x="39" y="10" text-anchor="middle" font-size="10" font-weight="600" class="t-link-bronze halo-thin">Nettoyage &amp;</text>
    <text x="39" y="38" text-anchor="middle" font-size="10" font-weight="600" class="t-link-bronze halo-thin">validation</text>
  </g>

  <!-- SILVER -->
  <g transform="translate(540, 105)">
    <rect rx="18" ry="18" class="card-silver" width="155" height="270" />
    <path d="M 0,18 A 18,18 0 0,1 18,0 L 137,0 A 18,18 0 0,1 155,18 L 155,50 L 0,50 Z" fill="#5a7d8c" />
    <circle cx="28" cy="25" r="11" fill="#ffffff" />
    <text x="28" y="29" text-anchor="middle" font-weight="bold" font-size="10" class="t-silver-2">02</text>
    <text x="92" y="30" text-anchor="middle" font-weight="bold" font-size="13" class="text-white">SILVER</text>
    <text x="77" y="78" text-anchor="middle" font-weight="bold" font-size="13" class="t-silver-1">Données épurées</text>
    <text x="77" y="96" text-anchor="middle" font-size="11" class="t-silver-2">Nettoyé &amp; enrichi</text>
    <line x1="20" y1="115" x2="135" y2="115" stroke="#b8ccd4" stroke-width="1.2" stroke-dasharray="3 3" />
    <text x="77" y="145" text-anchor="middle" font-size="11" class="t-silver-1">• Schémas uniformisés</text>
    <text x="77" y="172" text-anchor="middle" font-size="11" class="t-silver-1">• Données filtrées</text>
    <text x="77" y="199" text-anchor="middle" font-size="11" class="t-silver-1">• Tables consolidées</text>
    <text x="77" y="226" text-anchor="middle" font-size="11" class="t-silver-1">• Qualité vérifiée</text>
  </g>

  <g transform="translate(695, 225)">
    <path d="M 4,20 L 74,20" stroke="#cba258" stroke-width="2" fill="none" stroke-linecap="round" marker-end="url(#arrow-gold)" />
    <text x="39" y="10" text-anchor="middle" font-size="10" font-weight="600" class="t-link-gold halo-thin">Agrégation &amp;</text>
    <text x="39" y="38" text-anchor="middle" font-size="10" font-weight="600" class="t-link-gold halo-thin">modélisation</text>
  </g>

  <!-- GOLD (carte légèrement plus grande) -->
  <g transform="translate(775, 95)">
    <rect rx="20" ry="20" class="card-gold" width="165" height="290" />
    <path d="M 0,20 A 20,20 0 0,1 20,0 L 145,0 A 20,20 0 0,1 165,20 L 165,52 L 0,52 Z" fill="#cba258" />
    <circle cx="30" cy="26" r="12" fill="#ffffff" />
    <text x="30" y="30" text-anchor="middle" font-weight="bold" font-size="11" class="t-gold-2">03</text>
    <text x="98" y="31" text-anchor="middle" font-weight="bold" font-size="14" class="text-white">GOLD</text>
    <text x="82" y="82" text-anchor="middle" font-weight="bold" font-size="14" class="t-gold-1">Tables métier</text>
    <text x="82" y="100" text-anchor="middle" font-size="11" class="t-gold-2">Analytics &amp; décision</text>
    <line x1="20" y1="120" x2="145" y2="120" stroke="#dfc49d" stroke-width="1.4" stroke-dasharray="3 3" />
    <text x="82" y="152" text-anchor="middle" font-size="11" font-weight="600" class="t-gold-1">• Schémas en étoile</text>
    <text x="82" y="180" text-anchor="middle" font-size="11" font-weight="600" class="t-gold-1">• KPI précalculés</text>
    <text x="82" y="208" text-anchor="middle" font-size="11" font-weight="600" class="t-gold-1">• Requêtes sub-sec</text>
    <text x="82" y="236" text-anchor="middle" font-size="11" font-weight="600" class="t-gold-1">• Donnée certifiée</text>
  </g>

  <!-- Divergence vers les consommateurs -->
  <path d="M 940,240 C 962,240 962,160 985,160" class="arrow" style="stroke-width:2.2px" marker-end="url(#arrow)" />
  <path d="M 940,240 L 985,240" class="arrow" style="stroke-width:2.2px" marker-end="url(#arrow)" />
  <path d="M 940,240 C 962,240 962,320 985,320" class="arrow" style="stroke-width:2.2px" marker-end="url(#arrow)" />

  <!-- CONSOMMATEURS -->
  <g transform="translate(990, 95)">
    <g transform="translate(0, 30)">
      <rect rx="8" ry="8" class="pill" width="185" height="56" />
      <g transform="translate(14, 18)">
        <rect x="0" y="14" width="5" height="12" rx="1.5" fill="#cba258" />
        <rect x="8" y="6" width="5" height="20" rx="1.5" fill="#4a8a6e" />
        <rect x="16" y="0" width="5" height="26" rx="1.5" fill="#286044" />
      </g>
      <text x="48" y="26" font-weight="bold" font-size="11" class="text-accent">Tableau de bord</text>
      <text x="48" y="42" class="subtitle">Metabase / Superset</text>
      <line x1="92" y1="56" x2="92" y2="62" stroke="#286044" stroke-width="2" />
      <line x1="80" y1="62" x2="105" y2="62" stroke="#286044" stroke-width="2" stroke-linecap="round" />
    </g>

    <g transform="translate(0, 115)">
      <rect rx="14" ry="14" class="pill" width="185" height="56" />
      <g transform="translate(12, 16)">
        <line x1="4" y1="6" x2="20" y2="12" stroke="#7fb39a" stroke-width="1.2" />
        <line x1="4" y1="18" x2="20" y2="12" stroke="#7fb39a" stroke-width="1.2" />
        <circle cx="4" cy="6" r="3.5" fill="#286044" />
        <circle cx="4" cy="18" r="3.5" fill="#286044" />
        <circle cx="20" cy="12" r="4" fill="#10b981" />
      </g>
      <text x="48" y="26" font-weight="bold" font-size="11" class="text-accent">IA &amp; Machine Learning</text>
      <text x="48" y="42" class="subtitle">Feature store &amp; RAG</text>
    </g>

    <g transform="translate(0, 195)">
      <rect rx="14" ry="14" class="pill-accent" width="185" height="56" />
      <g transform="translate(14, 20)">
        <circle cx="10" cy="8" r="7" fill="#ffffff" stroke="#286044" stroke-width="1.4" />
        <circle cx="10" cy="8" r="3" fill="#286044" />
      </g>
      <text x="48" y="26" font-weight="bold" font-size="11" class="text-accent">Applications &amp; users</text>
      <text x="48" y="42" class="subtitle">API GraphQL &amp; REST</text>
    </g>
  </g>
</svg>
```

```json
{ "width": 1200, "height": 460, "zoom": 2 }
```

Mise en page de ce patron (à réutiliser pour toute architecture « sources → étages → consommateurs ») :

| Zone | x de départ | largeur | Remarque |
|---|---|---|---|
| Groupe sources | 30 | 215 | cartes de 185 de large, 62 de haut, pas vertical 78 |
| Flèches de convergence | 245 → 300 | 55 | arrivée 5px avant le palier |
| Palier 1 | 305 | 155 | y=105, h=270 |
| Flèche + label | 460 → 540 | 80 | label sur 2 lignes |
| Palier 2 | 540 | 155 | y=105, h=270 |
| Flèche + label | 695 → 775 | 80 | |
| Palier final | 775 | 165 | y=95, h=290 (plus grand) |
| Flèches de divergence | 940 → 985 | 45 | |
| Consommateurs | 990 | 185 | cartes de 56 de haut, pas vertical 80 |

### 11.4 Réseau neuronal avec connexion résiduelle (skip connection)

Pour : bloc Transformer, ResNet, encodeur-décodeur, module d'attention. Flux vertical du bas vers le haut, branche résiduelle en courbe pointillée.

```svg
<svg width="600" height="420" viewBox="0 0 600 420" xmlns="http://www.w3.org/2000/svg">
  <g transform="translate(300, 395)">
    <text x="0" y="0" text-anchor="middle" font-weight="bold" font-size="13" class="halo">Entrée X</text>
    <rect rx="6" ry="6" class="badge" x="40" y="-14" width="70" height="18" />
    <text x="75" y="0" text-anchor="middle" class="badge-text">(B, T, D)</text>
  </g>
  <line x1="300" y1="380" x2="300" y2="330" class="arrow" marker-end="url(#arrow)" />

  <circle cx="300" cy="335" r="4" fill="#286044" />

  <path d="M 300,335 C 100,335 100,165 205,165" class="arrow-dashed" marker-end="url(#arrow-soft)" />
  <text x="110" y="250" font-size="11" class="halo">Skip résiduel</text>

  <g transform="translate(180, 205)">
    <rect rx="16" ry="16" class="card" width="240" height="105" />
    <text x="120" y="28" text-anchor="middle" class="title">Self-Attention</text>
    <text x="120" y="44" text-anchor="middle" class="subtitle">Multi-Head (h=8)</text>

    <rect rx="14" ry="14" class="pill-soft" x="20" y="58" width="55" height="28" />
    <text x="47" y="76" text-anchor="middle" font-weight="bold" font-size="12" class="text-accent">Q</text>
    <rect rx="14" ry="14" class="pill-soft" x="92" y="58" width="55" height="28" />
    <text x="119" y="76" text-anchor="middle" font-weight="bold" font-size="12" class="text-accent">K</text>
    <rect rx="14" ry="14" class="pill-soft" x="165" y="58" width="55" height="28" />
    <text x="192" y="76" text-anchor="middle" font-weight="bold" font-size="12" class="text-accent">V</text>
  </g>

  <line x1="300" y1="205" x2="300" y2="185" class="arrow" marker-end="url(#arrow)" />

  <g transform="translate(210, 145)">
    <rect rx="20" ry="20" class="pill-accent" width="180" height="40" />
    <circle cx="28" cy="20" r="9" fill="#286044" />
    <text x="28" y="24" text-anchor="middle" font-weight="bold" font-size="12" fill="#ffffff">+</text>
    <text x="105" y="25" text-anchor="middle" font-weight="bold" font-size="12" class="text-accent">Add &amp; LayerNorm</text>
  </g>

  <line x1="300" y1="145" x2="300" y2="75" class="arrow" marker-end="url(#arrow)" />
  <g transform="translate(300, 50)">
    <text x="0" y="0" text-anchor="middle" font-weight="bold" font-size="13" class="halo">Sortie Z</text>
    <rect rx="6" ry="6" class="badge" x="35" y="-14" width="70" height="18" />
    <text x="70" y="0" text-anchor="middle" class="badge-text">(B, T, D)</text>
  </g>
</svg>
```

```json
{ "width": 600, "height": 420, "zoom": 2 }
```

Points d'attention : le badge est posé à droite du libellé (`x` du badge = demi-largeur du libellé + ~10px) ; la branche résiduelle contourne le bloc par la gauche et rejoint la capsule « Add » à `x=205` (juste avant son bord gauche à 210).

### 11.5 Boucle d'interaction et graphe d'états (agent ↔ environnement, MDP, RL)

Pour : apprentissage par renforcement, chaînes de Markov, automates, machines à états. Deux blocs empilés, deux boucles de retour courbes, nœuds d'états.

```svg
<svg width="650" height="360" viewBox="0 0 650 360" xmlns="http://www.w3.org/2000/svg">
  <text x="325" y="30" text-anchor="middle" class="title halo" font-size="15">Boucle d'interaction agent-environnement</text>

  <g transform="translate(165, 55)">
    <rect rx="18" ry="18" class="card" width="320" height="85" />
    <text x="160" y="30" text-anchor="middle" class="title">AGENT</text>
    <rect rx="12" ry="12" class="pill-soft" x="70" y="45" width="180" height="28" />
    <text x="160" y="63" text-anchor="middle" font-size="11" font-weight="bold" class="text-accent">Politique π(Action | État)</text>
  </g>

  <g transform="translate(165, 220)">
    <rect rx="18" ry="18" class="card" width="320" height="95" />
    <text x="160" y="28" text-anchor="middle" class="title">ENVIRONNEMENT</text>

    <circle cx="90" cy="62" r="24" class="node" />
    <text x="90" y="66" text-anchor="middle" font-weight="bold" font-size="11" class="text-accent">s_t</text>

    <circle cx="230" cy="62" r="24" class="node" />
    <text x="230" y="66" text-anchor="middle" font-weight="bold" font-size="10" class="text-accent">s_{t+1}</text>

    <path d="M 112,52 C 145,32 175,32 208,52" class="arrow" marker-end="url(#arrow)" />
  </g>

  <path d="M 485,97 C 590,97 590,267 485,267" class="arrow" style="stroke-width:2.5px" marker-end="url(#arrow)" />
  <g transform="translate(555, 180)">
    <rect rx="6" ry="6" class="badge" x="-50" y="-12" width="100" height="24" />
    <text x="0" y="4" text-anchor="middle" class="badge-text">Action a_t</text>
  </g>

  <path d="M 165,267 C 60,267 60,97 165,97" class="arrow" marker-end="url(#arrow)" />
  <g transform="translate(95, 180)">
    <rect rx="6" ry="6" class="badge" x="-45" y="-12" width="90" height="24" />
    <text x="0" y="4" text-anchor="middle" class="badge-text">État s_{t+1}</text>
  </g>
</svg>
```

```json
{ "width": 650, "height": 360, "zoom": 2 }
```

---

## 12. Choisir et composer un patron

| Besoin | Patron / composants |
|---|---|
| Étapes séquentielles (ETL, CI/CD, parcours) | 11.1 : capsules + flèches droites + ligne guide |
| Plusieurs entrées convergeant vers un système | 11.2 / 11.3 : conteneur pointillé + convergence en Bézier |
| Étages de maturité / qualité | 11.3 : cartes à header découpé et palette bronze/argent/or |
| Une entrée qui se distribue vers plusieurs sorties | 11.3 : divergence en Bézier vers des capsules |
| Réseau, bloc de modèle, tenseurs | 11.4 : carte + puces + badges de dimensions + skip pointillé |
| Interaction cyclique, états, transitions | 11.5 : deux blocs + boucles latérales + nœuds `.node` |
| Décision / étape sur un lien | 9.8 : losange / hexagone posé sur la flèche |
| Hiérarchie / regroupement | `container-dashed` autour de cartes (9.3) |

Principes de composition communs :
1. Flux **lisible d'un seul coup d'œil** : de gauche à droite (ou de bas en haut pour un empilement de couches).
2. **Un élément mis en avant** par schéma (palier final, étape clé) : plus grand, contour 2.2px, `pill-accent` ou couleur de palier.
3. **Cartes alignées sur une grille** (mêmes largeurs, pas vertical constant, marges de 30px).
4. 3 à 5 puces maximum par carte, libellés courts (≤ 20 caractères à 11px dans une carte de 150px).
5. Les flèches **ne se croisent pas** ; elles convergent ou divergent proprement.
6. Une icône par élément d'une même colonne, toutes à la même taille (~26–30px) et à la même position.

---

## 13. Procédure pour créer un nouveau schéma

1. Choisir un patron (§12), fixer la taille du canvas (largeur = marges + colonnes + écarts ; hauteur = titre + contenu + 30px de marge basse).
2. Écrire le SVG : balise `<svg width height viewBox xmlns>`, un seul élément racine, rien d'imbriqué.
3. Si des couleurs de palier ou des petites variantes sont nécessaires : bloc `<style>` interne (ou `extra_css`), en copiant depuis le §5.5.
4. Texte : classes `.title` / `.subtitle` / `.text-accent` / `.text-white`, ou classes `t-*` / `style="fill:…"`. **Jamais d'attribut `fill` sur `<text>` sauf `#ffffff`.**
5. Tout texte posé hors d'une carte reçoit `halo` (ou `halo-thin` pour ≤ 11px).
6. Vérifier les largeurs de texte avec le §7.2.
7. Créer le `.json` : `width`, `height` = taille du SVG, `zoom: 2`.
8. Générer, ouvrir le PNG **sur un fond blanc ET sur un fond vert nuit/noir** et contrôler.

### Checklist de validation

- [ ] Un seul `<svg>…</svg>`, aucun `<svg>` imbriqué, aucun `foreignObject`, aucun `var()`.
- [ ] Tous les `&` sont écrits `&amp;`.
- [ ] Chaque classe utilisée existe (§5) ou est définie dans `<style>` / `extra_css`.
- [ ] Aucune classe de forme (`card`, `pill`, `node`…) sur un `<g>`.
- [ ] Aucune forme sans fill (sinon noire).
- [ ] Aucun `fill=` sur un `<text>` (hors `#ffffff`) ; aucun `fill`/`stroke-width` en attribut sur un élément qui a une classe qui les définit (utiliser `style=`).
- [ ] `rx` et `ry` explicites sur tous les `<rect>` arrondis.
- [ ] Aucun texte ne dépasse de sa carte ni du canvas (formule §7.2, +15 % de marge).
- [ ] Tout texte hors carte a un halo ; les petits labels ont `halo-thin` et ≥ 10px.
- [ ] Flèches : marqueur adapté à la couleur, pointe 4–5px avant la cible, courbes de Bézier plutôt qu'angles droits.
- [ ] Aucun noir ni gris foncé ; palette du §5 uniquement.
- [ ] PNG contrôlé sur fond clair **et** fond sombre.
- [ ] `.json` présent avec `width`, `height` (+ `zoom: 2`).