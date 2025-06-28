#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type CollectionUuid = Uuid;
pub type Metadata = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserIdentityResponse {
    pub user_id: String,
    pub tenant: String,
    pub databases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatResponse {
    #[serde(rename = "nanosecond heartbeat")]
    pub nanosecond_heartbeat: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChecklistResponse {
    pub max_batch_size: i32,
    pub supports_base64_encoding: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTenantPayload {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTenantResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTenantResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateDatabasePayload {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateDatabaseResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub id: Uuid,
    pub name: String,
    pub tenant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteDatabaseResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingFunctionConfiguration {
    Legacy {
        r#type: String, // "legacy"
    },
    Known {
        r#type: String, // "known"
        #[serde(flatten)]
        config: EmbeddingFunctionNewConfiguration,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingFunctionNewConfiguration {
    pub name: String,
    pub config: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HnswSpace {
    L2,
    Cosine,
    Ip,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct HnswConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ef_construction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ef_search: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_neighbors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<HnswSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_threshold: Option<u32>,
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct SpannConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ef_construction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ef_search: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_neighbors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reassign_neighbor_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_nprobe: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<HnswSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_nprobe: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CollectionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_function: Option<EmbeddingFunctionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hnsw: Option<HnswConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spann: Option<SpannConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub id: CollectionUuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub configuration_json: CollectionConfiguration,
    pub tenant: String,
    pub database: String,
    pub log_position: i64,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateCollectionPayload {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CollectionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_or_create: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCollectionResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateCollectionPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_configuration: Option<UpdateCollectionConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateCollectionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_function: Option<EmbeddingFunctionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hnsw: Option<UpdateHnswConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spann: Option<SpannConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct UpdateHnswConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ef_search: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_neighbors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_threads: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_threshold: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingsPayload {
    Float(Vec<Vec<f32>>),
    String(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddCollectionRecordsPayload {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<EmbeddingsPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<Vec<Option<Metadata>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddCollectionRecordsResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RawWhereFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_document: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteCollectionRecordsPayload {
    #[serde(flatten)]
    pub where_fields: RawWhereFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCollectionRecordsResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForkCollectionPayload {
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Include {
    Distances,
    Documents,
    Embeddings,
    Metadatas,
    Uris,
}

pub type IncludeList = Vec<Include>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequestPayload {
    #[serde(flatten)]
    pub where_fields: RawWhereFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<IncludeList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    pub ids: Vec<String>,
    pub include: Vec<Include>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<Vec<Option<Metadata>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Vec<f32>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryRequestPayload {
    #[serde(flatten)]
    pub where_fields: RawWhereFields,
    pub query_embeddings: Vec<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<IncludeList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_results: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResponse {
    pub ids: Vec<Vec<String>>,
    pub include: Vec<Include>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distances: Option<Vec<Vec<Option<f32>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<Vec<Vec<Option<Metadata>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Vec<Option<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<Vec<Option<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Vec<Vec<Option<f32>>>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum UpdateEmbeddingsPayload {
    Float(Vec<Option<Vec<f32>>>),
    String(Vec<Option<String>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCollectionRecordsPayload {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<UpdateEmbeddingsPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<Vec<Option<Metadata>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCollectionRecordsResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpsertCollectionRecordsPayload {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<EmbeddingsPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<Vec<Option<Metadata>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpsertCollectionRecordsResponse {}
