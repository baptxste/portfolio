# 📐 Guide de Création des Schémas (HTML / SVG → PNG)

Ce guide décrit l'architecture et le fonctionnement du générateur de schémas techniques intégré à `img_generator`.

---

## 🗂️ 1. Organisation des dossiers

Les schémas sont placés dans `img_generator/schemas/`.  
L'arborescence des sous-dossiers est automatiquement reproduite dans le dossier cible des assets de ton vault Obsidian :

```text
img_generator/schemas/
├── machine_learning/
│   ├── transformer_block.html    ───► .vault/notes/assets/machine_learning/transformer_block.png
│   └── transformer_block.json
└── maths/
    ├── markov_decision_process.html ──► .vault/notes/assets/maths/markov_decision_process.png
    └── markov_decision_process.json
```

---

## 🎨 2. Principes Graphiques : Flat, Doux & Arrondi

Pour rester en harmonie parfaite avec le site et éviter tout aspect criard :
1. **Zéro dégradé de couleur** : Uniquement des aplats solides, sobres et élégants.
2. **Formes très arrondies (pas d'angles droits rigides)** :
   - Coins généreux : `rx="16"` à `20`.
   - Formes capsules / pilules (`rx="9999"`, classe `.pill`).
   - Connecteurs fluides : courbes de Bézier continues (`d="M ... C ..."`) au lieu de virages cassants à 90°.
3. **Fond 100% transparent** : L'image se fond directement sur le thème clair ou sombre.
4. **Halo blanc** : Classe `.halo` sur tous les textes pour une lisibilité totale quel que soit le thème du site.

---

## 🛠️ 3. Palette & Classes CSS Disponibles

### Palette de couleurs (Aplats unis)
| Variable / Couleur | Code | Rôle |
|---|---|---|
| Vert sapin (Marque) | `#286044` | Titres, contours principaux, flèches |
| Vert moyen | `#4a8a6e` | Sous-titres, flux secondaires, badges |
| Vert doux | `#7fb39a` | Nœuds d'état, éléments intermédiaires |
| Vert clair uni | `#c3ddd2` | Fond doux uni des cartes (`.card`) |
| Vert grisé | `#a9c2b8` | Lignes de repères et pointillés |
| Liseré blanc | `#ffffff` | Halo protecteur anti-dark mode |

### Classes CSS système
- **`.card`** : Conteneur plat uni fond vert clair (`#c3ddd2`), contour vert sapin, coins très arrondis (`rx=18`).
- **`.card-accent`** : Conteneur plein vert sapin foncé (`#286044`) avec texte blanc.
- **`.card-soft`** : Boîte très sobre semi-transparente (`rx=16`).
- **`.pill`** : Capsule ultra-arrondie (`rx=9999`) idéale pour les composants et étapes.
- **`.pill-accent`** : Capsule pleine en vert foncé avec texte blanc.
- **`.badge`** + **`.badge-text`** : Étiquette avec texte monospace blanc (ex: dimensions de tenseurs `(B, T, D)`).
- **`.node`** : Cercle d'état pour automates ou graphes.
- **`.arrow`** : Connecteur vert sapin avec extrémités arrondies (`stroke-linecap: round`).
- **`.arrow-dashed`** : Trait pointillé doux pour rétroactions ou skip-connections.
- **`.title`** : Typographie de titre en gras épurée.
- **`.subtitle`** : Typographie secondaire plus fine.
- **`.halo`** : **Indispensable** — applique un contour blanc net derrière le texte pour la lisibilité Dark/Light mode.

### Définitions système (`<defs>`) déjà disponibles
- `marker-end="url(#arrow)"` : Flèche vert sapin avec pointe fine.
- `marker-end="url(#arrow-soft)"` : Flèche vert moyen.
- `marker-start="url(#dot)"` : Point d'ancrage rond.

---

## 🚀 4. Lancer la génération

Toutes les commandes passent par `uv` :

### Générer tous les schémas et scripts d'images :
```bash
cd img_generator
uv run python main.py
```

### Générer uniquement un schéma précis :
```bash
uv run python main.py --filter transformer_block
```
