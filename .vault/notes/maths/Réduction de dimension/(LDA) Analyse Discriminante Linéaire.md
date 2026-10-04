---
title: (LDA) Analyse Discriminante Linéaire
tags:
  - math
  - dimension
date: 2026-10-04
publish: true
---
# Analyse Discriminante
L'analyse discriminante crée un modèle de prévision de groupe d'affectation. Le modèle est composé d'une fonction discriminante (ou, pour plus de deux groupes, un ensemble de fonctions discriminantes) basée sur les combinaisons linéaires des variables de prédicteur qui donnent la meilleure discrimination entre groupes. Les fonctions sont générées à partir d'un échantillon d'observations pour lesquelles le groupe d'affectation est connu. Les fonctions peuvent alors être appliquées aux nouvelles observations avec des mesures de variables de prédicteur, mais de groupe d'affectation inconnu.

La variable de regroupement doit avoir *un nombre limité de catégories distinctes, codifiées sous forme de nombres entiers.* Les variables indépendantes nominales doivent être re-codées en variables muettes ou de contraste.

 Les observations doivent être indépendantes. Les variables de prédicteur doivent avoir une *distribution gaussienne multivariée*, et les matrices de variance-covariance intra-groupes doivent être égales entre groupes. On part de l'hypothèse que les groupes d'affectation sont mutuellement exclusifs (c'est-à-dire qu'aucune observation n'est affectée à plus d'un groupe) et collectivement exhaustifs (c'est-à-dire que toutes les observations sont affectées à un groupe). La procédure est la plus efficace lorsque l'affectation à un groupe est une variable réellement catégorielle. Si l'affectation à un groupe est basée sur les valeurs d'une variable continue (par exemple, QI élevé contre QI bas), vous devez envisager d'utiliser la régression linéaire pour exploiter les informations plus riches données par la variable continue elle-même.


# Analyse Discriminante Linéaire

Contrairement à la régression logistique, l'analyse discriminante linéaire permet de séparer des données en plusieurs classe et donc d'effectuer des classification qui ne sont plus seulement binaires.
La LDA permet de réduire le nombre de paramètres, on regroupe des ensemble de données dans des classes et on s'intéresses aux paramètres de ces classes qui sont donc moins nombreux. 

### Hypothèses :
- Il faut que les données soient linéairement séparables 
- les données doivent suivre des distributions normales ou gaussiennes.
- Il faut que les classes extraites possèdent des matrices de covariance identiques.
( *nb.* Il est possible de ne pas respecter parfaitement ces hypothèses et d'obtenir des résultats corrects sous peine de ruser un petit peu.)

# Principe :
La LDA est une forme généralisée de la [discrimination linéaire de Fischer.](Fischer_discriminative_analysis.pdf) L'idée générale derrière cette méthode est de projeter les données sur des axes optimaux, c'est à dire qui vont maximiser la distance entre différentes classes et minimiser l'écart entre les points d'une même classe. Plus visuellement on obtiens ceci :
![[lda_projection.png]]Sur ce schéma on voit que l'on peut obtenir différentes représentations d'un même ensemble de données en fonction de l'axe sur lequel on effectue la projection. On voit clairement que dans le cadre de droite il est beaucoup plus facile de séparer les deux classes de données. 

On va considérer des points $x_{i}$( données) dans un espace de dimension $d$. 
Ces points appartiennent à deux classes distinctes $C_{1}$ et $C_{2}$ qui contiennent respectivement $N_{1}$ et $N_{2}$ points. Et $N = N_{1} +N_{2}$ 
On appelle $v$ le vecteur sur lequel on effectue la projection.
Pour un point $x_{i}$ on note $v_{xi}$ sa projection sur l'axe.
On note $m_{i}$ la moyenne d'une classe avant la projection et $M_{i}=v_{mi}$ la moyenne d'une classe une fois projetée sur le vecteur.

Intuitivement, on pourrait se dire le vecteur optimal est obtenu lorsque l'on maximise la distance |$M_{1} - M_{2}$|, cependant ce n'est pas car les projections des moyennes sont éloignés que les ensembles sont bien séparés. Comme le montre : ![[lda_fisher.png]]
L'écart est plus important sur l'axe horizontal cependant les données y sont très mal séparées. 
En effet cette méthode ne tiens pas compte de la variance de ces classes.
pour cela on calcule la *dispersion* : 
$$s_z^2 =  \sum_{i=1}^{n_z} (z_i - m_z)^2$$
*(qui n'est autre que la covariance de la classe multiplié par le nombre d'élément de cette classe. $\sigma^2 = \frac{1}{n} \sum_{i=1}^{n} (z_i - \mu_z)^2$  )*
Et plus particulièrement la dispersion des éléments projetés soit :  
$$S_z^2 =  \sum_{i=1}^{n_z} (z_i - M_z)^2$$

Ainsi on peut calculer le **Discriminant de Fischer** définit comme suit :
$$J(v) = \frac{(M_1 - M_2)^2}{S_1^2 + S_2}$$
L'analyse discriminante de Fischer vise donc à maximiser ce déterminant en faisant varier le vecteur sur lequel est effectué la projection. 
On cherche maintenant à exprimer $J(v)$ en fonction de $v$ afin de pouvoir le calculer facilement. Pour cela on définit les *matrices de dispersion intra-classe* : 
$s_z = \sum_{x_i \in \text{Class z}} (x_i - \mu_z)(x_i - \mu_z)^T$
et la matrice totale de dispersion intra-classe :
$S_w = \sum_{i} s_i$ 
Le [calcul (ici)](Cours9_ReductionDimension_Part_II.pdf#page=16|Cours9_ReductionDimension_Part_II, page 16) permet d'obtenir la relation $S_{i}² = v^t s_i v$   d'où    $S_1²+S_2² = v^t S_w v$
On définit maintenant les *matrices de dispersion inter-classe* :
$S_b  = (m_1 - m_2)(m_1 - m_2)^T$
Ainsi [on obtient](Cours9_ReductionDimension_Part_II.pdf#page=18|Cours9_ReductionDimension_Part_II, page 18) : $|M_{1} - M_{2}| = v^T S_b v$

d'où : 
$$J(v) = \frac{v^t S_b v}{v^t S_w v}$$
On dérive par rapport à $v$  et on annule la dérivée et on se retrouve à résoudre un problème de valeurs propres généralisé :
$$S_b v = \lambda S_w v$$
sous la condition que $S_w$ soit inversible ( ou $S_b$ ) on retombe sur un problème classique de valeurs propres. $S_w^{-1} S_b v =\lambda v$ , de plus par définition de $S_b$ [on a ](Cours9_ReductionDimension_Part_II.pdf#page=21|Cours9_ReductionDimension_Part_II, page 21) $S_b \propto \alpha(m_1-m_2)$
on résout et on obtient :$v=S_w^{-1}(m_1- m_2)$

### Cas des dimensions multiples 

Comme on l'a mentionné précédemment l'intérêt de la LDA est de pouvoir effectuer une discrimination entre de multiples classes ( >2). L'idée générale reste la même mais cette fois ci on ne projette plus sur un vecteur mais sur une matrice. 

On note $V$ la matrice de projection,$M$ la moyenne de toute les classes. $S_B = \sum_{i=1}^{c} n_i (m_i - M)(m_i - M)^T$ , $S_w = \sum_{i=1}^{c} S_i = \sum_{i=1}^{c} \sum_{x_k \in \text{class}_i} (x_k - m_i)(x_k - m_i)^T$ 

Le discriminant reste pareil en remplaçant juste le vecteur par la matrice de projection. On résout le problème de valeur propre qui admet au plus $c-1$ solutions avec $c$ le nombre de classe du problème. 
La matrice de projection optimale V vers un sous-espace de dimension $k$ est donnée par les vecteurs propres correspondant aux $k$ plus grandes valeurs propres.



### Ressources

[IBM Analyse Discriminante](https://www.ibm.com/docs/fr/spss-statistics/saas?topic=features-discriminant-analysis)
[IBM Analyse Discriminante Linéaire](https://www.ibm.com/topics/linear-discriminant-analysis)
[Scikit-learn LDA](https://scikit-learn.org/stable/modules/lda_qda.html)
[TD analyse discriminante linéaire en R](TD_LDA.pdf)
[Analyse discriminante et Régression logistique cours de l'UBS](poly_LDA_UBS.pdf)
[Cours réduction de dimension, Tato A. ](Cours9_ReductionDimension_Part_II.pdf)
[Geeks for geeks](https://www.geeksforgeeks.org/ml-linear-discriminant-analysis/
)
[analyticsvidhya](https://www.analyticsvidhya.com/blog/2021/08/a-brief-introduction-to-linear-discriminant-analysis/)
