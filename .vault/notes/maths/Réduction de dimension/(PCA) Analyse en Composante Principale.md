---
title: (PCA) Analyse en Composante Principale
tags:
  - math
  - dimension
date: 2026-10-04
publish: true
---

# Introduction

L'analyse en composantes principales (ACP) est une technique de réduction de dimension.  
L'idée principale est de regrouper plusieurs variables qui suivent une même tendance en une seule, afin de réduire la quantité de données à traiter tout en conservant l'essentiel de l'information.  
En pratique, on parle de variables corrélées (qui portent une information redondante) et de nouvelles variables décorrélées.  
Ces nouvelles variables sont appelées *composantes principales*, *axes principaux* ou encore *facteurs principaux*.

---

# Principe général

L'ACP repose sur l'étude de la variance et de la covariance, puis sur la diagonalisation de la matrice de covariance.

#### Rappel :
- Soit $U$ une variable aléatoire $\in \mathbf{R}$, alors $\mathrm{Var}(U) = \mathbb{E}\big[(U - \mathbb{E}(U))^2\big]$.
- Pour un vecteur aléatoire $X$, la matrice de covariance est
  $$\mathrm{cov}(X) = \mathbb{E}\big[(X - \mathbb{E}(X))(X - \mathbb{E}(X))^T\big],$$
  c'est une matrice symétrique réelle définie positive.

![[pca_axe_principale.png]]

On note $D = \{x_i, 1 \le i \le n\}$, avec $x_i \in \mathbf{R}^d$ (par exemple $d=2$).  
On cherche un vecteur unitaire $\vec{v}$ tel que la variance de la projection de $D$ sur l'axe engendré par $\vec{v}$ soit maximale.

On projette un point $x$ sur $\vec{v}$ par :
$$x' = \vec{v}^T x.$$

On suppose que les $x$ sont centrés, ( $\mathbb{E}_x(x) = 0$ ) alors $\mathbb{E}_x(\vec{v}^T x) = \vec{v}^T \mathbb{E}(x) = 0,$ et
$$\mathbb{E}_x\big[(\vec{v}^T x)^2\big] = \mathbb{E}_x\big[\vec{v}^T (xx^T) \vec{v}\big].$$

Comme par hypothèse $\mathbb{E}(x) = 0$, on a $\mathrm{cov}(x) = \mathbb{E}(x x^T).$
On cherche donc la direction $\vec{v}$ qui maximise la variance projetée, c'est-à-dire qui maximise
$$\mathbb{E}_x\big[\vec{v}^T (x x^T) \vec{v}\big].$$

---

# Décomposition spectrale de la covariance

On note la matrice de covariance $\mathrm{cov}(x) = \Sigma$.  
D'après le théorème spectral, une matrice symétrique réelle est diagonalisable dans une base orthonormée.

On peut donc écrire :
$$\Sigma = V \, \Omega \, V^T,$$
où $V$ est la matrice dont les colonnes sont les vecteurs propres $e_i$, et $\Omega$ est la matrice diagonale des valeurs propres $\lambda_i$.

On peut représenter $V$ ainsi :
$$V = \begin{bmatrix}
\vdots & \vdots &        & \vdots \\
e_1    & e_2    & \cdots & e_n    \\
\vdots & \vdots &        & \vdots
\end{bmatrix},
\quad \text{avec } e_i \text{ les vecteurs propres.}$$

Comme les $e_i$ forment une base orthonormée, on a :
$$V^T e_i = \begin{pmatrix} e_1^T e_i \\ e_2^T e_i \\ \vdots \\ e_n^T e_i \end{pmatrix} = \begin{pmatrix} 0 \\ \vdots \\ 1 \leftarrow \text{$i$-ème position} \\ \vdots \\ 0 \end{pmatrix}.$$

Par ailleurs,
$$\Sigma e_i = \lambda_i e_i,$$
c'est-à-dire que chaque vecteur propre $e_i$ est une direction privilégiée associée à la valeur propre $\lambda_i$.

---

# Maximisation de la variance projetée

On cherche un vecteur unitaire $\vec{v}^*$ tel que
$$\vec{v}^T V \Omega V^T \vec{v}$$
soit maximal, sous la contrainte $\|\vec{v}\| = 1$.

