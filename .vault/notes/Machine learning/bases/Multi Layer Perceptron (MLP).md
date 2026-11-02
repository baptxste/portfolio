---
title: Multi Layer Perceptron (MLP)
tags:
  - MLP
date: 2026-10-28
publish: true
---
Le but d'un perceptron ou d'un perceptron multicouche est d'apprendre à partir d'un jeu de données un modèle qui les représente. 
# Théorème d'approximation universel

Si un MLP est capable d'apprendre un modèle qui représente des données, c'est que, en théorie, il peut approcher n'importe quelle fonction. C'est ce qu'énonce le **théorème d'approximation universelle** (Cybenko 1989, Hornik 1991) : 

> Soit $f$ une fonction continue sur un compact $K \subset \mathbb{R}^n$ et $\varepsilon > 0$. Il existe un MLP à **une seule couche cachée**, avec assez de neurones $N$ et une activation $g$ non linéaire non polynomiale, tel que 

$$\sup_{x \in K} \left| f(x) - \sum_{j=1}^{N} v_j \, g(w_j \cdot x + b_j) \right| < \varepsilon$$
Autrement dit, un MLP peut approcher toute fonction continue avec une précision arbitraire.

**Intuition** : chaque neurone produit une petite « marche » (ou un segment, avec ReLU) décalée et mise à l'échelle par ses poids et son biais. En additionner beaucoup permet de reconstruire n'importe quelle courbe, comme des rectangles sous une courbe en intégration. 

**Limites** : c'est un théorème d'*existence*. Il ne dit pas : 
- combien de neurones sont nécessaires (potentiellement énormément) 
- que la descente de gradient trouvera les bons paramètres 
- que le réseau **généralisera** : un modèle qui colle parfaitement aux données d'entraînement peut sur-apprendre (*overfitting*).

# MLP

Un perceptron multicouche est une forme basique de réseau de neurones. On peut le représenté comme cela: 
![[MLP.png]]Il est composé d'une couche d'entrée (input layer), de couches intermédiaires appelées couches cachées (hidden layers) dont le nombre peut varier, et d'une couche de sortie (output layer). Le nombre de neurones au sein d'une couche peut varier, en fonction des besoins du problèmes ou de la dimension des données à traiter.
Chaque couche est composée de neurones qui sont des fonction de la forme $a \cdot x+b$ . La couche d'entrée prend un vecteur initial, puis grâce au différents neurones calcule un premier vecteur dit caché, ce vecteur sert d'input pour la seconde couche etc jusqu'à la sortie.

# Première Approche
On note le vecteur d'entré $X = \begin{pmatrix} x_1 \\ x_2 \\ \vdots \\ x_n  \end{pmatrix} \in \mathbb{R}^n$, les fonctions des neurones ne sont donc plus scalaire mais vectorielles, le $a$ devient donc $w = \begin{pmatrix}w_1 \\ \vdots \\ w_n \end{pmatrix} \in \mathbb{R}^n$, ces $w$ sont appelés les poids (weight) du réseau, $b$ reste un scalaire appelé le biais. Chaque neurone effectue donc l'opération suivante : 
$$
y = X \cdot w +b = x_1w_1 + x_2w_2 + \dots + x_nw_n + b
$$
Etant donné que les fonction affine sont stable par composition cela signifie que notre modèle est uniquement capable d'apprendre une fonction affine peut importe le nombre de couche qu'il possède. Pour sortir de cette limitation il est nécessaire d'ajouter de la non linéarité dans le réseau, pour cela on utilise des fonctions d'activation. (cf l'exemple de XOR dans le [MIT Deeplearning book](https://www.deeplearningbook.org/) page 167)

## Fonction d'activation 

Les plus répandues sont ReLu, Sigmoïde et tanh.

| Nom      | Formule                               | Domaine         | Usage                      |
| -------- | ------------------------------------- | --------------- | -------------------------- |
| ReLu     | $max(0,z)$                            | $[0, +\infty [$ | Défaut des couches cachées |
| Sigmoïde | $\sigma(z) = 1 / (1 + e^{-z})$        | $]0, 1[$        | Classification Binaire     |
| Tanh     | $tanh(z) = (e^z-e^{-z})/(e^z+e^{-z})$ | $]-1, 1[$       | Couches chachées           |
| ...      | ...                                   | ...             | ...                        |


![[fct_activation.png]]

Chaque couche neurone est suivit d'une activation ce qui permet de les noter $y = g(a \cdot x +b )$ où $g$ représente la fonction d'activation. 

Il existe de nombreux autres types de fonctions d'activations (cf notes: [[Fonctions d'activation]]).

# Entraînement 

