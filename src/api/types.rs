use crate::hnsw::Index;

use serde::{Deserialize, Serialize};

use crate::db::dbtypes::{Collection, IndexDB};
use crate::db::RocksDB;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

pub type IndexStore = RwLock<HashMap<[u8; 16], Mutex<Index>>>;

pub struct State {
    pub index_store: IndexStore,
    pub db: RocksDB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIndexRequest {
    pub field_name: String,
    pub dimension: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexRequestPath {
    pub field_name: String,
    pub collection_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRequestPath {
    pub document_id: String,
    pub collection_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicesInfo {
    pub indices: Vec<IndexResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexResponse {
    pub collection_name: String,
    pub field_name: String,

    pub n_documents: u64,
    pub distance_metric: String,
    pub dimension: usize,
    // pub buffer_size: usize,
    // pub k: usize,
}

impl IndexResponse {
    pub fn from_db_type(index_db: &IndexDB) -> Self {
        Self {
            collection_name: index_db.collection_id.clone(),
            field_name: index_db.field_id.clone(),
            n_documents: index_db.n_elements,
            distance_metric: "euclidean".to_owned(),
            dimension: index_db.dimension,
            // buffer_size: index_db.buffer_size,
            // k: index_db.k,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionRequest {
    pub collection_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub collection_name: String,
    pub n_documents: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionsResponse {
    pub collections: Vec<CollectionResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub data: Vec<Vec<String>>,
}

impl CollectionResponse {
    pub fn from(collection: &Collection) -> Self {
        Self {
            collection_name: collection.collection_id.clone(),
            n_documents: collection.n_documents,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTypeSimple {
    pub id: String,
    pub vector: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTypeComplex {
    pub id: String,
    pub vector: Vec<f64>,
    pub vector2: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTypeSimpleDocument {
    pub documents: Vec<EntryTypeSimple>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTypeComplexDocument {
    pub documents: Vec<EntryTypeComplex>,
}

#[derive(Serialize, Deserialize)]
pub struct Vector {
    pub vector: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryByID {
    pub id: String,
    pub max_neighbors: usize,
}

#[derive(Serialize, Deserialize)]
pub struct QueryByVec {
    pub vector: Vec<f64>,
    pub max_neighbors: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_owned() }
    }
}
