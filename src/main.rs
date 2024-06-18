mod api;
mod db;
mod distance;
mod hiddb;
mod hnsw;
mod index_store;
mod metrics;
mod reverse_sorted_list;
mod sorted_list;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use api::handlers::*;
use api::types::*;

use std::env;

use metrics::middleware::Metrics;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_path = match env::var("HIDDB_PATH") {
        Ok(path) => path,
        Err(_) => "HIDDBrocksdb".to_owned(),
    };

    let bind: String = match env::var("HIDDB_LISTEN") {
        Ok(b) => b,
        Err(_) => "127.0.0.1:8080".to_owned(),
    };

    let db = db::RocksDB::init(&db_path);

    let state = web::Data::new(State {
        index_store: index_store::init(&db),
        db,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(Metrics::new())
            .wrap(Cors::permissive()) // TODO: use default() and make allow stuff
            .data(web::JsonConfig::default().limit(131072))
            .app_data(state.clone())
            .route("/health", web::get().to(check_health))
            .route("/metrics", web::get().to(get_metrics))
            //
            // /collection
            .route("/collection", web::get().to(get_collections)) // Get information about collections
            .route("/collection", web::post().to(create_collection)) // Create new collection
            //
            // /collection/{collection_id}
            .route("/collection/{collection_name}", web::get().to(get_collection)) // Get information about collection
            .route("/collection/{collection_name}", web::delete().to(delete_collection)) // Delete collection
            //
            // /collection/{collection_id}/index
            .route("/collection/{collection_name}/index", web::get().to(get_indices)) // Get information about existing indices
            .route("/collection/{collection_name}/index", web::post().to(create_index)) // Create new index in {collection_id}
            //
            // /collection/{collection_id}/index/{index_id}
            .route("/collection/{collection_name}/index/{field_name}", web::get().to(get_index)) // Get information about specific index
            .route("/collection/{collection_name}/index/{field_name}", web::delete().to(delete_index)) // Delete index
            //
            // /collection/{collection_id}/document
            .route("/collection/{collection_name}/document/search", web::post().to(search_documents)) // Search for document. Supply at least "field" and "document_id"
            .route("/collection/{collection_name}/document", web::post().to(insert_documents))
            // Insert documents. The field "field_id" will be indexed by all existing indices.
            .route("/collection/{collection_name}/document/{document_id}", web::delete().to(not_implemented)) // Remove document. Indices will be updated
            .route("/collection/{collection_name}/document/{document_id}", web::get().to(get_document_by_id))
        // Get document by ID
    })
    .bind(&bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests;
