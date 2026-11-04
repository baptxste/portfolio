---
title: Métriques de classification
tags:
  - classification
date: 2026-11-04
publish: false
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
- si les faux positifs sont coûteux : utiliser le taux de faux positifs
- si les faux négatifs sont couteux : utiliser le rappel
- si on chercher à détecter tous les positifs : utiliser la précision

## Matrice de confusion
On utilise souvent une matrice de confusion pour avoir une première idée des performances du modèle. 

|                        | valeur prédite positive | valeur prédite négative |
| ---------------------- | ----------------------- | ----------------------- |
| valeur attendue : vrai | VP                      | FN                      |
| valeur attendue : faux | FP                      | VN                      |
La matrice de précision peut être étendue à de la classification multi classes de la manière suivante : 

|        | prédit A | prédit B | prédit C |
| ------ | -------- | -------- | -------- |
| vrai A |          |          |          |
| vrai B |          |          |          |
| vrai C |          |          |          |
On cherche alors à avoir une diagonale maximale puisque chaque valeur hors diagonale correspond à une erreur du modèle. 

## Score F1

Le score F1 est la moyenne harmonique de la précision et du rappel : 
$$
F1 = \frac{2 ~ rappel \times précision}{rappel + précision}$$
> la moyenne harmonique c'est l'inverse de la moyenne arithmétique des inverses: $H = \sum \frac{n}{1/x_i}$
# Sources
[Google course](https://developers.google.com/machine-learning/crash-course/classification?hl=fr)
[Scikit Learn](https://scikit-learn.org/stable/modules/model_evaluation.html)

## 4. Le compromis précision / rappel
- Le seuil de décision (0,5 par défaut sur une sortie sigmoïde) : le monter réduit les FP et augmente les FN, le baisser fait l'inverse
- Quand privilégier le rappel (dépistage, sécurité) ou la précision (filtrage automatique, alertes coûteuses)

## 5. Score F1
- Moyenne **harmonique** : $F_1 = 2 \cdot \frac{\text{précision} \cdot \text{rappel}}{\text{précision} + \text{rappel}}$
- Pourquoi harmonique et pas arithmétique (pénalise les déséquilibres entre les deux)
- Généralisation $F_\beta$ : pondérer le rappel ou la précision
- Limites : ignore les VN, ne dit rien du coût réel des erreurs

## 6. Évaluer sans fixer de seuil
- Courbe ROC et AUC
- Courbe précision-rappel et AP : plus informative quand les positifs sont rares
- Quand préférer l'une ou l'autre

## 7. Cas multiclasse
- Métriques calculées par classe (une contre toutes)
- Moyennes **macro** (classes à égalité), **micro** (pondérée par les exemples), **pondérée**
- Matrice de confusion n × n

## 8. Choisir sa métrique selon le problème
- Coût d'un FP vs coût d'un FN (tableau de décision)
- Déséquilibre des classes
- Lien avec le surapprentissage : toujours évaluer sur le jeu de **test** (cf [[Réseaux de neurones - MLP]])

