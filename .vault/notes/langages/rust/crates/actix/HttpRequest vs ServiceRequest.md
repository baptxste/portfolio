---
title: HttpRequest vs ServiceRequest
tags:
  - code
  - rust
date: 2026-10-05
publish: true
---

Les deux représentent une requête HTTP, mais pas au même niveau.
- `HttpRequest` → utilisé côté application / handlers / extracteurs
- `ServiceRequest` → utilisé côté middleware / pipeline Actix

# `HttpRequest`

On le retrouve surtout dans les **handlers** et les **extracteurs** (`FromRequest`) :
```rust
use actix_web::HttpRequest;

async fn get_user(req: HttpRequest) -> impl Responder {
    let auth = req.headers().get("Authorization");
    // ...
}
```

Il sert principalement à **lire** les informations de la requête :
- headers
- méthode HTTP
- URI / path
- query string
- informations de connexion
- app data

> `HttpRequest` = « je suis dans mon endpoint et j'ai besoin d'informations sur la requête. »

À noter : `HttpRequest` n'a **pas accès au body**. Dans un handler, le body est lu via des extracteurs (`web::Json`, `web::Bytes`, `Payload`...).

## Exemple

Pour simplement récupérer un header :
```rust
async fn endpoint(req: HttpRequest) -> impl Responder {
    let value = req.headers().get("X-Request-ID");
    // ...
}
```

Pas besoin de `ServiceRequest` ici.

---

# `ServiceRequest`

`ServiceRequest` est dans `dev` car c'est une API plus bas niveau d'Actix.
On l'utilise surtout dans les **middlewares** :

```rust
use actix_web::dev::ServiceRequest;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // vérifier la requête
    next.call(req).await
}
```

La différence importante : le middleware fait partie du **pipeline de traitement**. Il peut décider de :
- **bloquer** la requête ;
- **laisser** la requête continuer.

```text
                    Request
                       │
                       ▼
               Auth middleware
                       │
                token valide ?
                 /          \
               NON          OUI
                │             │
               401      next.call(req)
                              │
                              ▼
                           Handler
```

---

# Le rôle de `Next`

```rust
next.call(req).await
```

signifie :
> « J'ai fini mon traitement, passe la requête au service suivant. »

Exemple :
```rust
match is_valid_token(token).await {
    Ok(true) => next.call(req).await,
    Ok(false) => {
        Err(ErrorUnauthorized("Invalid token"))
    }
    Err(_) => {
        Err(ErrorUnauthorized("Invalid token"))
    }
}
```

Si le token est **valide** :
```text
middleware
    │
    ▼
next.call(req)
    │
    ▼
handler
```

Si le token est **invalide** :
```text
middleware
    │
    ▼
401
```

Le handler n'est jamais appelé.

---

# Pourquoi `ServiceRequest` et pas `HttpRequest` dans un middleware ?

Ce n'est pas « moins optimal » : le type est **imposé** par Actix.

Tout ce qui est dans le pipeline (middlewares inclus) est un `Service<ServiceRequest>` qui renvoie un `ServiceResponse`. Et `next.call(...)` n'accepte qu'un `ServiceRequest`. Un middleware déclaré avec `HttpRequest` ne compile tout simplement pas.

## Même avec un middleware custom

Ça ne change rien : la contrainte vient de `wrap()`, pas de `from_fn`.

```rust
impl<S, B> Transform<S, ServiceRequest> for MyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    // ...
}
impl<S, B> Service<ServiceRequest> for MyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // ...
        self.service.call(req)
    }
}
```

`Transform<S, ServiceRequest>` et `Service<ServiceRequest>` sont imposés. On peut techniquement écrire un `Service<HttpRequest>`, mais le pipeline ne pourra jamais l'appeler : ce ne serait pas un middleware.

## Ce que contient `ServiceRequest`

C'est un wrapper : **`HttpRequest` + `Payload`** (le body en streaming).
```text
ServiceRequest ──into_parts()──► (HttpRequest, Payload) ──► extracteurs ──► handler
```

- Lecture (headers, path, méthode, app data) : marche directement sur `ServiceRequest`.
- Récupérer un `HttpRequest` dans un middleware : `req.request()` (référence) ou `req.into_parts()`, puis `ServiceRequest::from_parts(http_req, payload)` pour reconstruire.
- Lire ou modifier le body dans un middleware : il faut `ServiceRequest`.

C'est ce découpage qui explique le partage des rôles : `ServiceRequest` (requête + body brut) circule dans le pipeline, `HttpRequest` est la vue « sans body » donnée au code applicatif.

---

# `ServiceRequest` et `ServiceResponse`

Dans le pipeline, on retrouve souvent les deux :
```text
ServiceRequest
      │
      ▼
  Middleware
      │
      ▼
ServiceResponse
```

Un middleware peut donc aussi faire du traitement **après** le handler :
```rust
let response = next.call(req).await?;
// traitement de la réponse
Ok(response)
```

Ce qui donne :
```text
Request
   │
   ▼
Middleware
   │
   │ avant
   ▼
next.call(req)
   │
   ▼
Handler
   │
   ▼
Response
   │
   │ après
   ▼
Middleware
```

