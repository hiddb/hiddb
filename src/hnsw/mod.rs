use rand::rngs::StdRng;

use serde::{Deserialize, Serialize};

use serde_json::Value;

pub mod builder;
pub mod document;
pub mod index;
pub mod key;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    pub id_user: String,
    pub id_hash: [u8; 8],

    pub data: Value,
}

#[derive(Debug)]
pub struct Index {
    pub collection_id: String,
    pub field_id: String,

    pub collection_hash: [u8; 8],
    pub field_hash: [u8; 8],
    pub index_hash: [u8; 16],

    pub distance_metric: String,
    pub buffer_size: usize,
    pub dimension: usize,
    pub k: usize,
    pub entry_point: Option<[u8; 8]>,

    // pub document_map: Vec<HashMap<u64, Document>>, // index corresponds to level
    // pub neighbor_map: Vec<HashMap<u64, SortedList<f64, u64>>>,
    pub rng: StdRng,

    // parameter for number of layers
    // influences exponential propability
    // for element insertion in layer
    pub reverse_size: f64,
    pub n_layers: u8, // number of layers
    pub n_elements: u64,
}

pub struct IndexBuilder {
    collection_id: String,
    field_id: String,
    distance_metric: String,
    buffer_size: usize,
    dimension: usize,
    k: usize,

    seed: u64,

    m: f64,
}
