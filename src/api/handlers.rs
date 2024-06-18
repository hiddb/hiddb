use crate::api::types::*;
use crate::db::dbtypes::*;
use crate::hnsw::key::*;
use crate::hnsw::{Document, IndexBuilder};

use actix_web::{web, HttpResponse};
use serde_json::Value;
use std::sync::Mutex;

use crate::hnsw::index::get_index_hash;
use seahash::hash;

use prometheus::{Encoder, TextEncoder};

use crate::metrics;

use crate::hiddb::{collection, document, index};

// pub async fn check_health() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

pub async fn dummy() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn not_implemented() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

pub async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn get_metrics(state: web::Data<State>) -> HttpResponse {
    // Process Metrics from Rocksdb statistics string
    let metrics_rocksdb = state.db.options.get_statistics().unwrap();
    metrics::rocksdb::process_rocksdb_metrics(metrics_rocksdb);

    // Gather metrics
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output
    return HttpResponse::Ok().body(String::from_utf8(buffer).unwrap());
}

pub async fn get_collections(state: web::Data<State>) -> HttpResponse {
    let collections = match collection::get_all(&state.db) {
        Ok(collections) => collections,
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    };
    let collections: Vec<CollectionResponse> = collections.iter().map(|c| CollectionResponse::from(c)).collect();
    HttpResponse::Ok().json(CollectionsResponse { collections })
}

pub async fn create_collection(item: web::Json<CollectionRequest>, state: web::Data<State>) -> HttpResponse {
    match collection::create(&state.db, &item.collection_name) {
        Ok(collection) => {
            return HttpResponse::Ok().json(CollectionResponse::from(&collection));
        }
        Err(collection::Error::AlreadyExists) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!("collection '{}' already exists.", &item.collection_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn get_collection(path: web::Path<CollectionRequest>, state: web::Data<State>) -> HttpResponse {
    match collection::get(&state.db, &path.collection_name) {
        Ok(collection) => {
            return HttpResponse::Ok().json(CollectionResponse::from(&collection));
        }
        Err(collection::Error::DoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn delete_collection(path: web::Path<CollectionRequest>, state: web::Data<State>) -> HttpResponse {
    match collection::delete(&state.db, &path.collection_name, &state.index_store) {
        Ok(collection) => {
            return HttpResponse::Ok().json(CollectionResponse::from(&collection));
        }
        Err(collection::Error::DoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn create_index(path: web::Path<CollectionRequest>, item: web::Json<CreateIndexRequest>, state: web::Data<State>) -> HttpResponse {
    match index::create(&state.db, &state.index_store, &path.collection_name, &item.field_name, item.dimension) {
        Ok(index) => {
            return HttpResponse::Ok().json(IndexResponse::from_db_type(&index));
        }
        Err(index::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(index::Error::AlreadyExists) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!("index '{}' already exists.", &path.collection_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn get_indices(path: web::Path<CollectionRequest>, state: web::Data<State>) -> HttpResponse {
    let indices = match index::get_all(&state.db, &path.collection_name) {
        Ok(collections) => collections,
        Err(index::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    };
    let indices: Vec<IndexResponse> = indices.iter().map(|i| IndexResponse::from_db_type(i)).collect();
    HttpResponse::Ok().json(IndicesInfo { indices })
}

pub async fn get_index(path: web::Path<IndexRequestPath>, state: web::Data<State>) -> HttpResponse {
    match index::get(&state.db, &path.collection_name, &path.field_name) {
        Ok(index) => {
            return HttpResponse::Ok().json(IndexResponse::from_db_type(&index));
        }
        Err(index::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(index::Error::IndexDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!(
                "no index with field {} in {}",
                path.field_name, path.collection_name
            )));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn delete_index(path: web::Path<IndexRequestPath>, state: web::Data<State>) -> HttpResponse {
    match index::delete(&state.db, &state.index_store, &path.collection_name, &path.field_name) {
        Ok(index) => {
            return HttpResponse::Ok().json(IndexResponse::from_db_type(&index));
        }
        Err(index::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(index::Error::IndexDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!(
                "no index with field {} in {}",
                path.field_name, path.collection_name
            )));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn insert_documents(path: web::Path<CollectionRequest>, body: web::Bytes, state: web::Data<State>) -> HttpResponse {
    let item: Value = match serde_json::from_slice(&body) {
        Ok(body) => body,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid body"));
        }
    };

    let items = match item.get("documents") {
        Some(items) => items,
        _ => return HttpResponse::BadRequest().json(ErrorResponse::new("expected field \"documents\".")),
    };

    let documents = match items.as_array() {
        Some(items) => items,
        _ => return HttpResponse::BadRequest().json(ErrorResponse::new("field \"documents\" should be an array of objects.")),
    };

    match document::insert(&state.db, &state.index_store, &path.collection_name, &documents) {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        }
        Err(document::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(document::Error::MissingFieldId) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("document should have a field named 'id'"));
        }
        Err(document::Error::DimensionsNotEqual {
            field,
            index_dimension,
            vector_dimension,
        }) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!(
                "vector in field '{}' has dimension {} but index has dimension {}",
                &field, vector_dimension, index_dimension,
            )));
        }
        Err(document::Error::InvalidInput) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("invalid input"));
        }
        Err(document::Error::NotImplemented) => {
            return HttpResponse::NotImplemented().json(ErrorResponse::new(""));
        }
        Err(document::Error::AlreadyExists {
            collection_name,
            document_id,
        }) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!(
                "document '{}' already exists in collection '{}'",
                document_id, collection_name,
            )));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn search_documents(path: web::Path<CollectionRequest>, body: web::Bytes, state: web::Data<State>) -> HttpResponse {
    let item: Value = match serde_json::from_slice(&body) {
        Ok(body) => body,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid body"));
        }
    };

    match document::search_ann(&state.db, &state.index_store, &path.collection_name, &item) {
        Ok(data) => HttpResponse::Ok().json(SearchResponse { data }),
        Err(document::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(document::Error::DimensionsNotEqual {
            field,
            index_dimension,
            vector_dimension,
        }) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!(
                "vector in field '{}' has dimension {} but index has dimension {}",
                &field, vector_dimension, index_dimension,
            )));
        }
        Err(document::Error::InvalidInput) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("invalid input"));
        }
        Err(document::Error::IndexDoesNotExist { field_name }) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!("index '{}' does not exist", &field_name)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}

pub async fn get_document_by_id(path: web::Path<DocumentRequestPath>, state: web::Data<State>) -> HttpResponse {
    match document::get_by_id(&state.db, &path.collection_name, &path.document_id) {
        Ok(document) => HttpResponse::Ok().json(document.data),
        Err(document::Error::CollectionDoesNotExist) => {
            return HttpResponse::NotFound().json(ErrorResponse::new(&format!("collection '{}' does not exist.", &path.collection_name)));
        }
        Err(document::Error::DocumentDoesNotExist) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(&format!("document '{}' does not exist.", &path.document_id)));
        }
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::new("")),
    }
}
