---
title: template
tags:
  - data
  - pipeline
  - datalake
  - todo
date: 2026-11-08
publish: true
---
# Définitions

Tous les trois des solutions de gestion de données. Ces solutions sont différentes et peuvent être combinées pour former une _Data fabric_

**Data warehouse** : Aggrège, nettoie et prépapre de la donnée.

**Data Lake**: Stocke de grandes quantités à bas coûts.

**Data Lakehouse** : Combine les deux.

**Transaction ACID**: Atomicité, Cohérence, Isolation, Durabilité

- A: la transaction se fait en un seul bloc qui réussit ou échoue (rollback) pas de requête partielle, si une partie de la req échoue tout est annulé.
- C: la transaction maintient la base dans un état valide (respect règles, contraintes ... )
- I: des transaction effectuée en même temps n'interfèrent pas, le résultat doit etre identique à une execution séquentielle
- D: une fois la transcation terminée/ validée (commit) la persistance est permanente

**Delta Lake_**: Format de stockage de donnée qui combine le format parquet avec un journal de métadonnées

**fichier ORC** : Optimized row columnar - équivalent à parquet dans hive / hadoop

**Z-order** : Algorithme d'optimisation qui permet de stocker des données similaires proche les unes des autres. C'est une sorte de tri mais sur plusieurs colonnes en même temps. 

**OLAP** : Online Analytical processing opposé à **OLTP** : Online Transactionnal Processing


# OLAP vs OLTP 
#todo cas d'usage + exemple de db
# Data Lake

Objectifs traiter une très grande quantité de données, pas de formattage initial, ça utilise une approche _schema on read_ ce qui evite tout le preprocess de la donnée. Les données sont stockées comme quand elles arrivent, structurée, semi structurée ou non structurée ( images, free form text, ...), utilisation de cloud objets -> pas cher mais pas acid et pas possible de lister metadonnée

## Architecture:

- construit sur des systèmes de stockage d'objet genre amazon s3,microsoft azure blob storage ...
- Séparation strict entre le stockage et le compute l'un peut scale sans l'autre
- pour accéder aux données il faut passer par d'autres types d'outils genre spark qui ne font pas parti intégrante du lake
- souvent utilisé pour stocker de la donnée pas trop utilisée genre backup, archive etc etc

avantages : pas cher pour traiter des volumes de donnés massifs. plus souple car pas besoin de formater incovénients: comme y'a pas de structure c'est une galère pour la gouvernance ou la qualité, rend beaucoup plus compliqué la BI, ne permet pas de garantir ACID, pas possible de faire des query hautes performances. se base sur des

## Techno

- apache parquet
- apache iceberg

# Data Warehouse

aggrège de la donnée venant de différentes bases ou flux dans un seul stockage. le style c'est du _schema on write_

## archi d'un data warehouse

- the bottom layer: la couche d'entrée des flux via un ETL (extract transform and load), dans les data warehouse on stocke dans du relationnel ou similaire
- middle layer : moteur sql ou olap, permet de query la donnée de manière optimisée, permet d'avoir des stats et des outils d'analyse
- top layer: toutes les interfaces utilisateur, pas vraiment une partie du datawarehouse mais ça représente tout ce qui va s'y connecter(excel, site intern, tableau, BI tools... )

avantages :Comme la donnée est structurée ou/et dans un schéma relationnel ça supporte des reqûetes sql haute performance. Donnée structurée donc facilite son utilisation par la suite. Facilite intégration BI - permet de faire facilement des query multi tables - peut globalement être query avec du sql Incovénients: ça coûte cher à maintenir, ça peut etre compliqué à scale et si on ne maintient pas bien on peut perdre les performances. et ça supporte mal de la donnée mal structurée.

# Data Lakehouse

essaye de combiner le meilleur des deux mondes, pareil que dans un dtalake on doit garder une séparation entre la couche de stockage et la couche de compute

## Architecture

- couche d'ingestion : peut faire du ETL mais la majorité fais du ELT (extract load transform) le but charger la donnée brute et la transformer plus tard uniquement quand ou si nécessaire
- Stockage pareil qu'un datalake stockage objet structurés ou non
- couche de méta data: c'est le point important chaque objet comporte des métadata qui permet de créer de sindex, de gérer de la gouvernance, la qualité etc
- couche d'exposition : API ou BI

avantage : éviter d'avoir un datalake et un data warehouse en parallèle bien que cela fonctionne, limite les risques de duplication de data, gouvernance ... incovénients: courbe d'apprentissage plus difficile, technologie nouvelle qui evolue encore nécessite une techno pour query la base ( spark, ... )

# Delta Lake

Ca ne remplace pas un data lake, c'est complémentaire. le data lake c'est le "où" ? le deltalake c'est le "comment"

Comme on conserve toutes les données brut mais que l'on ajoute de la métadata, deltalake permet de transformer un datalake en lakehouse

## Fonctionnement

deux composants :

- la donnée est stockée dans des tables appelées "tables delta" il s'agit en réalité de fichier format parquet
- métadonnées dont un journal : l'activité récente est stockée en json, l'ancienne et en parquet, dans les méta on a entre autre le min et le max de chaque colonne

Ces deux composants stockent la donnée dans le data lake. le journal est ce qui différencie le fichier q'un simple parquet.

