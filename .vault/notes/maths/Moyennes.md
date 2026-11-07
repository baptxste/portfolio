---
title: Moyennes
tags:
  - average
date: 2026-11-04
publish: true
---
# Moyenne arithmétique

C'est la moyenne « de tous les jours » : la somme des valeurs divisée par leur nombre.

$$A = \frac{x_1 + x_2 + \dots + x_n}{n} = \frac{1}{n}\sum_{i=1}^{n} x_i$$

**Exemple.** Notes 12, 15 et 18 : A = (12 + 15 + 18) / 3 = 15.

**Exemple (manifestation).** La police annonce 10 000 manifestants, les organisateurs 100 000 : A = (10 000 + 100 000) / 2 = 55 000. Si la police n'en annonçait que 1 000, on aurait A = 50 500 : la moyenne suit l'écart entre les deux chiffres.

**Quand l'utiliser.** Pour des quantités qui s'additionnent et sont du même ordre : notes, températures, salaires, tailles. Elle est sensible aux valeurs extrêmes : un seul très grand nombre la tire vers le haut.

**Version pondérée.** Si chaque valeur a un coefficient wᵢ (comme les coefficients d'une matière), on utilise :

$$A_w = \frac{\sum_{i} w_i x_i}{\sum_{i} w_i}$$

# Moyenne géométrique

C'est la racine n-ième du produit des valeurs : le nombre qui, multiplié n fois par lui-même, redonne le même produit.

$$G = \sqrt[n]{x_1 \times x_2 \times \dots \times x_n} = \left(\prod_{i=1}^{n} x_i\right)^{1/n}$$

**Exemple.** Pour 2 et 8 : G = √(2 × 8) = √16 = 4 (alors que A = 5).

**Exemple (manifestation).** Avec les mêmes chiffres, G = √(10 000 × 100 000) = √10⁹ ≈ 31 623, très loin de 55 000. Si chaque camp triche par un même facteur k (la police annonce x / k, les organisateurs k × x), alors G = √(x / k × k × x) = x : la moyenne géométrique retrouve le vrai nombre x.

**Quand l'utiliser.** Pour des grandeurs qui se multiplient : taux de croissance, rendements successifs, indices, proportions. Exemple : un placement qui gagne +10 % puis +30 % a un taux annuel moyen de √(1,10 × 1,30) − 1 ≈ 19,6 %, et non 20 %.

**Remarque.** Elle exige des valeurs strictement positives, et le calcul se fait souvent avec les logarithmes :

$$\ln G = \frac{1}{n}\sum_{i=1}^{n} \ln x_i$$

# Moyenne harmonique

C'est l'inverse de la moyenne arithmétique des inverses.

$$H = \frac{n}{\dfrac{1}{x_1} + \dfrac{1}{x_2} + \dots + \dfrac{1}{x_n}}$$

**Exemple.** Un trajet aller à 40 km/h, retour à 60 km/h (même distance) : H = 2 / (1/40 + 1/60) = 48 km/h. La moyenne arithmétique (50 km/h) serait fausse, car on passe plus de temps à la vitesse la plus lente.

**Exemple (vélo).** Une côte de 1 km montée à 20 km/h puis descendue à 30 km/h : la vitesse moyenne n'est pas 25 km/h, mais H = 2 / (1/20 + 1/30) = 24 km/h.

**Quand l'utiliser.** Pour des rapports ou des taux calculés sur une même quantité au numérateur : vitesses sur des distances égales, prix par unité pour un budget fixe, résistances en parallèle, score F1 en apprentissage automatique.

**Remarque.** Elle est très sensible aux petites valeurs, et elle est nulle ou indéfinie si une valeur vaut 0.

# Comparaison

Pour des nombres strictement positifs, les trois moyennes sont toujours rangées dans le même ordre, avec égalité uniquement si toutes les valeurs sont identiques :

$$H \le G \le A$$

Avec 2 et 8 : H = 3,2 ≤ G = 4 ≤ A = 5. De plus, G² = A × H pour deux nombres.

|Moyenne|Formule|Données adaptées|Exemple d'usage|
|---|---|---|---|
|Arithmétique|(x₁ + … + xₙ) / n|Quantités additives|Notes, salaires|
|Géométrique|(x₁ × … × xₙ)^(1/n)|Facteurs multiplicatifs|Croissance, rendements|
|Harmonique|n / (1/x₁ + … + 1/xₙ)|Taux sur une quantité commune|Vitesses, prix unitaires|


# Sources
[Bibmath moyennes](https://www.bibmath.net/dico/index.php?action=affiche&quoi=./m/moyenne.html)