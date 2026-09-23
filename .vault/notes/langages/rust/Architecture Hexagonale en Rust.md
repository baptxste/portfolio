---
title: Architecture Hexagonale en Rust
tags:
  - rust
date: 2026-09-23
publish: true
---

#   Architecture Hexagonale en Rust — Guide des principes à suivre

> Synthèse basée sur l'article _"Master Hexagonal Architecture in Rust"_ de [howtocodeit.com](https://www.howtocodeit.com/guides/master-hexagonal-architecture-in-rust). Ce document se concentre sur les **principes actionnables** et des **exemples de bon code**, pas sur la reproduction des mauvais patterns.

---

## 1. Le problème que l'architecture hexagonale résout

Le problème central : **les dépendances tierces (framework HTTP, client de base de données, etc.) fuient dans toute l'application**.

Symptômes typiques d'une application mal architecturée :

- `main` connaît les détails de configuration d'axum, du pool SQLite, etc.
- Les handlers HTTP exécutent directement des requêtes SQL et gèrent des transactions.
- Les handlers doivent connaître les codes d'erreur spécifiques du driver de base de données.
- Impossible d'écrire des tests unitaires : il faut une vraie base de données pour tester quoi que ce soit.

**Principe fondateur** : il faut encapsuler et abstraire les dépendances externes derrière des interfaces que définit _votre_ domaine métier — pas l'inverse.

### Quand une dépendance dure ("hard dependency") est-elle acceptable ?

Certaines dépendances sont si fondamentales qu'il ne sert à rien de les abstraire :

- **Tokio** : le runtime async est presque partie intégrante du langage.
- **anyhow** : son adoption est si large et son usage si générique que le risque de devoir le remplacer est minime.

En revanche, les clients HTTP, les drivers de base de données, les message queues, etc. **doivent** être abstraits : les équipes les changent régulièrement (montée en charge, dépréciation, sécurité, décisions organisationnelles).

---

## 2. Le pattern Repository

### Principe

Un handler ne doit jamais dire _"donne-moi ce store de données précis"_, mais _"donne-moi n'importe quel store de données qui respecte ce contrat"_.

On définit ce contrat sous forme de **trait** (un "port" dans le vocabulaire hexagonal). Une implémentation concrète de ce trait (SQLite, Postgres, en mémoire...) est un **adaptateur**.

```rust
/// AuthorRepository représente un store de données d'auteurs.
pub trait AuthorRepository: Clone + Send + Sync + 'static {
    /// Persiste un nouvel auteur.
    ///
    /// # Erreurs
    /// - Retourne `CreateAuthorError::Duplicate` si un auteur avec le même
    ///   nom existe déjà.
    fn create_author(
        &self,
        req: &CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}
```

### Pourquoi ça compte

- Le code appelant (handler HTTP, autre service...) ne connaît plus rien de SQL, MongoDB ou d'un message broker.
- Changer de base de données = écrire un nouvel adaptateur, sans toucher au reste de l'application.
- Le trait devient **la source de vérité** sur le comportement attendu de tout store de données pour les auteurs.

### Bornes de trait nécessaires pour l'async

Un trait async "naïf" ne suffit pas pour une application web multi-thread. Il faut expliciter :

```rust
pub trait AuthorRepository: Clone + Send + Sync + 'static {
    fn create_author(
        &self,
        req: &CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}
```

Explication des bornes :

- `Send` sur le `Future` retourné : nécessaire pour qu'il puisse être déplacé entre threads (obligatoire pour la plupart des serveurs web).
- `Send + Sync` sur le trait lui-même : requis dès qu'on veut le partager via `Arc` entre plusieurs tâches.
- `'static` : garantit que l'implémentation vit pendant toute la durée du programme.
- `Clone` : souvent requis par les frameworks web (ex. axum) pour injecter l'état dans chaque requête.

---

## 3. Les modèles de domaine

### Principe

Les modèles de domaine (`Author`, `CreateAuthorRequest`, `CreateAuthorError`...) sont la **représentation canonique** des données acceptées par la logique métier. Rien d'autre n'est valide.

```rust
/// Un auteur d'articles de blog identifié de manière unique.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
    id: Uuid,
    name: AuthorName,
}

impl Author {
    pub fn new(id: Uuid, name: AuthorName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}

/// Un nom d'auteur validé et normalisé.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorName(String);

#[derive(Clone, Debug, thiserror::Error)]
#[error("le nom de l'auteur ne peut pas être vide")]
pub struct AuthorNameEmptyError;

impl AuthorName {
    pub fn new(raw: &str) -> Result<Self, AuthorNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(AuthorNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}
```

