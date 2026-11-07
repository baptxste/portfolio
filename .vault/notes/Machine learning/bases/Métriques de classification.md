---
title: Métriques de classification
tags:
  - classification
date: 2026-11-04
publish: true
---
# Avant propos
Dans l'ensemble de cette note on notera : 
- $VP$ les vrais positifs
- $VN$ les vrais négatifs
- $FP$ les faux positifs
- $FN$ les faux négatifs

## exemple : test de dépistage

Un modèle doit repérer les patients atteints d'une maladie. La classe « positive » est celle qu'on cherche à détecter (ici **malade**) : « positif » signifie « détecté par le modèle », pas « bon ».
Sur **1000 patients** dont **10 sont malades**, le modèle en signale 28 :

|                       | **Prédit malade**         | **Prédit sain**      |
| --------------------- | ------------------------- | -------------------- |
| **Réellement malade** | VP = 8                    | FN = 2 (cas manqués) |
| **Réellement sain**   | FP = 20 (fausses alertes) | VN = 970             |
Pour retenir : le **second mot** est ce que le modèle a prédit (positif / négatif), le **premier** dit si c'est juste (vrai) ou faux.

Les deux erreurs n'ont pas le même coût : un **FN** est grave (un malade n'est pas soigné), un **FP** est gênant mais rattrapable (examens complémentaires). C'est pourquoi un seul chiffre ne suffit pas, d'où les métriques suivantes.

> **Attention à ce que l'on entend par précision vs accuracy** : on parle souvent de la précision d'un modèle pour parler de sa performance, cependant en classification la précision diffère de l'accuracy. On parle d'accuracy pour définir l'exactitude du modèle, c'est à dire, le taux de classes correctement prédites : $accuracy = \frac{VP + VN}{ VP + VN + FP + FN}$, à ne pas confondre avec la précision qui donne le taux de vraies alertes parmi toutes les alertes levées : $précision = \frac{VP}{VP + FP}$ 

Le site de [google](https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall?hl=fr) permet de bien visualiser ces propriétés.
# Ne pas s'arrêter à l'accuracy
L'accuracy peu sembler être une bonne métrique pour un classifieur. En effet pour un modèle qui prédit tout correctement ($FP = FN = 0$) l'accuracy vaut 1 et 0 pour un modèle qui prédit tout faux, cependant cette métrique est incomplète sur les valeurs intermédiaires. 

En effet, si on reprend le cas du dépistage, dans le cas d'une répartition inégale des classes un modèle qui prédit négatif tout le temps obtient une accuracy de 0.97. On pense donc que notre modèle est globalement performant alors qu'il est purement constant. 

## Métriques et erreurs
Pour évaluer correctement la performance d'un classifieur il est important de s'appuyer sur plusieurs métriques : 
- **Accuracy**: $\frac{VP + VN}{ VP + VN + FP + FN}$, aussi appelé exactitude, la proportion de bonnes prédictions
- **Précision** : $\frac{VP}{VP + FP}$, parmi les alertes, combien sont justes
- **Rappel** (*recall*, sensibilité) : $\frac{VP}{VP + FN}$, parmi les vrais cas, combien sont détectés
- **Spécificité** : $\frac{VN}{VN + FP}$
- **Taux de faux positifs** : $\frac{FP}{FP + VN}$
Il existe aussi deux types d'erreurs : 
- type I : la fausse alerte -> les faux positifs
- type II : les ratés -> les vrais négatifs

Le choix de la métrique à optimiser dépend du problème à résoudre et de ces contraintes.
- si les faux positifs sont coûteux : utiliser le **taux de faux positifs**
- si les faux négatifs sont couteux : utiliser le **rappel**
- si on chercher à détecter tous les positifs : utiliser la **précision**

## Matrice de confusion
On utilise souvent une matrice de confusion pour avoir une première idée des performances du modèle. 

|                        | valeur prédite positive | valeur prédite négative |
| ---------------------- | ----------------------- | ----------------------- |
| valeur attendue : vrai | VP                      | FN                      |
| valeur attendue : faux | FP                      | VN                      |

## Score F1

Le score F1 est la moyenne harmonique de la précision et du rappel : 
$$
F1 = \frac{2 ~ rappel \times précision}{rappel + précision} = \frac{2VP}{2VP + FP + FN} $$
> la moyenne harmonique c'est l'inverse de la moyenne arithmétique des inverses: $H = \sum \frac{n}{1/x_i}$ (cf note [[Moyennes]])