Utile pour du logging, du tracing, la modification de headers de réponse, etc.

---

# Comparaison

| |`HttpRequest`|`ServiceRequest`|
|---|:-:|:-:|
|Niveau|haut niveau|bas niveau|
|Handler|✅|❌|
|Extracteur (`FromRequest`)|✅|❌|
|Middleware (type du paramètre)|❌|✅|
|Lire les headers|✅|✅|
|Lire le path|✅|✅|
|Lire la méthode|✅|✅|
|Accéder aux app data|✅|✅|
|Accéder au body (`Payload`)|❌|✅|
|Contrôler le pipeline|❌|✅|
|`next.call(req)`|❌|✅|
|Bloquer avant le handler|❌|✅|
|API `dev`|❌|✅|

---

# Cas d'utilisation

## `HttpRequest`

À utiliser dans un handler ou un extracteur quand on a besoin d'informations sur la requête :
- lire un header ;
- récupérer le path ;
- récupérer des infos sur la connexion ;
- accéder à certaines données de contexte.

```rust
async fn endpoint(req: HttpRequest) -> impl Responder {
    let user_agent = req.headers().get("User-Agent");
    // logique métier
}
```

## `ServiceRequest`

À utiliser quand on construit un middleware :
- authentification ;
- autorisation ;
- logging ;
- tracing ;
- rate limiting ;
- vérification de headers ;
- validation globale ;
- ajout d'informations au contexte de la requête.

```rust
async fn middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // vérifier quelque chose
    next.call(req).await
}
```

---

# Auth : extracteur vs middleware

Pour l'authentification, il y a deux approches, et elles n'utilisent pas le même type.

## Approche 1 : extracteur (`FromRequest`)

Un extracteur reçoit un `&HttpRequest`. On valide le token dans `from_request`, et il suffit d'ajouter `claims: Claims` dans les paramètres du handler :

```rust
impl FromRequest for Claims {
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone(); // peu coûteux, HttpRequest est un Rc en interne
        Box::pin(async move {
            // 1. header Authorization
            // 2. decode_header -> kid
            // 3. récupération de la clé (AppState)
            // 4. decode::<Claims>(...)
        })
    }
}
```

```rust
async fn me(claims: Claims) -> impl Responder {
    // ...
}
```

Ce n'est **pas un middleware**, même si ça fait le même travail de vérification.

## Approche 2 : middleware

Appliqué sur tout un scope avec `.wrap()`, avec `ServiceRequest`.

## Comparaison

| |Extracteur|Middleware|
|---|---|---|
|Où ça s'applique|Seulement les handlers qui ont `claims: Claims`|Tout le scope / l'app où on fait `.wrap()`|
|Oubli possible|Oui|Non : protégé par défaut|
|Type d'entrée|`&HttpRequest`|`ServiceRequest`|
|Accès aux claims dans le handler|Direct via le paramètre|Via `extensions()` ou un extracteur|

Le risque principal de l'extracteur seul, c'est l'**oubli** : si on ajoute un endpoint sans `Claims` / `AuthenticatedUser` dans ses paramètres, il est public sans que rien ne le signale.

Avec un middleware sur le scope, c'est l'inverse : il faut sortir explicitement une route du scope pour la rendre publique.

## Piège : double décodage

Si `AuthenticatedUser` appelle `Claims::from_request` en interne, un handler qui demande les deux fait valider le token **deux fois** (et potentiellement refaire l'appel pour récupérer la clé Keycloak).

---

# Pattern hybride (recommandé)

Le middleware **valide une seule fois** et dépose les claims dans les extensions de la requête. L'extracteur ne fait plus que les relire.

```text
Request
   │
   ▼
Auth middleware ── token invalide ──► 401
   │
   │ token valide
   │ req.extensions_mut().insert(claims)
   ▼
next.call(req)
   │
   ▼
Handler (claims: Claims) ── lit les extensions
```

## Middleware

```rust
use actix_web::HttpMessage; // pour extensions() / extensions_mut()
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // même logique que l'ancien Claims::from_request
    // (headers() et app_data() marchent aussi sur ServiceRequest)
    let claims = validate_token(&req).await?;
    req.extensions_mut().insert(claims);
    next.call(req).await
}
```

## Extracteur

```rust
impl FromRequest for Claims {
    type Error = ActixError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        std::future::ready(
            req.extensions()
                .get::<Claims>()
                .cloned() // Claims doit être Clone
                .ok_or_else(|| ErrorUnauthorized("Not authenticated")),
        )
    }
}
```

`AuthenticatedUser` reste identique : il appelle `Claims::from_request`, mais cette fois ça ne fait que lire les extensions. Plus de double décodage, et plus d'async dans l'extracteur.

## Branchement

```rust
web::scope("/api")
    .wrap(from_fn(auth_middleware))
    .service(...)
```

Les routes publiques (login, health...) restent hors de ce scope.

---
# Sources

[Doc ServiceResquest](https://docs.rs/actix-web/latest/actix_web/dev/struct.ServiceRequest.html)
[Doc HttpRequest](https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html)
Claude