### Règles à suivre

1. **Les validations vivent dans les constructeurs du domaine**, pas dans les handlers. Un `AuthorName` invalide ne peut tout simplement pas exister — voir le pattern _newtype_.
2. **Ne pas fusionner "requête de création" et "entité persistée"** dans un même type, même si ça paraît redondant au début.
    - Une entité complète (`Author`) et les données nécessaires pour la créer (`CreateAuthorRequest`) divergent presque toujours à mesure que l'application grandit.
    - Exemple concret : un profil client dans une fintech peut avoir des dizaines de champs optionnels collectés sur plusieurs semaines, alors que la création initiale ne nécessite que 2-3 champs.
3. **Attention à `serde`** : n'annotez un modèle de domaine avec `Serialize`/`Deserialize` que si (a) vous voulez permettre aux adaptateurs de (dé)sérialiser directement ce type **et** (b) le type ne fait strictement aucune validation sur ses champs. Sinon, la désérialisation contourne vos constructeurs et permet de créer des modèles invalides.

---

## 4. Gestion des erreurs dans le domaine

### Principe

Un type d'erreur de domaine doit décrire **exhaustivement** tout ce qui peut mal se passer lors d'une opération métier :

```rust
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CreateAuthorError {
    #[error("un auteur nommé {name} existe déjà")]
    Duplicate { name: AuthorName },

    #[error("erreur inattendue")]
    Unknown(#[source] anyhow::Error),
    // à étendre au fur et à mesure des nouveaux scénarios d'erreur
}
```

### Règles à suivre

- **Ne pas rendre l'enum `non_exhaustive`** si vous contrôlez tous les sites d'utilisation : vous _voulez_ que le compilateur casse quand une nouvelle variante d'erreur apparaît, pour forcer sa prise en charge partout. `non_exhaustive` n'a de sens que pour une bibliothèque publique.
- **Utiliser une variante `Unknown(anyhow::Error)`** comme filet de sécurité pour les erreurs que le domaine ne sait pas gérer spécifiquement (panne de connexion, erreur réseau imprévue...). `anyhow::Error` est pratique ici car il embarque une backtrace.
- **Ajouter du contexte aux erreurs opaques.** Votre code sait souvent des choses utiles que la bibliothèque appelée ignore :

```rust
sqlx_call()
    .await
    .with_context(|| format!("échec de sauvegarde de l'auteur {:?}", name))?;
```

- **Ne jamais paniquer sur une erreur inattendue.** Deux raisons concrètes :
    1. Un panic empoisonne les mutex détenus (`Arc<Mutex<T>>`). Si un thread panique en tenant le verrou, plus aucun thread ne pourra jamais l'acquérir — le programme est mort, même avec un middleware de récupération de panic.
    2. Les autres développeurs ne s'attendent pas à ce que votre code panique. Retournez des erreurs, suivez les conventions établies.

---

## 5. Implémenter un adaptateur (exemple SQLite)

### Principe

