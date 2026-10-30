---
title: Multi Layer Perceptron (MLP)
tags:
  - MLP
date: 2026-10-28
publish: false
---
Un perceptron multicouche est une forme basique de réseau de neurones. On peut le représenté comme cela: 
![[MLP.png]]Il est composé d'une couche d'entrée (input layer), de couches intermédiaires appelées couches cachées (hidden layers) dont le nombre peut varier, et d'une couche de sortie (output layer). Le nombre de neurones au sein d'une couche peut varier, en fonction des besoins du problèmes ou de la dimension des données à traiter.
Chaque couche est composée de neurones qui sont des fonction de la forme $a \cdot x+b$ . La couche d'entrée prend un vecteur initial, puis grâce au différents neurones calcul un premier vecteur dit caché, ce vecteur sert d'input pour la seconde couche etc jusqu'à la sortie.

# Première Approche
On note le vecteur d'entré $X = \begin{pmatrix} x_1 \\ x_2 \\ \vdots \\ x_n  \end{pmatrix} \in \mathbb{R}^n$, les fonctions des neurones ne sont donc plus scalaire mais vectorielles, le $a$ devient donc $w = \begin{pmatrix}w_1 \\ \vdots \\ w_n \end{pmatrix} \in \mathbb{R}^n$, ces $w$ sont appelés les poids (weight) du réseau, $b$ reste un scalaire appelé le biais. Chaque neurone effectue donc l'opération suivante : 
$$
y = X \cdot w +b = x_1w_1 + x_2w_2 + \dots + x_nw_n + b
$$
Etant donné que la composition de fonction affine reste une fonction affine il est nécessaire d'ajouter de la non linéatité dans réseau, pour cela on utilise des fonctions d'activation.
## Fonction d'activation 

Les plus 