On écrit $\vec{v}$ dans la base des vecteurs propres :
$$\vec{v} = \sum_{j=1}^{N} b_j e_j.$$

Alors
$$\vec{v}^T V \Omega V^T \vec{v}
= \left(\sum_{j=1}^{N} b_j e_j^T\right)
  \left(\Sigma \sum_{j=1}^{N} b_j e_j\right)
= \left(\sum_{j=1}^{N} b_j e_j^T\right)
  \left(\sum_{j=1}^{N} b_j \Sigma e_j\right).$$

Or $\Sigma e_j = \lambda_j e_j$, donc
$$\left(\sum_j b_j e_j^T\right)
\left(\sum_j b_j \lambda_j e_j\right)
= \sum_i \sum_j b_i b_j \lambda_j \, e_i^T e_j.$$

Comme les $e_i$ sont orthonormés, $e_i^T e_j = 0$ si $i \ne j$ et $1$ si $i=j$, d'où :
$$\vec{v}^T \Sigma \vec{v} = \sum_j b_j^2 \lambda_j.$$

La contrainte $\|\vec{v}\| = 1$ devient
$$\|\vec{v}\|^2 = \left\|\sum_j b_j e_j\right\|^2
= \sum_j b_j^2 = 1.$$

On cherche donc à maximiser
$$\sum_j b_j^2 \lambda_j
\quad \text{sous la contrainte} \quad
\sum_j b_j^2 = 1.$$

Pour maximiser cette somme, il suffit de prendre $b_j = 1$ pour l'indice $j$ correspondant à la plus grande valeur propre $\lambda_j$, et $b_k = 0$ pour tous les autres $k \ne j$.  
L'axe $e_j$ est ainsi appelé *axe d'inertie* : c'est la direction qui maximise la variance.

---

# Réduction de dimension et projection

Le principe de l'ACP est donc de diagonaliser la matrice de covariance $\Sigma$ et d'ordonner les valeurs propres $\lambda_i$ dans l'ordre décroissant.  
Les vecteurs propres associés aux plus grandes valeurs propres définissent les axes principaux « les plus informatifs ».

Pour réduire la dimension, il suffit de projeter les données sur les premiers axes principaux (ceux correspondant aux plus grandes valeurs propres).  
Si $V$ est la matrice des vecteurs propres, les coordonnées d'un point $x$ dans la nouvelle base sont :
$$x' = V^T x.$$

En ne gardant que les $k$ premières composantes de $x'$, on obtient une représentation en dimension $k$ qui conserve le plus possible de variance.

---

# Variance expliquée

La projection en dimension 2 (ou en dimension $k$ plus généralement) n'est pas toujours intéressante si l'on perd trop de variance.  
Pour quantifier cela, on introduit la *variance expliquée*.

La variance totale est :
$$\mathrm{Var}(x) = \lambda_1 + \lambda_2 + \dots + \lambda_n.$$

On définit le rapport de variance expliquée par les deux premières composantes :
$$\frac{\lambda_1 + \lambda_2}{\mathrm{Var}(x)}.$$
Si ce rapport est proche de 1, cela signifie que l'on conserve une grande partie de la variance dans les deux premières composantes principales, ce qui rend la représentation en dimension 2 pertinente.

Plus généralement, on peut tracer la fonction
$$f(k) = \frac{\sum_{i=1}^k \lambda_i}{\sum_{i=1}^n \lambda_i},$$
qui donne la part de variance expliquée par les $k$ premières composantes principales.

Ce graphe permet de choisir un $k$ adapté : par exemple, le plus petit $k$ tel que $f(k)$ soit supérieur à un seuil (0.9, 0.95, etc.), afin de conserver une grande partie de l'information tout en réduisant la dimension.

![[pca_var_cumulee.png]]

# Ressources
#todo
[[Vraisemblance]]
### cercle des corrélations


[[Cours9_ReductionDimension_Part_I.pdf]]


[IBM](https://www.ibm.com/topics/principal-component-analysis)
[Cours pdf Rennes](https://perso.univ-rennes2.fr/system/files/users/jegou_n/acp-cours.pdf)
[Cours pdf Toulouse](https://www.math.univ-toulouse.fr/~besse/Wikistat/pdf/st-m-explo-acp)