use actix_web::{web, App};

use std::sync::RwLock;

use std::collections::HashMap;

use crate::api::handlers::*;
use crate::api::types::*;
use crate::db::RocksDB;

use actix_web::{test, Error};

use crate::tests::helpers;

#[actix_rt::test]
pub async fn test_collection() -> Result<(), Error> {
    let db_name = "./build/collection_test.rdb";
    let db_options;
    let result = {
        let db = RocksDB::init(db_name);
        db_options = db.options.clone();
        let state = web::Data::new(State {
            index_store: RwLock::new(HashMap::new()),
            db,
        });
        let mut app = test::init_service(
            App::new()
                .data(web::JsonConfig::default().limit(1024 * 1024))
                .app_data(state.clone())
                // /collection
                .route("/collection", web::get().to(get_collections)) // Get information about collections
                .route("/collection", web::post().to(create_collection)) // Create new collection
                //
                // /collection/{collection_id}
                .route("/collection/{collection_name}", web::get().to(get_collection)) // Get information about collection
                .route("/collection/{collection_name}", web::delete().to(delete_collection)), // Delete collection
        )
        .await;

        let collection_name = "test_collection";

        // Create
        let collection_response: CollectionResponse = helpers::create_collection(&mut app, collection_name).await;
        assert_eq!(collection_response.collection_name, collection_name);

        // List
        let collections_response: CollectionsResponse = helpers::get_collections(&mut app).await;
        assert_eq!(collections_response.collections.len(), 1);
        assert_eq!(collections_response.collections[0].collection_name, collection_name);

        // TODO: Create -> should not work
        // let collection_response: CollectionResponse = helpers::create_collection(&mut app, collection_name).await;
        // assert_eq!(collection_response.collection_name, collection_name);
    };
    RocksDB::destroy(&db_options, db_name);
    Ok(())
}
