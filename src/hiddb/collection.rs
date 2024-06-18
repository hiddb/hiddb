use crate::api::types::*;
use crate::db::dbtypes::*;
use crate::hnsw::key::*;

use crate::db::RocksDB;

use seahash::hash;

use crate::hnsw::index::get_index_hash;

#[derive(Debug, PartialEq)]
pub enum Error {
    AlreadyExists,
    InternalError,
    DoesNotExist,
}

pub fn get_all(db: &RocksDB) -> Result<Vec<Collection>, Error> {
    let prefix = Prefix::new().prefix_type(COLLECTION).finish();
    let collections = db.get_by_prefix("default", &prefix).or(Err(Error::InternalError))?;
    let collections: Vec<Collection> = collections
        .iter()
        .map(|c| {
            let collection = Collection::from_binary(c);
            let collection_hash = hash(collection.collection_id.as_bytes()).to_be_bytes();

            db.get_collection(&collection_hash).unwrap()
        })
        .collect();
    Ok(collections)
}

pub fn create(db: &RocksDB, name: &str) -> Result<Collection, Error> {
    let collection = Collection::new(name);
    let collection_hash = hash(name.as_bytes()).to_be_bytes();
    match db.get_collection(&collection_hash) {
        Some(_) => {
            return Err(Error::AlreadyExists);
        }
        _ => {}
    };
    db.insert_collection(&collection_hash, &collection).unwrap();

    Ok(collection)
}

pub fn get(db: &RocksDB, name: &str) -> Result<Collection, Error> {
    // TODO: include more info like n_indices, createdAt, deletedAt, ...
    let collection_id = name;
    let collection_hash = hash(collection_id.as_bytes()).to_be_bytes();

    match db.get_collection(&collection_hash) {
        Some(collection) => Ok(collection),
        _ => return Err(Error::DoesNotExist),
    }
}

pub fn delete(db: &RocksDB, name: &str, index_store: &IndexStore) -> Result<Collection, Error> {
    let collection_hash = hash(name.as_bytes()).to_be_bytes();
    let collection = match db.get_collection(&collection_hash) {
        Some(collection) => collection,
        _ => return Err(Error::DoesNotExist),
    };

    // Delete associated indices from index_store
    let indices = db
        .get_by_prefix_key_value("default", &Prefix::new().prefix_type(INDEX).collection(&collection_hash).finish())
        .unwrap();
    for (key, _) in indices.iter() {
        let key = get_index_hash(key.get_collection_id(), key.get_field_id());
        index_store.write().unwrap().remove(&key).unwrap();
    }

    // Delete associated indices from db
    db.delete_by_prefix("default", &Prefix::new().prefix_type(INDEX).collection(&collection_hash).finish())
        .unwrap();

    // Delete associated documents from db
    db.delete_by_prefix("default", &Prefix::new().prefix_type(DOCUMENT).collection(&collection_hash).finish())
        .unwrap();

    match db.delete_collection(&collection_hash) {
        Ok(_) => Ok(collection),
        _ => Err(Error::InternalError),
    }
}