Initialement les paramètres de notre réseau (les poids et les biais) sont aléatoire, et par conséquent la sortie l'est aussi. Pour que notre réseau soit utile il est nécessaire de l'entraîner sur une tâche, entraîner un réseau revient à modifier ses paramètres pour améliorer la performance sur une tâche.

Pour entraîner un réseau de neurones il faut un jeu de données annotées c'est à dire un ensemble de données contenant des entrées (notées x) à donner au réseau et des sorties attendu (notées y) , le but étant de faire apprendre ce motif là au réseau. Pour cela on a besoin d'une fonction erreur (ou Loss), cette fonction doit être capable de mesurer l'erreur entre la sortie produite par le réseau (notée $\hat{y}$ ) pour un entrée donnée par rapport à la sortie attendue.
## Fonction d'erreur


La fonction la plus répandue est l'erreur quadratique (MSE : mean square error) définie par :
$$L = 1/N \sum_{i=1}^N (y_i-\hat{y}_i)^2  = 1/N \sum^N_{i=1}(f(x_i, W, B) -y_i)^2$$
où $f$ représente la fonction apprise par le réseau paramétrée par les poids et biais $W$ et $B$. Comme la fonction erreur dépend des paramètres de notre modèle et qu'elle est différentiable on peut donc calculer le gradient par rapport aux paramètres $\nabla_{\mathbf{w}} L = \frac{\partial L}{\partial \mathbf{w}}$. Plus de détails ici: [[Fonctions d'erreur]]

## Train vs Test 

Lors de l'entraînement il est important séparer son dataset en un jeu d'entraînement et d'évaluation, on prend souvent une séparation 80/20 et les deux jeux doivent représenter correctement des données. 
Cette séparation est importante pour détecter le sur apprentissage (overfitting) ce qui signifie que le modèle apprend un jeu de donnée parfaitement mais perd toute capacité à généraliser sur de la nouvelle donnée. 
![[overfitting.png]]
> à gauche on peut voir un modèle qui ne minimise pas la perte sur le jeu de train ni de test mais qui est constant sur les deux -> modèle OK, à droite la perte est minimale sur le jeu de train mais très important en test -> modèle overfit

Le jeu de test n'est jamais utilisé pour calculer mettre à jour les paramètres du modèle mais uniquement pour l'évaluer. La séparation permet de calculer la loss sur le jeu de train (sur lequelle le modèle apprend) ainsi que sur le test tout au long de l'apprentissage et de pouvoir les comparer. Si la loss en train chute mais pas la loss sur le jeu de test alors le modèle sur-apprend, les deux loss doivent évoluer de manière similaire ce qui signifie que notre modèle réussi à généraliser. 

## Descente de gradient 

On calcule le gradient parce qu’il indique, pour chaque paramètre, dans quel sens et de combien la perte augmente quand on le modifie. En se déplaçant dans la direction opposée au gradient, on peut faire baisser la perte, et donc par conséquent améliorer le réseau. Pour cela il est nécessaire de venir mettre à jour les paramètres de la manière suivante : 
$$w \leftarrow w - \eta \, \frac{\partial L}{\partial w}  \iff w_{ij} \leftarrow w_{ij} - \eta \frac{\partial L}{\partial w_{ij}}$$
où $\eta$ est le taux d'apprentissage (souvent noté lr pour learning rate), paramètre à choisir pour l'entraînement, un lr trop petit et le réseau apprend lentement, un lr trop grand et le réseau n'arrive pas à converger. 
Pour chaque point de données du jeu d'entraînement on vient calculer sa prédiction par le modèle, puis la loss puis le gradient et on vient mettre à jour tous les paramètres du  modèle. (En pratique la rétro propagation, ne s'effectue pas pour chaque point de données mais pour un lot de points (gradient par batch), pour des raisons de stabilité de l'apprentissage et d'efficacité de calcule.) Plus de détails ici : [[Descente de gradient]]

## Paramètres vs Hyperparamètres

Dans un réseau de neurones, les paramètres font références au valeurs apprises par le réseau, dans ce cas ci les poids $W$ et les biais $B$, ils sont initialisé aléatoirement (ou suivant une [[Initialisation de réseau | fonction d'initialisation]] ) et sont modifiés lors de l'apprentissage. Les hyperparamètres font eux référence à toutes les variables que l'on doit choisir : le nombre de couches, la dimension de ces couches, les fonctions d'activations, le taux d'apprentissage, ... 

# Sources

[MIT Deeplearning book](https://www.deeplearningbook.org/)
[Collège de France - Théorème d'approximation Universel](https://www.college-de-france.fr/fr/agenda/cours/apprentissage-par-reseaux-de-neurones-profonds/approximation-universelle-par-un-reseau-une-couche-cachee)