Le score F1 permet d'équilibrer la précision et le rappel il est a privilégier à l'accuracy particulièrement dans le cas d'un déséquilibre des classes. De manière général le score F1 est similaire à la métrique la plus faible.

## Courbe ROC et AUC

La courbe ROC (Receiver Operating Characteristic) représente le taux de vrai positif en ordonnée et le taux de faux positif en abscisse, pour la tracer on fait varier le seuil de décision du modèle. Cette courbe permet de calculer l'AUC (area under the curve) qui représente la probabilité que le modèle classe correctement deux exemple aléatoires. (un AUC de 0.5 correspond donc à l'aléatoire) On cherche en général à maximiser l'AUC. 

> Cette courbe n'est valable que dans le cas d'une répartition correcte entre les classes, dans le cas contraire il est préférable de tracer la PRC (precision recall curve) avec la précision en y, le recall en x 

## Biais de classification

Regarder la fréquence d'une classe dans les données d'entraînement et la comparer à la fréquence de cette classe prédite par le modèle peut permettre de déceler un biais. La fréquence vu en entraînement doit être proche de la fréquence observée lors des prédiction du modèle. 

## Cas multiclasse

Avec $n$ classes, la matrice de confusion est une matrice $n \times n$. Les métriques ne se calculent plus pour le modèle entier mais **pour chaque classe**, en se ramenant au cas binaire « cette classe contre toutes les autres » (*one-vs-rest*). Pour une classe $k$ :
- $VP_k$ : la case diagonale $(k, k)$
- $FN_k$ : le reste de la **ligne** $k$ (exemples de la classe $k$ prédits dans une autre classe)
- $FP_k$ : le reste de la **colonne** $k$ (exemples d'autres classes prédits $k$)
- $VN_k$ : tout le reste

### Exemple

150 exemples répartis en 3 classes déséquilibrées :

|        | prédit A | prédit B | prédit C | total |
| ------ | -------- | -------- | -------- | ----- |
| vrai A | **90**   | 6        | 4        | 100   |
| vrai B | 8        | **20**   | 2        | 30    |
| vrai C | 6        | 4        | **10**   | 20    |

On en déduit les métriques de chaque classe, avec $F1_k = \frac{2VP_k}{2VP_k + FP_k + FN_k}$ :

| Classe | $VP$ | $FP$ | $FN$ | Précision | Rappel | F1   |
| ------ | ---- | ---- | ---- | --------- | ------ | ---- |
| A      | 90   | 14   | 10   | 0.87      | 0.90   | 0.88 |
| B      | 20   | 10   | 10   | 0.67      | 0.67   | 0.67 |
| C      | 10   | 6    | 10   | 0.63      | 0.50   | 0.56 |

L'accuracy vaut $\frac{90 + 20 + 10}{150} = 0.8$, mais la classe C est mal reconnue (un exemple sur deux est raté).

### Résumer en un seul nombre

Pour obtenir une valeur globale, on moyenne les scores des classes de trois façons :
- **Macro** : moyenne simple des scores par classe, toutes les classes comptent **à égalité**. Elle est sensible aux classes rares : $P_{macro} = \frac{1}{n}\sum_k P_k$
- **Pondérée** (*weighted*) : moyenne pondérée par le nombre d'exemples $N_k$ de chaque classe : $P_{pond} = \sum_k \frac{N_k}{N} P_k$
- **Micro** : on additionne d'abord les $VP$, $FP$ et $FN$ de toutes les classes, puis on calcule la métrique : $P_{micro} = \frac{\sum_k VP_k}{\sum_k VP_k + \sum_k FP_k}$

| Moyenne  | Précision | Rappel | F1   |
| -------- | --------- | ------ | ---- |
| Macro    | 0.72      | 0.69   | 0.70 |
| Pondérée | 0.79      | 0.80   | 0.80 |
| Micro    | 0.80      | 0.80   | 0.80 |

> La moyenne **micro** de la précision, du rappel et du F1 est toujours égale à l'**accuracy** (quand chaque exemple a une seule classe) : elle est dominée par les classes fréquentes et n'apporte rien de plus. La moyenne **macro** (0.70) est ici bien plus basse car elle met en évidence que la classe rare C est mal prédite, ce que l'accuracy cache.

# Sources
[Google course](https://developers.google.com/machine-learning/crash-course/classification?hl=fr)
[Scikit Learn](https://scikit-learn.org/stable/modules/model_evaluation.html)



