// -> à placer dans src/vault_assets.rs
//
// Ce fichier ne contient rien "en dur" : il inclut simplement le code généré
// par build.rs (un `asset!()` par image du vault + resolve_vault_asset()).
// Ajouter `mod vault_assets;` dans main.rs / lib.rs.

include!(concat!(env!("OUT_DIR"), "/vault_assets.rs"));