Avantages : ajouter un journal permet de résoudre le manque d'ACID sur un datalake classique

# Technologies / framework

## Stockage objet

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|Amazon S3|Stockage objet, brique de base de la plupart des datalakes|Service managé|Pas d'ACID ni de listing des métadonnées|Propriétaire|Cloud|
|Azure Blob Storage|Stockage objet Microsoft|Service managé|Équivalent S3 côté Azure|Propriétaire|Cloud|
|MinIO|Stockage objet compatible API S3|Service à déployer|Alternative self-hosted à S3|Go|Hébergeable|

## Formats de fichier et de table

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|Parquet|Format de fichier colonnaire, compressé, adapté à l'analytique|Format de fichier|Base de Delta Lake|n/a (spec)|n/a|
|ORC|Format colonnaire optimisé, équivalent de parquet côté Hive / Hadoop|Format de fichier||n/a (spec)|n/a|
|Delta Lake|Parquet + journal de transactions : ACID, time travel, transforme un datalake en lakehouse|Format de table|Z-order disponible pour l'optimisation|Scala / Java|n/a|
|Apache Iceberg|Format de table ouvert : ACID, snapshots, évolution de schéma|Format de table|Cité dans la section Data Lake|Java|n/a|
|Apache Hudi|Format de table orienté upserts et traitement incrémental|Format de table||Java|n/a|

## Moteurs de calcul et de requête

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|Spark|- Moteur de calcul distribué open source<br>- traitement batch et streaming, SQL, ML<br>- sert à query un datalake / lakehouse (parquet, delta, iceberg)|Framework / cluster de calcul|Cité plus haut comme outil d'accès au datalake|Scala (API Python, SQL, Java, R)|Hébergeable|
|trino/presto|- Moteur de requête SQL distribué open source<br>- permet de fédérer différentes sources de données ( datalake, db , sql/nosql, ...)|Service à déployer||Java|Hébergeable|
|hive|- Data warehouse sur Hadoop (HDFS)<br>- permet de query des fichiers (parquet, ORC) en SQL via HiveQL<br>- le Hive Metastore centralise les métadonnées des tables|Service à déployer|Le Metastore est réutilisé par d'autres outils (Trino, Spark)|Java|Hébergeable|
|Flink|Calcul distribué orienté streaming (temps réel)|Framework / cluster de calcul||Java / Scala|Hébergeable|
|DuckDB|Moteur SQL analytique embarqué, lit directement parquet / CSV|Librairie embarquée|Pas de serveur à déployer|C++|n/a|

## Data warehouse et plateformes lakehouse

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|snowflake|- Data warehouse cloud managé<br>- stockage et compute séparés, scale indépendant<br>- supporte aussi la donnée semi-structurée (JSON, parquet)|SaaS (payant)|Pas self-hosted|Propriétaire (SQL)|Cloud|
|BigQuery|Data warehouse serverless de Google Cloud|SaaS (payant)||Propriétaire (SQL)|Cloud|
|Redshift|Data warehouse managé d'AWS|SaaS (payant)||Propriétaire (SQL)|Cloud|
|Databricks|Plateforme lakehouse construite autour de Spark et Delta Lake|SaaS (payant)|Éditeur de Delta Lake|Scala / Python|Cloud|
|ClickHouse|Base OLAP colonnaire très rapide sur gros volumes|Service à déployer|Existe aussi en offre managée|C++|Hébergeable|

## Ingestion, orchestration et transformation

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|dagster|- Orchestrateur de pipelines de données<br>- approche par assets (on déclare les données produites et leurs dépendances)<br>- UI de suivi, scheduling, monitoring|paquet|Alternative à Airflow|Python|Hébergeable|
|Airflow|Orchestrateur de workflows (DAGs) avec scheduling et monitoring|Service à déployer|Référence du marché|Python|Hébergeable|
|dbt|Transformation SQL versionnée dans le warehouse (approche ELT)|paquet||Python / SQL|Hébergeable|
|Kafka|Bus d'événements / streaming de données en temps réel|Service à déployer||Java / Scala|Hébergeable|
|Airbyte|Ingestion de données via des connecteurs (sources -> destination)|Service à déployer|Alternative open source à Fivetran|Java / Python|Hébergeable|

## Visualisation et BI

|Nom|But|Type|Commentaire|Language / techno|Hébergement|
|---|---|---|---|---|---|
|superset|Visualisation de données, création de dashboard||Déjà intégré dans Studio. Projet Apache|Python|Hébergeable|
|Metabase|BI simple, dashboards et requêtes sans SQL|Service à déployer||Clojure|Hébergeable|
|Grafana|Dashboards et monitoring, orienté métriques / séries temporelles|Service à déployer||Go|Hébergeable|
|Power BI|BI Microsoft, rapports et dashboards|SaaS (payant)||Propriétaire|Cloud|
|Tableau|BI et visualisation de données|SaaS (payant)|Existe aussi en Tableau Server|Propriétaire|Cloud|

# Sources :

IBM  
IBM OLAP  
IBM Delta Lake  
Microsoft Lakehouse vs warehouse  
Delta Lake: High-Performance ACID Table Storage over Cloud Object Stores  
Z order Medium  
Delta Z order
