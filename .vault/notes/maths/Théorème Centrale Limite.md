

#todo 

*Loi Normale :*
![[thm_centrale_limite.png]]
# Théorème 
Soit $(X_n$) une suite de variables aléatoires indépendantes et identiquement distribuées (i.i.d) admettant un moment d'ordre 2. On note :  $$ \mu = E(X) \text{ et } \sigma^2 = Var(X) = E(X-E(X))$$ $$S_n = X_1 + X_2 + ... + X_n \text{ , } Y_n = \frac{S_n - n \mu}{\sigma \sqrt{n}}$$
Alors la suite ($Y_n$) converge en loi vers une variable aléatoire de loi $N(0,1)$ . En d'autres termes, pour tout $x \in \mathbb{R}$ , $$P(Y_n \le x) \rightarrow \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{x} e^{-u^2/2} du $$

---
## Corrolaire
$$P(a \le Y_n \le b) \rightarrow \frac{1}{\sqrt{2 \pi}} \int_{a}^{b} e^{-u^2/2} du $$

# Exemples


# Ressources

- [Bibmath](https://www.bibmath.net/dico/index.php?action=affiche&quoi=./t/tcl.html)
- [Youtube 3Blue1Brown](https://youtu.be/zeJD6dqJ5lo?si=vdPlgMmrjHP-h5cf)
