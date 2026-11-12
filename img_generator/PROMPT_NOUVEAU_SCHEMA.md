# 📋 Prompt / Template pour générer un schéma (Claude, ChatGPT, etc.)

> **Mode d'emploi** : Copie l'intégralité du bloc ci-dessous dans Claude, ChatGPT ou tout autre LLM, puis ajoute à la toute fin ta demande spécifique (ex: *"Crée-moi un schéma pour expliquer l'algorithme PageRank"*). L'IA te fournira directement les 2 fichiers (`.html` et `.json`) prêts à copier-coller dans ton projet.

---

```markdown
Tu es un expert en design graphique technique et en SVG épuré.
Je veux que tu conçoives un schéma technique pour mon portfolio.

### 🎯 RÈGLES DE STYLE STRICTES & MINIMALISTES
1. **ZÉRO DÉGRADÉ DE COULEUR** : N'utilise AUCUN gradient (pas de linearGradient, pas de radialGradient). Utilise uniquement des aplats de couleurs solides et sobres.
2. **FORMES TRÈS ARRONDIES & DOUCES (PAS D'ANGLES DROITS)** :
   - Évite au maximum les angles droits stricts (90°) quand ce n'est pas nécessaire.
   - Les boîtes doivent avoir de grands arrondis : `rx="16"` ou `rx="18"`.
   - Privilégie la forme capsule / pilule : `<rect class="pill" ... />` ou `rx="9999"`.
   - Pour les connecteurs et flux : privilégie des courbes de Bézier douces et fluides (`d="M x1,y1 C cx1,cy1 cx2,cy2 x2,y2"`) plutôt que des virages orthogonaux cassants à 90°.
3. **FOND 100% TRANSPARENT** : Ne dessine aucun grand rectangle de fond.
4. **HALO BLANC OBLIGATOIRE** : Mets impérativement `class="halo"` sur tous les textes sombres pour qu'ils ressortent aussi bien sur fond blanc que sur fond sombre.

### 🎨 PALETTE DE COULEURS DU SITE (Aplats solides uniquement)
- **Vert sapin (Marque / Accent)** : `#286044` (traits, flèches, textes principaux).
- **Vert moyen** : `#4a8a6e` (sous-titres, badges, flux secondaires).
- **Vert doux** : `#7fb39a` (états intermédiaires, cercles).
- **Vert très clair (Fond doux)** : `#c3ddd2` (remplissage sobre des cartes).
- **Vert grisé (Grille)** : `#a9c2b8` (lignes de structure en pointillés).
- **Blanc** : `#ffffff` (textes dans les blocs foncés, halo).

### 🛠️ CLASSES CSS DÉJÀ DISPONIBLES (Ne pas les réécrire)
- `.card` : Boîte sobre à fond vert clair uni (`#c3ddd2`), contour vert sapin, coins très arrondis (`rx=18`).
- `.card-accent` : Boîte en aplat vert sapin foncé (`#286044`) pour l'élément central à mettre en avant.
- `.card-soft` : Boîte très légère translucide (`rx=16`).
- `.pill` : Capsule ultra-arrondie (`rx=9999`) idéale pour les étapes, fonctions ou composants.
- `.pill-accent` : Capsule pleine en vert foncé avec texte blanc.
- `.badge` + `.badge-text` : Petit badge arrondi avec texte blanc monospace (dimensions de tenseurs, types, tags).
- `.node` : Cercle d'état pour automates ou graphes.
- `.arrow` : Trait fluide vert avec bouts arrondis (`stroke-linecap: round`).
- `.arrow-dashed` : Trait pointillé doux pour rétroactions ou raccourcis résiduels.
- `.title` : Titre en gras épuré.
- `.subtitle` : Sous-titre explicatif plus discret.
- `.halo` : Liseré blanc protecteur pour lisibilité Dark & Light mode.

### 📐 DÉFINITIONS SYSTÈME PRÊTES À L'EMPLOI
- `marker-end="url(#arrow)"` : Flèche vert sapin avec pointe fine.
- `marker-end="url(#arrow-soft)"` : Flèche vert moyen pour flux secondaires.
- `marker-start="url(#dot)"` : Point d'ancrage rond.

---

### 📦 FORMAT DU FICHIER JSON ATTENDU
```json
{
  "width": 800,
  "height": 450
}
```

---

### 📌 EXEMPLE MINIMAL ATTENDU

#### `mon_schema.html`
```html
<svg width="650" height="180" viewBox="0 0 650 180" xmlns="http://www.w3.org/2000/svg">
  <!-- Ligne de fond douce -->
  <line x1="60" y1="90" x2="590" y2="90" class="grid-line" />

  <!-- Étape 1 -->
  <g transform="translate(40, 50)">
    <rect class="pill" width="150" height="80" />
    <text x="75" y="42" text-anchor="middle" class="title halo">Entrée</text>
    <text x="75" y="58" text-anchor="middle" class="subtitle">Dataset brut</text>
  </g>

  <!-- Flèche fluide -->
  <line x1="195" y1="90" x2="245" y2="90" class="arrow" marker-end="url(#arrow)" />

  <!-- Étape 2 (Accent) -->
  <g transform="translate(250, 50)">
    <rect class="pill-accent" width="160" height="80" />
    <text x="80" y="42" text-anchor="middle" class="title" fill="#ffffff">Traitement</text>
    <text x="80" y="58" text-anchor="middle" font-size="11" fill="#c3ddd2">Optimisation AdamW</text>
  </g>

  <!-- Flèche fluide -->
  <line x1="415" y1="90" x2="465" y2="90" class="arrow" marker-end="url(#arrow)" />

  <!-- Étape 3 -->
  <g transform="translate(470, 50)">
    <rect class="pill" width="150" height="80" />
    <text x="75" y="42" text-anchor="middle" class="title halo">Sortie</text>
    <text x="75" y="58" text-anchor="middle" class="subtitle">Inférence</text>
  </g>
</svg>
```

#### `mon_schema.json`
```json
{
  "width": 650,
  "height": 180
}
```

---

### 🚀 MA DEMANDE DE SCHÉMA :
[Décris ici ce que doit représenter le schéma, ses étapes, ses formules, etc.]
```
