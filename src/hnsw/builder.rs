use crate::hnsw::Index;
use crate::hnsw::IndexBuilder;

use seahash::hash;

use crate::hnsw::index::get_index_hash;
use rand::prelude::*;
use rand::rngs::StdRng;

impl IndexBuilder {
    pub fn new() -> Self {
        Self {
            collection_id: String::new(),
            field_id: String::new(),
            distance_metric: String::new(),
            buffer_size: 0,
            dimension: 0,
            k: 16,                              // number of nearest neighbors to save
            seed: StdRng::from_entropy().gen(), // random seed

            m: 2.0,
        }
    }
    pub fn set_collection(mut self, collection_id: &str) -> Self {
        self.collection_id = collection_id.to_owned();
        self
    }

    pub fn set_field(mut self, field_id: &str) -> Self {
        self.field_id = field_id.to_owned();
        self
    }

    pub fn set_distance_metric(mut self, distance_metric: &str) -> Self {
        self.distance_metric = distance_metric.to_owned();
        self
    }

    pub fn set_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn set_dimension(mut self, dimension: usize) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn set_k(mut self, k: usize) -> Self {
        self.k = k;
        self
    }

    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    // from https://github.com/nmslib/hnswlib/blob/master/ALGO_PARAMS.md
    // Should be between 2.0 and 100.0
    pub fn set_m(mut self, m: f64) -> Self {
        self.m = m;
        self
    }

    pub fn build(self) -> Index {
        let collection_hash = hash(self.collection_id.as_bytes()).to_be_bytes();
        let field_hash = hash(self.field_id.as_bytes()).to_be_bytes();
        let index_hash = get_index_hash(collection_hash, field_hash);

        Index {
            collection_id: self.collection_id.to_owned(),
            field_id: self.field_id,
            collection_hash,
            field_hash,
            index_hash,
            distance_metric: self.distance_metric,
            buffer_size: self.buffer_size,
            dimension: self.dimension,
            k: self.k,
            entry_point: None,

            rng: StdRng::seed_from_u64(self.seed),
            reverse_size: 1.0 / (1.0 / self.m.ln()),
            n_layers: 1,
            n_elements: 0,
        }
    }
}
