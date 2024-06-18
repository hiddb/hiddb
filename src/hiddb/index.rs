use crate::db::RocksDB;

use crate::api::types::*;
use crate::db::dbtypes::*;
use crate::hnsw::key::*;

use crate::hnsw::index::get_index_hash;

use seahash::hash;

use crate::hnsw::key::*;
use crate::hnsw::{Document, IndexBuilder};
use std::sync::{Mutex, RwLock};

#[derive(Debug, PartialEq)]
pub enum Error {
    AlreadyExists,
    InternalError,
    CollectionDoesNotExist,
    IndexDoesNotExist,
    NotImplemented,
}

pub fn get_all(db: &RocksDB, collection_name: &str) -> Result<Vec<IndexDB>, Error> {
    let collection_id = collection_name;
    let collection_hash = hash(collection_id.as_bytes()).to_be_bytes();

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    }

    let indices = db.get_indices_in_collection(&collection_hash).or(Err(Error::InternalError))?;
    let indices: Vec<IndexDB> = indices.iter().map(|c| IndexDB::from_binary(c)).collect();

    Ok(indices)
}

pub fn create(db: &RocksDB, index_store: &IndexStore, collection_name: &str, field_name: &str, dimension: usize) -> Result<IndexDB, Error> {
    let collection_id = collection_name;
    let field_id = field_name;

    let collection_hash = hash(collection_id.as_bytes()).to_be_bytes();
    let field_hash = hash(field_id.as_bytes()).to_be_bytes();
    let index_hash = get_index_hash(collection_hash, field_hash);

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    }
    // Creation of indices only possible if no documents present
    // TODO: implement this feature
    let prefix = Prefix::new().prefix_type(DOCUMENT).collection(&collection_hash).finish();
    let documents = db.get_by_prefix("default", &prefix).or(Err(Error::InternalError))?;
    if documents.len() > 0 {
        return Err(Error::NotImplemented);
    }

    match db.get_index(&collection_hash, &field_hash) {
        Some(_) => return Err(Error::AlreadyExists),
        _ => {
            match index_store.read().or(Err(Error::InternalError))?.get(&index_hash) {
                Some(_) => return Err(Error::InternalError),
                _ => {}
            };
        }
    };

    let index = IndexBuilder::new()
        .set_collection(collection_id)
        .set_field(field_id)
        .set_dimension(dimension)
        .build();

    db.insert_index(&collection_hash, &field_hash, &index).or(Err(Error::InternalError))?;
    let index = Mutex::new(index);
    match index_store.write().or(Err(Error::InternalError))?.insert(index_hash, index) {
        Some(_) => Err(Error::InternalError),
        _ => {
            let index = db.get_index(&collection_hash, &field_hash).unwrap();
            Ok(IndexDB::from_hnsw_type(&index))
        }
    }
}

pub fn get(db: &RocksDB, collection_name: &str, field_name: &str) -> Result<IndexDB, Error> {
    let collection_id = collection_name;
    let field_id = field_name;

    let collection_hash = hash(collection_id.as_bytes()).to_be_bytes();
    let field_hash = hash(field_id.as_bytes()).to_be_bytes();

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    }

    match db.get_index(&collection_hash, &field_hash) {
        Some(index) => {
            return Ok(IndexDB::from_hnsw_type(&index));
        }
        _ => return Err(Error::IndexDoesNotExist),
    }
}

pub fn delete(db: &RocksDB, index_store: &IndexStore, collection_name: &str, field_name: &str) -> Result<IndexDB, Error> {
    let collection_id = collection_name;
    let collection_hash = hash(collection_id.as_bytes()).to_be_bytes();
    let field_hash = hash(field_name.as_bytes()).to_be_bytes();
    let index_hash = get_index_hash(collection_hash, field_hash);

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    }

    match db.get_index(&collection_hash, &field_hash) {
        Some(index) => {
            // TODO: create collections and fields
            db.delete_index(&collection_hash, &field_hash).or(Err(Error::InternalError))?;

            let mut index_store = index_store.write().or(Err(Error::InternalError))?;
            index_store.remove(&index_hash);
            Ok(IndexDB::from_hnsw_type(&index))
        }
        _ => Err(Error::IndexDoesNotExist),
    }
}
