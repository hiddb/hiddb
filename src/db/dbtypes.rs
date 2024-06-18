use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::sorted_list::SortedList;

use crate::hnsw;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    pub collection_id: String,
    pub n_documents: usize,
}

impl Collection {
    pub fn new(collection_id: &str) -> Self {
        Self {
            collection_id: collection_id.to_owned(),
            n_documents: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexDB {
    pub field_id: String,      // here vector
    pub collection_id: String, // here vector
    pub metric: String,
    pub buffer_size: usize,
    pub dimension: usize,
    pub k: usize,
    pub entry_point: Option<[u8; 8]>,
    pub reverse_size: f64,
    pub n_layers: u8, // number of layers
    pub n_elements: u64,
}

impl IndexDB {
    pub fn to_hnsw_type(self) -> hnsw::Index {
        let mut index = hnsw::IndexBuilder::new()
            .set_collection(&self.collection_id)
            .set_field(&self.field_id)
            .set_distance_metric(&self.metric)
            .set_buffer_size(self.buffer_size)
            .set_dimension(self.dimension)
            .set_k(self.k)
            .build();
        index.entry_point = self.entry_point;
        index.reverse_size = self.reverse_size;
        index.n_layers = self.n_layers;
        index.n_elements = self.n_elements;

        index
    }
    pub fn from_hnsw_type(index: &hnsw::Index) -> Self {
        Self {
            field_id: index.field_id.clone(),
            collection_id: index.collection_id.clone(),
            metric: index.distance_metric.clone(),
            buffer_size: index.buffer_size,
            dimension: index.dimension,
            k: index.k,
            entry_point: index.entry_point,
            reverse_size: index.reverse_size,
            n_layers: index.n_layers,
            n_elements: index.n_elements,
        }
    }
}

pub trait BinaryConverison {
    fn to_binary(&self) -> Vec<u8>;
    fn from_binary(data: &Vec<u8>) -> Self;
}

// TODO: implement and benchmark own implementation
// TODO: Write costum derive macro?
macro_rules! implement_BinaryConversion {
    (for $($t:ty),+) => {
        $(impl BinaryConverison for $t {
            fn to_binary(&self) -> Vec<u8> {
                serialize(self).unwrap()
            }

            fn from_binary(data: &Vec<u8>) -> Self {
                deserialize(data).unwrap()
            }
        })*
    }
}

implement_BinaryConversion!(for Collection, IndexDB, SortedList<f64, [u8; 8]>, Vec<f64>);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_conversion() {
        let collection = Collection {
            collection_id: "test_collection".to_owned(),
            n_documents: 0,
        };

        let encoded: Vec<u8> = bincode::serialize(&collection).unwrap();
        let decoded: Collection = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(collection.collection_id, decoded.collection_id);
    }
}
