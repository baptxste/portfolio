---
title: Régression Linéaire
tags:
  - math
  - code
date: 2026-10-04
publish: true
---

l'erreur est habituellement mesurée avec la fonction RSS ( residual sum of squares ) : 
$$RSS = \sum_{i=1}^{n} \Bigg(y_i -\big(\beta_0  + \sum_{j=1}^{p}\beta_jx_{ij}\big)^2\Bigg) 
$$
Avec  :
- $n$ le nombre de donnée
- $y_i$ la sortie attendue
- $p$ le nombre de paramètres 
- $\beta_j$ les coefficients du modèle
# Régularisation 
L'un des soucis avec la régression linéaire est que l'on peut facilement voir apparaître de l'overfitting. Pour éviter cela il est courant d'appliquer des régularisation qui ont pour but de modifier la fonction erreur : 
## - 1) Régularisation L1 ( ou Lasso)
***L**east **A**solute **S**hrinkage and **S**election **O**perator*
on ajoute un terme de pénalité à l'erreur RSS : 
$$ RSS + \lambda \sum_{j=1}^p |\beta_j|$$
le paramètre $\lambda$ est à définir par l'utilisateur, il est aussi souvent appelé $\alpha$. 

Permet de diminuer le nombre de coefficients effectif, c'est-à-dire que certains coefficients seront exactement nuls. Cela peut être utilisé pour la sélection de caractéristiques.
## - 2) Régularisation L2 ( ou Ridge)
on ajoute un terme de pénalité à l'erreur RSS : 
$$ RSS + \lambda \sum_{j=1}^p \beta_j^2$$
le paramètre $\lambda$ est à définir par l'utilisateur, il est aussi souvent appelé $\alpha$.

Tends à rendre les coefficients plus petits, mais sans les forcer à être nuls. Les coefficients sont simplement réduits proportionnellement à leur taille.

## - 3) Régularisation Elastic Net
Parfois, il est utile de combiner les deux types de régularisation. Cela donne la régularisation Elastic Net :

- **Formule** : La pénalité Elastic Net est une combinaison linéaire des pénalités L1 et L2, $\lambda_1 \sum_{j=1}^{p} |w_j| + \lambda_2 \sum_{j=1}^{p} w_j^2$ 
- **Effet** : Combine les avantages de L1 (sparsité) et L2 (stabilité numérique).

# Utilisation en Python avec scikit-learn

Voici un exemple d'utilisation des régularisations L1 et L2 avec la bibliothèque `scikit-learn` en Python :
``` python
from sklearn.linear_model import Lasso, Ridge, ElasticNet
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error

# Suppose we have some data
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Lasso Regression (L1)
lasso = Lasso(alpha=0.1)
lasso.fit(X_train, y_train)
y_pred_lasso = lasso.predict(X_test)
print("Lasso MSE:", mean_squared_error(y_test, y_pred_lasso))

# Ridge Regression (L2)
ridge = Ridge(alpha=0.1)
ridge.fit(X_train, y_train)
y_pred_ridge = ridge.predict(X_test)
print("Ridge MSE:", mean_squared_error(y_test, y_pred_ridge))

# Elastic Net (L1 + L2)
elastic_net = ElasticNet(alpha=0.1, l1_ratio=0.5)
elastic_net.fit(X_train, y_train)
y_pred_enet = elastic_net.predict(X_test)
print("Elastic Net MSE:", mean_squared_error(y_test, y_pred_enet))

```
En résumé, le choix entre L1 et L2 dépend de vos besoins spécifiques : utilisez L1 si vous souhaitez un modèle parcimonieux avec sélection de caractéristiques, et L2 si vous voulez un modèle plus stable avec toutes les caractéristiques conservées. Elastic Net peut être utilisé pour combiner les avantages des deux.
# Sources 

[Medium](https://medium.com/@novus_afk/regularized-linear-regression-35d5eaaa84d5)