L'adaptateur encapsule **toute** la connaissance de l'implémentation concrète (SQL, transactions, codes d'erreur du driver...). Rien de tout ça ne doit fuiter vers l'appelant.

```rust
#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: sqlx::SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let pool = sqlx::SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(path)
                .with_context(|| format!("chemin de base de données invalide : {path}"))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("échec d'ouverture de la base à {path}"))?;

        Ok(Self { pool })
    }
}

impl AuthorRepository for Sqlite {
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let mut tx = self.pool.begin().await.context("échec de démarrage de transaction")?;

        let author_id = self
            .save_author(&mut tx, req.name())
            .await
            .map_err(|e| {
                if is_unique_constraint_violation(&e) {
                    CreateAuthorError::Duplicate { name: req.name().clone() }
                } else {
                    anyhow!(e)
                        .context(format!("échec de sauvegarde de l'auteur {:?}", req.name()))
                        .into()
                }
            })?;

        tx.commit().await.context("échec de commit de la transaction")?;

        Ok(Author::new(author_id, req.name().clone()))
    }
}
```

### Règles à suivre

- **Wrapper les types tiers** (ex. `sqlx::SqlitePool`) dans votre propre struct plutôt que de les exposer tels quels. Cela évite qu'un changement de version majeure de la bibliothèque ne se propage dans toute l'application.
- **Qualifiez les chemins de modules tiers explicitement** (`sqlx::SqlitePool` plutôt qu'un `use` générique) pour bien marquer la frontière entre code tiers et code applicatif.
- **La gestion de transaction reste entièrement dans l'adaptateur.** Le trait `AuthorRepository` ne mentionne jamais de transaction — c'est un détail d'implémentation SQL.
- **Traduisez les erreurs spécifiques du driver vers les erreurs du domaine** (ex. code d'erreur SQLite `2067` → `CreateAuthorError::Duplicate`).
- Les erreurs vraiment imprévues (panne, timeout...) sont enveloppées dans `Unknown` avec du contexte ajouté.

---

## 6. Adapter la couche transport (HTTP) au domaine

### Principe : conversion systématique entrée → domaine → sortie

Un handler HTTP :

1. reçoit une charge utile brute (JSON, etc.) ;
2. la convertit en modèle de domaine (avec validation) ;
3. appelle le port du domaine (repository ou service) ;
4. convertit le résultat (succès ou erreur) en réponse HTTP.

```rust
pub async fn create_author<AR: AuthorRepository>(
    State(state): State<AppState<AR>>,
    Json(body): Json<CreateAuthorHttpRequestBody>,
) -> Result<ApiSuccess<CreateAuthorResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .author_repo
        .create_author(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref author| ApiSuccess::new(StatusCode::CREATED, author.into()))
}
```

```rust
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateAuthorHttpRequestBody {
    name: String,
}

impl CreateAuthorHttpRequestBody {
    fn try_into_domain(self) -> Result<CreateAuthorRequest, AuthorNameEmptyError> {
        let author_name = AuthorName::new(&self.name)?;
        Ok(CreateAuthorRequest::new(author_name))
    }
}
```

### Règles à suivre

- **Ne jamais exposer directement une erreur de domaine à l'utilisateur final.** Construisez toujours un type d'erreur d'API séparé (`ApiError`) et convertissez explicitement :

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<CreateAuthorError> for ApiError {
    fn from(e: CreateAuthorError) -> Self {
        match e {
            CreateAuthorError::Duplicate { name } => {
                Self::UnprocessableEntity(format!("un auteur nommé {name} existe déjà"))
            }
            CreateAuthorError::Unknown(cause) => {
                tracing::error!("{:?}", cause);
                Self::InternalServerError("erreur interne du serveur".to_string())
            }
        }
    }
}
```

- Pourquoi construire le message à la main plutôt que réutiliser `Display` de l'erreur de domaine ? Parce que renvoyer des erreurs de domaine "telles quelles" :
    - risque de fuiter des détails d'implémentation privés ;
    - couple silencieusement votre réponse HTTP à la structure interne du domaine — un changement interne casse alors le contrat public.
- **Les modèles de requête HTTP et les modèles de domaine restent des types distincts**, reliés par une fonction de conversion explicite (`try_into_domain`). C'est ce qui permet de faire évoluer l'API et le domaine indépendamment.

---

## 7. Testabilité : le vrai gain de cette architecture

Grâce à l'injection de trait, on peut tester un handler HTTP **sans aucune base de données réelle**, en fournissant un mock qui implémente le même trait :

```rust
#[derive(Clone)]
struct MockAuthorRepository {
    create_author_result: Arc<Mutex<Result<Author, CreateAuthorError>>>,
}

impl AuthorRepository for MockAuthorRepository {
    async fn create_author(&self, _: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let mut guard = self.create_author_result.lock().await;
        let mut result = Err(CreateAuthorError::Unknown(anyhow!("erreur de substitution")));
        std::mem::swap(&mut *guard, &mut result);
        result
    }
}
```

### Règles à suivre

- Préférez une bibliothèque de mocking comme [`mockall`](https://docs.rs/mockall) plutôt que d'écrire des mocks à la main, sauf besoin spécifique.
- Chaque handler devient testable unitairement en isolant :
    - le cas de succès,
    - chaque variante d'erreur du domaine, sans jamais toucher à une vraie base de données. Réservez les tests d'intégration aux chemins critiques et aux happy paths globaux.

---

## 8. Le trait `Service` : où vit la vraie logique métier

### Principe

Dès que la logique métier dépasse "écrire en base et répondre 201" (envoi de notifications, métriques, événements...), il ne faut **ni** la mettre dans le handler HTTP **ni** dans le repository. Elle vit dans un **service**.

```rust
pub trait AuthorService: Clone + Send + Sync + 'static {
    fn create_author(
        &self,
        req: &CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}

#[derive(Debug, Clone)]
pub struct Service<R, M, N>
where
    R: AuthorRepository,
    M: AuthorMetrics,
    N: AuthorNotifier,
{
    repo: R,
    metrics: M,
    notifier: N,
}

impl<R, M, N> AuthorService for Service<R, M, N>
where
    R: AuthorRepository,
    M: AuthorMetrics,
    N: AuthorNotifier,
{
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let result = self.repo.create_author(req).await;

        if result.is_err() {
            self.metrics.record_creation_failure().await;
        } else {
            self.metrics.record_creation_success().await;
            self.notifier.author_created(result.as_ref().unwrap()).await;
        }

        result
    }
}
```

### Pourquoi c'est essentiel

Sans cette couche, chaque combinaison d'erreurs possibles entre le repository, les métriques et le notifier se retrouve mélangée à la gestion HTTP — un cauchemar combinatoire, testable uniquement en intégration (lent, coûteux, incomplet).

Avec un `Service` :

- **Tester le handler** : on mock le `Service`, un seul niveau d'abstraction suffit.
- **Tester le `Service`** : on mock chacune de ses dépendances (`AuthorRepository`, `AuthorMetrics`, `AuthorNotifier`) indépendamment.
- **Tests d'intégration** : réservés aux happy paths et aux scénarios d'erreur les plus critiques.

---

## 9. `main` sert uniquement au bootstrap

### Principe

`main` (ou une fonction `setup` dédiée) ne fait que :

1. construire les adaptateurs concrets (base de données, client email, etc.),
2. les assembler dans un `Service`,
3. injecter ce service dans les adaptateurs entrants (serveur HTTP...),
4. démarrer et arrêter proprement l'application.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    tracing_subscriber::fmt::init();

    let sqlite = Sqlite::new(&config.database_url).await?;
    let metrics = Prometheus::new();
    let email_client = EmailClient::new();

    let author_service = Service::new(sqlite, metrics, email_client);

    let server_config = HttpServerConfig { port: &config.server_port };
    let http_server = HttpServer::new(author_service, server_config).await?;

    http_server.run().await
}
```

### Règles à suivre

- **Même `main` ne doit pas connaître les détails du framework HTTP.** Créez votre propre wrapper (`HttpServer`) qui expose uniquement ce dont l'application a besoin (routes, port, timeouts). Si axum change son API, seul ce wrapper doit changer.
- **Minimisez le code dans `main`** : c'est une zone difficile à tester (dépendances non-mockables, erreurs gérées par simple sortie du programme). Moins il y a de code ici, plus petite est votre "zone morte" de tests.
- Cette approche évite aussi de dupliquer la configuration serveur entre `main` et les tests d'intégration : les deux réutilisent le même wrapper.

---

## 10. Comment choisir les bonnes frontières de domaine

Deux règles pratiques :

1. **Un domaine représente une branche tangible du métier** (ex. "blog", "facturation", "identité utilisateur"), pas une simple entité technique.
2. **Un domaine doit regrouper toutes les entités qui doivent changer ensemble de façon atomique.**
    - Si supprimer un `Author` doit supprimer atomiquement tous ses articles → auteurs et articles appartiennent au **même** domaine.
    - Si un délai est acceptable → ils peuvent être des domaines séparés communiquant via leurs API de `Service` (synchrone ou événementielle).
    - **Signal d'alerte** : si votre logique métier a besoin de propager des transactions SQL à travers plusieurs domaines pour rester atomique, vos frontières de domaine sont mal placées — ces entités doivent fusionner.

### Commencez avec de gros domaines

Il vaut mieux démarrer avec un **domaine unique et large**, puis le découper progressivement à mesure que les points de friction réels apparaissent. Découper trop tôt impose :

- d'écrire du code de communication inter-domaines avant d'en avoir besoin,
- de sacrifier l'atomicité,
- de fusionner à nouveau des domaines mal découpés au départ.

### Cas particulier : authentification et "master records"

Une entité comme `User` "possède" souvent de nombreuses autres entités (`Profile`, `Settings`, `Subscription`...). Pour les petites applications, tout regrouper dans un seul domaine est acceptable. Pour les applications plus grandes ou utilisant un fournisseur d'authentification tiers :

- isolez `User` et l'authentification dans leur propre domaine ;
- les autres domaines conservent une simple référence (ID) vers le propriétaire ;
- les suppressions se font de façon **non atomique**, soit par appel synchrone (`Service::delete_by_user_id`), soit par événement asynchrone.

C'est une conséquence assumée de la montée en échelle, pas un défaut de l'architecture.

---

## 11. Structure de projet recommandée

Une organisation qui a fait ses preuves à l'échelle :

```
src/lib/
├── domain/     # logique métier : modèles, traits (ports), services
├── inbound/    # adaptateurs entrants : HTTP, gRPC, CLI...
└── outbound/   # adaptateurs sortants : bases de données, clients externes...
```

Cette convention n'est pas sacrée — l'important est de **documenter le choix** fait pour que toute l'équipe s'y retrouve.

---

## 12. Quand utiliser (ou éviter) l'architecture hexagonale

### Utilisez-la si :

- **Projet perso pour apprendre** : excellent terrain d'entraînement à faible enjeu.
- **Startup qui vise une forte croissance** : donnez-vous dès le départ une couverture de tests solide et la capacité de changer de dépendances sans tout casser. Cela évite aussi la tentation dangereuse de partir directement en microservices avec de mauvaises frontières de domaine.
- **Gros monolithe difficile à maintenir, avant un passage aux microservices** : migrez vers un _monolithe hexagonal_ d'abord (frontières de domaine claires, sans le coût réseau), stabilisez les API de domaine, _puis_ extrayez en microservices si le besoin organisationnel est réel.
- **Nouveau projet dans une grande entreprise établie**, où la logique métier est non triviale et où les dépendances techniques changeront presque certainement au gré des décisions organisationnelles.

### Évitez-la si :

- **Vous êtes seul(e) sur un petit projet** que vous gardez "en tête" et que vous ne comptez pas partager : l'abstraction ajoute de la friction sans bénéfice pratique.
- **L'application n'a quasiment aucune logique métier** (CRUD basique, faible charge) : il n'y a rien à encapsuler, les ports/adaptateurs sont une complexité inutile. Un test d'intégration simple suffit souvent.
- **Application haute performance où chaque nanoseconde/octet compte** (trading haute fréquence, embarqué, zerocopy...) : le coût des conversions entrée→domaine→sortie devient significatif.

### Le coût à assumer

Comparée à une implémentation "directe", une application hexagonale demande objectivement plus de code :

- conversion systématique requête ↔ domaine ↔ réponse,
- une couche `Service` explicite,
- des wrappers autour de chaque dépendance tierce.

Ce n'est pas du code superflu : c'est le prix de la découplabilité, de la testabilité et de l'évolutivité. Mais en dessous d'un certain seuil de complexité métier, ce prix dépasse le bénéfice.

---

## Résumé express des principes clés

|Principe|En une phrase|
|---|---|
|Ports & adaptateurs|Le domaine définit des traits (ports) ; les dépendances externes les implémentent (adaptateurs).|
|Modèles de domaine|Seule représentation valide des données métier ; validation dans les constructeurs, pas dans les handlers.|
|Séparer requête / entité|`CreateXRequest` ≠ `X` : ils divergeront avec le temps.|
|Erreurs de domaine exhaustives|Une variante par cas métier + une variante `Unknown` pour l'imprévu ; jamais `non_exhaustive` dans votre propre code.|
|Ne jamais paniquer|Retournez des erreurs, même pour l'imprévu — le panic empoisonne les mutex et surprend l'équipe.|
|Adaptateurs encapsulent tout|Transactions, codes d'erreur spécifiques, détails du protocole : jamais visibles hors de l'adaptateur.|
|Erreurs domaine ≠ erreurs API|Toujours convertir explicitement, ne jamais exposer une erreur de domaine brute à l'utilisateur.|
|`Service` porte la logique métier|Orchestration de plusieurs ports (repo, métriques, notifications...) — jamais dans le handler ni le repository.|
|`main` = bootstrap uniquement|Construire les adaptateurs, assembler le service, démarrer/arrêter — rien de plus.|
|Domaines larges au départ|Regroupez ce qui doit changer atomiquement ; affinez au fil des frictions réelles observées.|
|Testabilité par mock|Chaque trait de domaine peut être mocké → tests unitaires rapides et exhaustifs, intégration réservée aux happy paths.|

---

_Basé sur : Angus Morrison, "Master Hexagonal Architecture in Rust", howtocodeit.com._