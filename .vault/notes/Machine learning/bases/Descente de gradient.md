---
title: Descente de gradient
tags:
  - gradient
date: 2026-09-21
publish: false
---


# Descente de gradient 
## 1. Problème à résoudre -
Minimiser une fonction de perte $L(\theta)$ par rapport aux paramètres $\theta$ (poids et biais), cf [[Fonctions d'erreur]] - Pourquoi on ne peut pas résoudre directement : trop de paramètres, pas de solution analytique (sauf cas simples comme la régression linéaire) 
## 2. Rappels mathématiques -
Dérivée, dérivée partielle - Gradient $\nabla_\theta L$ : vecteur des dérivées partielles - Propriété clé : le gradient pointe dans la direction de **plus forte croissance** de $L$ 
## 3. Règle de mise à jour -
$\theta \leftarrow \theta - \eta \, \nabla_\theta L(\theta)$ - Justification : développement de Taylor à l'ordre 1, $L(\theta + \delta) \approx L(\theta) + \nabla L \cdot \delta$, minimal pour $\delta$ opposé au gradient - Algorithme complet : initialiser, boucle (prédire, perte, gradient, mise à jour), arrêt 
## 4. Taux d'apprentissage $\eta$ 
- Trop grand : oscillations, divergence - Trop petit : convergence très lente - Planification du taux (*learning rate schedule*) 
## 5. Exemples -
1D : $L(w) = w^2$, calcul à la main sur quelques itérations - Régression linéaire avec MSE : gradient calculé explicitement - Figure : trajectoire sur des courbes de niveau 
## 6. Variantes selon les données utilisées - 
Batch (tout le jeu), stochastique (un point), mini-batch (un lot) - Vocabulaire : batch size, itération, epoch - Compromis stabilité / coût de calcul / bruit utile 
## 7. Améliorations - 
Momentum - RMSProp, Adam (taux adaptatif par paramètre) - Régularisation / *weight decay* (lien avec le sur-apprentissage) 
## 8. Lien avec la rétropropagation -
La descente de gradient *utilise* le gradient, la rétropropagation le *calcule* - Règle de la chaîne : $\frac{\partial L}{\partial w} = \frac{\partial L}{\partial \hat{y}} \cdot \frac{\partial \hat{y}}{\partial z} \cdot \frac{\partial z}{\partial w}$ - Renvoi vers [[Rétropropagation]] si tu en fais une note 
## 9. Difficultés 
- Perte non convexe : minima locaux, points selles, plateaux - Gradients qui disparaissent ou explosent (lien avec les activations, cf [[Fonctions d'activation]]) - Mauvais conditionnement (vallées étroites) 
- 
## 10. Convergence et critères d'arrêt - 
- Norme du gradient proche de 0, perte qui stagne, nombre max d'itérations - Suivi de la perte d'entraînement et de validation (arrêt anticipé) 

## 11. Implémentation - 
- Code minimal en numpy (régression linéaire) - Comparaison avec un optimiseur de bibliothèque# Descente de gradient 