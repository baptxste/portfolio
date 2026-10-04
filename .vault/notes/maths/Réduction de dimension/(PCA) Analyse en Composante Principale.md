---
title: (PCA) Analyse en Composante Principale
tags:
  - math
  - dimension
date: 2026-10-04
publish: true
---

## 1. Problème

On dispose d'un jeu de données :

$$D = {x_i,\ 1 \le i \le n}, \qquad x_i \in \mathbb{R}^2$$

On cherche un vecteur unitaire $\vec{v}$ tel que la **variance projetée** de $D$ sur la direction $\vec{v}$ soit **maximale**. La projection d'un point est :

$$x' = \vec{v} \cdot \vec{x}$$

### Rappel : variance d'une variable aléatoire

Pour $U$ une variable aléatoire réelle :

$$\mathrm{Var}(U) = \mathbb{E}\left[(U - \mathbb{E}(U))^2\right]$$

## 2. Hypothèse : données centrées

On suppose que les $x$ sont centrés, c'est-à-dire $\mathbb{E}_x(x) = 0$. Alors :

$$\mathbb{E}_x(v^T x) = v^T \mathbb{E}_x(x) = 0$$

La variance de la projection se simplifie donc en :

$$\mathbb{E}_x\left[(v^T x)^2\right] = \mathbb{E}_x\left[v^T (x x^T) v\right]$$

## 3. Matrice de covariance

$$\mathrm{Cov}(x) = \mathbb{E}\left[(x - \mathbb{E}(x))(x - \mathbb{E}(x))^T\right]$$

C'est une matrice **symétrique (semi-)définie positive**. Ici, comme $\mathbb{E}(x) = 0$ par hypothèse :

$$\mathrm{Cov}(x) = \frac{1}{N}\sum_{i=1}^{N} x_i x_i^T = \mathbb{E}(x x^T)$$

On cherche donc $\vec{v}$ tel que $\mathbb{E}_x(v^T x x^T v)$ soit maximal.

## 4. Diagonalisation (théorème spectral)

On note $\mathrm{Cov}(x) = \Sigma$. D'après le **théorème spectral**, $\Sigma$ est diagonalisable :

$$\Sigma = V \Omega V^T$$

avec :

- $V = \begin{pmatrix} e_1 & e_2 & \cdots & e_N \end{pmatrix}$ : matrice dont les colonnes sont les vecteurs propres $e_i$ de $\Sigma$
- $\Omega = \begin{pmatrix} \lambda_1 & & 0 \ & \ddots & \ 0 & & \lambda_N \end{pmatrix}$ : matrice diagonale des valeurs propres

> **Rappel :** les vecteurs propres sont orthogonaux (orthonormés), donc $V^T V = I$.

Les lignes de $V^T$ sont les $e_i^T$, d'où :

$$V^T e_i = \begin{pmatrix} e_1^T e_i \ e_2^T e_i \ \vdots \ e_N^T e_i \end{pmatrix} = \begin{pmatrix} 0 \ \vdots \ 1 \ \vdots \ 0 \end{pmatrix} = \varepsilon_i \quad (\text{le 1 est en position } i)$$

Puis :

$$\Omega V^T e_i = \begin{pmatrix} 0 \ \vdots \ \lambda_i \ \vdots \ 0 \end{pmatrix} \qquad \Longrightarrow \qquad V\left(\Omega V^T e_i\right) = \lambda_i e_i$$

## 5. Maximisation sous contrainte

On cherche $\vec{v}$ qui maximise $v^T V \Omega V^T v$ sous la contrainte $|v| = 1$.

On décompose $\vec{v}$ dans la base des vecteurs propres :

$$\vec{v} = \sum_{j=1}^{N} b_j e_j$$

Alors, d'après la section précédente, $V \Omega V^T v = \sum_j b_j \lambda_j e_j$ (matrice appliquée à $v$), et :

$$ \begin{aligned} v^T V \Omega V^T v &= \left(\sum_{j} b_j e_j^T\right)\left(\sum_{j} b_j \lambda_j e_j\right) \ &= \sum_i \sum_j b_i b_j ,\lambda_j, e_i^T e_j \ &= \sum_j b_j^2 \lambda_j \end{aligned} $$

car $e_i^T e_j = 1$ si $i = j$, et $0$ sinon (orthogonalité).

On cherche donc à **maximiser** $\sum_j b_j^2 \lambda_j$ sous la contrainte $|v| = 1$, soit :

$$\sum_j b_j^2 = 1$$

### Solution

Pour résoudre, on prend :

- $b_j = 1$ pour la **plus grande** valeur propre,
- $b_i = 0$ pour tout le reste.

Le vecteur propre $e_j$ correspondant est appelé **axe d'inertie** : c'est l'axe qui **maximise la variance**.

## 6. Conclusion : ce qu'est l'ACP

> **ACP = diagonaliser la matrice de covariance.**

(En pratique, on ordonne les valeurs propres de manière **décroissante** dans la matrice diagonale $\Omega$.)

## 7. Changement de base et réduction de dimension

### Nouvelles coordonnées

Pour obtenir les coordonnées dans la nouvelle base :

$$x' = V^T x \in \mathbb{R}^N$$

### Réduction de dimension

Pour réduire la dimension, on fait une **projection** sur les $k$ axes (par exemple $k = 2$) qui correspondent aux **plus grandes valeurs propres**.

> Parfois, projeter en dimension 2 n'est pas intéressant (trop de variance perdue).

## 8. Quelle dimension garder ? Variance expliquée

La variance totale est la somme des valeurs propres :

$$\mathrm{Var}(x) = \lambda_1 + \lambda_2 + \cdots + \lambda_N$$

Après réduction en dimension 2, la part de variance conservée est :

$$\frac{\lambda_1 + \lambda_2}{\mathrm{Var}(x)}$$

Si ce rapport est **proche de 1**, c'est bien : on garde le maximum de variance.

### Généralisation : courbe de variance cumulée

Si on trace, en fonction de $k$ :

$$\frac{\sum_{i=1}^{k} \lambda_i}{\sum_{j=1}^{N} \lambda_j}$$

- **Courbe foncée** : la variance est concentrée sur les premiers axes, quelques composantes suffisent (≈ 0,9 dès les premières dimensions).
- **Courbe claire** : la variance est répartie sur beaucoup d'axes, la réduction est moins efficace.
![[pca_var_cumulee.png]]
Souvent, on cherche à **conserver un maximum de variance** : ce rapport permet donc de **trouver la dimension de réduction** adaptée.