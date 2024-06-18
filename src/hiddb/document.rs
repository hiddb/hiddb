use crate::db::RocksDB;

use crate::api::types::*;
use crate::db::dbtypes::*;
use crate::hnsw::key::*;
use crate::hnsw::{Document, IndexBuilder};

use crate::hnsw::index::get_index_hash;

use seahash::hash;

use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum Error {
    AlreadyExists {
        collection_name: String,
        document_id: String,
    },
    InternalError,
    CollectionDoesNotExist,
    IndexDoesNotExist {
        field_name: String,
    },
    DocumentDoesNotExist,
    NotImplemented,
    MissingFieldId,
    InvalidInput,
    DimensionsNotEqual {
        field: String,
        index_dimension: usize,
        vector_dimension: usize,
    },
}

pub fn get_by_id(db: &RocksDB, collection_name: &str, document_id: &str) -> Result<Document, Error> {
    let collection_hash = hash(collection_name.as_bytes()).to_be_bytes();
    let document_hash = hash(document_id.as_bytes()).to_be_bytes();

    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    };

    match db.get_document(&collection_hash, &document_hash) {
        Some(document) => Ok(document),
        _ => return Err(Error::DocumentDoesNotExist),
    }
}

pub fn insert(db: &RocksDB, index_store: &IndexStore, collection_name: &str, documents: &Vec<Value>) -> Result<(), Error> {
    let collection_name = collection_name.clone();
    let collection_hash = hash(&collection_name.as_bytes()).to_be_bytes();

    let index_store = index_store.read().or(Err(Error::InternalError))?;
    let indexed_fields = index_store.values().map(|index| index.lock().unwrap().field_id.clone());

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    };

    // Check if any document already exists in collection
    // TODO: don't do this check and instead update element and don't count size
    for document in documents {
        let document_id = match document["id"].as_str() {
            Some(d) => d.to_owned(),
            _ => return Err(Error::MissingFieldId),
        };
        let document_hash = hash(&document_id.as_bytes()).to_be_bytes();
        match db.get_document(&collection_hash, &document_hash) {
            Some(_) => {
                return Err(Error::AlreadyExists {
                    collection_name: collection_name.to_owned(),
                    document_id,
                });
            }
            _ => {
                // let document_id = document["id"].as_str().unwrap().to_owned();
                let entry = Document::new(document_id, document.clone());
                db.insert_document(&collection_hash, &entry.id_hash, &entry)
                    .or(Err(Error::InternalError))?;

                let mut collection = db.get_collection(&collection_hash).unwrap();
                collection.n_documents += 1;
                db.insert_collection(&collection_hash, &collection).unwrap();
            }
        };
    }

    for field_id in indexed_fields {
        for document in documents.iter() {
            match document.get(&field_id) {
                None => continue,
                Some(_) => {
                    let field_hash = hash(&field_id.as_bytes()).to_be_bytes();
                    let index_hash = get_index_hash(collection_hash, field_hash);

                    match index_store.get(&index_hash) {
                        Some(index) => {
                            let mut index = index.lock().unwrap();
                            let vector = match document[&field_id].as_array() {
                                Some(vec) => vec,
                                _ => return Err(Error::InvalidInput),
                            };
                            let vector: Vec<f64> = vector
                                .iter()
                                .map(|v| v.as_f64().ok_or(Error::InvalidInput))
                                .collect::<Result<Vec<f64>, Error>>()?;

                            if index.dimension != vector.len() {
                                return Err(Error::DimensionsNotEqual {
                                    field: field_id,
                                    index_dimension: index.dimension,
                                    vector_dimension: vector.len(),
                                });
                            }

                            let document_id = match document["id"].as_str() {
                                Some(doc_id) => doc_id.to_owned(),
                                _ => return Err(Error::InvalidInput),
                            };
                            let entry = Document::new(document_id, document.clone());

                            index.insert(&db, &entry);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn search_ann(db: &RocksDB, index_store: &IndexStore, collection_name: &str, item: &Value) -> Result<Vec<Vec<String>>, Error> {
    let field_id = match item.get("field_name") {
        Some(field_id) => match field_id.as_str() {
            Some(f) => f,
            _ => return Err(Error::InvalidInput),
        },
        _ => return Err(Error::InvalidInput),
    };
    let collection_id = collection_name;
    let field_hash = hash(&field_id.as_bytes()).to_be_bytes();
    let collection_hash = hash(&collection_id.as_bytes()).to_be_bytes();
    let index_hash = get_index_hash(collection_hash, field_hash);

    // Only possible if collection exists
    match db.get_collection(&collection_hash) {
        Some(_) => {}
        _ => return Err(Error::CollectionDoesNotExist),
    }

    let index_store = index_store.read().or(Err(Error::InternalError))?;
    let index = index_store.get(&index_hash);
    match index {
        Some(index) => {
            let max_neighbors = match item.get("max_neighbors") {
                Some(max_neighbors) => match max_neighbors.as_u64() {
                    Some(max_n) => max_n as usize,
                    _ => return Err(Error::InvalidInput),
                },
                _ => 20 as usize,
            };

            let index = index.lock().or(Err(Error::InternalError))?;
            let data: Vec<Vec<[u8; 8]>> = match (item.get("vectors"), item.get("ids")) {
                (Some(vectors), _) => {
                    let vectors: Vec<Vec<f64>> = match vectors.as_array() {
                        Some(vecs) => vecs
                            .iter()
                            .map(|vector| match vector.as_array() {
                                Some(vec) => vec
                                    .iter()
                                    .map(|x| x.as_f64().ok_or(Error::InvalidInput))
                                    .collect::<Result<Vec<f64>, Error>>(),
                                _ => return Err(Error::InvalidInput),
                            })
                            .collect::<Result<Vec<Vec<f64>>, Error>>()?,
                        _ => return Err(Error::InvalidInput),
                    };

                    for vector in vectors.iter() {
                        if index.dimension != vector.len() {
                            return Err(Error::DimensionsNotEqual {
                                field: field_id.to_owned(),
                                index_dimension: index.dimension,
                                vector_dimension: vector.len(),
                            });
                        }
                    }
                    vectors.iter().map(|vector| index.knn_search(db, &vector, max_neighbors)).collect()
                }
                (_, Some(ids)) => {
                    let mut data = Vec::new();

                    let ids = match ids.as_array() {
                        Some(i) => i,
                        _ => return Err(Error::InvalidInput),
                    };
                    for id in ids.iter() {
                        let id_user = match id.as_str() {
                            Some(id) => id.as_bytes(),
                            _ => return Err(Error::InvalidInput),
                        };
                        let id_hash = hash(id_user).to_be_bytes();

                        let vector = match db.get_document(&collection_hash, &id_hash) {
                            Some(document) => document.get_field_vector(&index.field_id).clone(),
                            _ => {
                                return Err(Error::InvalidInput);
                            }
                        };
                        data.push(index.knn_search(&db, &vector, max_neighbors));
                    }
                    data
                }
                (_, _) => {
                    return Err(Error::InvalidInput);
                }
            };

            // Get id_user from id_hash
            // TODO: do this more efficiently!
            let data: Vec<Vec<String>> = data
                .iter()
                .map(|knn| {
                    knn.iter()
                        .map(|id_hash| db.get_document(&collection_hash, &id_hash).unwrap().id_user.clone())
                        .collect()
                })
                .collect();
            return Ok(data);
        }
        _ => {
            return Err(Error::IndexDoesNotExist {
                field_name: field_id.to_owned(),
            });
        }
    }
}

// pub fn delete(db: &RocksDB) -> Result<Value, Error> {}
