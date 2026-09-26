# 🎨 img-generator

Générateur automatique d'images et de graphiques pour les notes du Vault Obsidian.

## 🚀 Fonctionnement

`main.py` parcourt **récursivement** le dossier `images/`, exécute chaque script Python et enregistre l'image générée dans le dossier d'assets du vault Obsidian configuré dans `.env`.

La structure de sous-dossiers dans `images/` est préservée automatiquement dans le vault :
`images/maths/thm_centrale_limite.py` ➔ `<PATH_TO_VAULT>/<PATH_IN_VAULT>/maths/thm_centrale_limite.png`

---

## ⚙️ Configuration (`.env`)

Créez ou modifiez le fichier `.env` à la racine de `img_generator` :

```env
PATH_TO_VAULT=../.vault/notes
PATH_IN_VAULT=assets
```

- **`PATH_TO_VAULT`** : Chemin (relatif ou absolu) vers le dossier racine de vos notes Obsidian.
- **`PATH_IN_VAULT`** : Sous-dossier au sein du vault destiné à recevoir les images (ex: `assets`).

---

## 💻 Exécution

Pour exécuter tous les scripts d'images :

```bash
uv run python main.py
```

Options CLI disponibles :
```bash
# Filtrer pour n'exécuter qu'un script ou sous-dossier spécifique :
uv run python main.py --filter maths

# Spécifier un dossier d'arrivée personnalisé :
uv run python main.py --assets-dir /chemin/vers/mon/dossier
```

---

## 📁 Structure d'un script d'image

Pour ajouter une nouvelle image, créez simplement un fichier `.py` dans `images/` (ou un sous-dossier) en suivant cette structure standard :

```python
from styles import Colors, Fonts, styled_figure, save

def generate(output_path):
    fig, ax = styled_figure(figsize=(11, 7))

    # Votre code de tracé matplotlib...
    ax.plot([0, 1, 2], [0, 1, 4], color=Colors.CURVE)

    # Toujours retourner save(fig, output_path)
    return save(fig, output_path)

if __name__ == "__main__":
    generate("test_output.png")
```

- L'export standard `generate(output_path)` est automatiquement détecté et appelé par `main.py`.
- Les fichiers et dossiers commençant par `_` (ex: `_utils.py`) sont ignorés par le scanner récursif.
