use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Elemento representativo de un crate en listados y búsquedas del catálogo oficial.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateApiItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub max_version: String,
    pub description: Option<String>,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub recent_downloads: Option<u64>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Respuesta de búsqueda o listado de crates de crates.io.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CratesApiResponse {
    #[serde(default)]
    pub crates: Vec<CrateApiItem>,
}

/// Respuesta estructurada de la API de resumen (dashboard) de crates.io.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SummaryApiResponse {
    #[serde(default)]
    pub new_crates: Vec<CrateApiItem>,
    #[serde(default)]
    pub most_downloaded: Vec<CrateApiItem>,
    #[serde(default)]
    pub just_updated: Vec<CrateApiItem>,
    #[serde(default)]
    pub most_recently_downloaded: Vec<CrateApiItem>,
    #[serde(default)]
    pub popular_keywords: Vec<KeywordItem>,
    #[serde(default)]
    pub popular_categories: Vec<CategoryItem>,
}

/// Detalle completo de un crate incluyendo metadatos, versiones y categorías.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateFullDetailResponse {
    #[serde(rename = "crate")]
    pub krate: CrateMetadata,
    #[serde(default)]
    pub categories: Vec<CategoryItem>,
    #[serde(default)]
    pub keywords: Vec<KeywordItem>,
    #[serde(default)]
    pub versions: Vec<VersionItem>,
}

/// Metadatos profundos de un crate individual.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub recent_downloads: Option<u64>,
    pub max_version: String,
    pub max_stable_version: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Categoría oficial de clasificación en Crates.io.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryItem {
    pub id: String,
    pub category: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub crates_cnt: u64,
}

/// Palabra clave / tag para búsqueda en Crates.io.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeywordItem {
    pub id: String,
    pub keyword: String,
    #[serde(default)]
    pub crates_cnt: u64,
}

/// Versión individual publicada de un crate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionItem {
    pub id: Option<u64>,
    pub num: String,
    pub license: Option<String>,
    pub edition: Option<String>,
    pub rust_version: Option<String>,
    pub crate_size: Option<u64>,
    #[serde(default)]
    pub downloads: u64,
    pub created_at: Option<String>,
    pub yanked: Option<bool>,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}
