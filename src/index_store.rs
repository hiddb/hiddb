use crate::api::handlers::*;
use crate::api::types::*;

use crate::api::types::*;
use crate::db::dbtypes::*;
use crate::db::*;
use crate::hnsw::index::get_index_hash;
use crate::hnsw::key::*;
use crate::hnsw::Index;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;

pub fn init(db: &RocksDB) -> IndexStore {
    // Get indices from database
    let indices = db.get_by_prefix_key_value("default", &Prefix::new().prefix_type(INDEX).finish()).unwrap();
    let mut index_hashmap: HashMap<[u8; 16], Mutex<Index>> = HashMap::new();
    for (key, index) in indices.iter() {
        let key = get_index_hash(key.get_collection_id(), key.get_field_id());
        let index = IndexDB::from_binary(index).to_hnsw_type();
        match index_hashmap.insert(key, Mutex::new(index)) {
            // index_hashmap should initially be empty!
            Some(_) => panic!(),
            _ => {}
        }
    }
    return RwLock::new(index_hashmap);
}
