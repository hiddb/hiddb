use actix_web::{web, App};

use std::sync::RwLock;

use std::collections::HashMap;

use crate::api::handlers::*;
use crate::api::types::*;
use crate::db::RocksDB;

use actix_web::{http, test, Error};

use rand::distributions::Uniform;
use rand::prelude::*;

use actix_web::dev::Service;
use serde::{Deserialize, Serialize};

#[actix_rt::test]
async fn test_index() -> Result<(), Error> {
    let db_options;
    {
        let db = RocksDB::init("./build/test_index.rdb");
        db_options = db.options.clone();
        let state = web::Data::new(State {
            index_store: RwLock::new(HashMap::new()),
            db,
        });
        let mut app = test::init_service(
            App::new()
                .data(web::JsonConfig::default().limit(1024 * 1024))
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
                .route("/collection/{collection_name}/document/{document_id}", web::get().to(get_document_by_id)), // Get document by ID
        )
        .await;

        let req = test::TestRequest::with_uri("/health?collection_id=lala").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Check health
        let req = test::TestRequest::with_uri("/health").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        #[derive(Debug, Serialize, Deserialize)]
        struct CreateCollectionParameters {
            collection_name: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct CreateIndexParameters {
            field_name: String,
            dimension: usize,
            k: usize,
        }

        // Create collection "collection1"
        let req = test::TestRequest::post()
            .uri("/collection")
            .set_json(&CreateCollectionParameters {
                collection_name: "collection1".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        // let response_body = match resp.response().body().as_ref() {
        //     Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        //     _ => panic!("Response error"),
        // };
        // println!("{:?}", response_body);
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Check if collection was created
        let req = test::TestRequest::get().uri("/collection/collection1").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: CollectionResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.collection_name, "collection1");

        let req = test::TestRequest::get().uri("/collection").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: CollectionsResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.collections.len(), 1);
        // assert_eq!(index_info_json.collections, "collection1");

        let req = test::TestRequest::delete().uri("/collection/collection1").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: CollectionResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.collection_name, "collection1");

        // Create collection "collection1"
        let req = test::TestRequest::post()
            .uri("/collection")
            .set_json(&CreateCollectionParameters {
                collection_name: "collection1".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Create index for "vector"
        let req = test::TestRequest::post()
            .uri("/collection/collection1/index")
            .set_json(&CreateIndexParameters {
                field_name: "vector".to_owned(),
                dimension: 50,
                k: 10,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        // let response_body = match resp.response().body().as_ref() {
        //     Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        //     _ => panic!("Response error"),
        // };
        // println!("{:?}", response_body);
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Create index for "vector2"
        let req = test::TestRequest::post()
            .uri("/collection/collection1/index")
            .set_json(&CreateIndexParameters {
                field_name: "vector2".to_owned(),
                dimension: 5,
                k: 20,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Check if indices are created
        let req = test::TestRequest::get().uri("/collection/collection1/index").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: IndicesInfo = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.indices.len(), 2);

        // Insert to index "vector"
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100.0, 100.0);
        for id in 0..100 {
            let vector: Vec<f64> = (0..50).map(|_| rng.sample(range)).collect();
            let req = test::TestRequest::post()
                .uri("/collection/collection1/document")
                .set_json(&EntryTypeSimpleDocument {
                    documents: vec![EntryTypeSimple {
                        id: format!("single_vector_doc_{}", id),
                        vector: vector,
                    }],
                })
                .to_request();
            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        // Get information about index "vector"
        let req = test::TestRequest::get().uri("/collection/collection1/index/vector").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: IndexResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.n_documents, 100);

        // Insert to index "vector" and "vector2"
        for id in 0..10 {
            let vector: Vec<f64> = (0..50).map(|idx| (idx * id) as f64).collect();
            let vector2: Vec<f64> = (0..5).map(|idx| 2.0 * (idx as f64) * (id as f64).sqrt()).collect();
            let req = test::TestRequest::post()
                .uri("/collection/collection1/document")
                .set_json(&EntryTypeComplexDocument {
                    documents: vec![EntryTypeComplex {
                        id: format!("multiple_vector_doc_{}", id),
                        vector,
                        vector2,
                    }],
                })
                .to_request();
            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        // Insert multiple
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100.0, 100.0);
        for id in 0..100 {
            let vector: Vec<f64> = (0..50).map(|_| rng.sample(range)).collect();
            let req = test::TestRequest::post()
                .uri("/collection/collection1/document")
                .set_json(&EntryTypeSimpleDocument {
                    documents: vec![
                        EntryTypeSimple {
                            id: format!("single_vector_doc_{}", id + 100),
                            vector: vector.clone(),
                        },
                        EntryTypeSimple {
                            id: format!("single_vector_doc_{}", id + 200),
                            vector: vector,
                        },
                    ],
                })
                .to_request();
            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), http::StatusCode::OK);
        }

        // ANN search

        #[derive(Debug, Serialize, Deserialize)]
        struct SearchANN {
            vectors: Vec<Vec<f64>>,
            max_neighbors: u64,
            field_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SearchANNResponse {
            data: Vec<Vec<String>>,
        }

        let req = test::TestRequest::post()
            .uri("/collection/collection1/document/search")
            .set_json(&SearchANN {
                vectors: vec![vec![0.0; 50]],
                max_neighbors: 10,
                field_name: "vector".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let index_info_json: SearchANNResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        assert_eq!(index_info_json.data[0].len(), 10);

        let req = test::TestRequest::get()
            .uri("/collection/collection1/document/multiple_vector_doc_0")
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DocumentResponse {
            pub id: String,
            pub vector: Vec<f64>,
            pub vector2: Vec<f64>,
        }

        let document_by_id: DocumentResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();
        println!("{:?}", document_by_id);

        // Create collection "collection3"
        let req = test::TestRequest::post()
            .uri("/collection")
            .set_json(&CreateCollectionParameters {
                collection_name: "collection3".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Create index for "dummy"
        let req = test::TestRequest::post()
            .uri("/collection/collection3/index")
            .set_json(&CreateIndexParameters {
                field_name: "dummy".to_owned(),
                dimension: 5,
                k: 20,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let _index_info_json: IndexResponse = serde_json::from_str(std::str::from_utf8(response_body).unwrap()).unwrap();

        let req = test::TestRequest::delete().uri("/collection/collection3/index/dummy").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    RocksDB::destroy(&db_options, "test_index.rdb");
    Ok(())
}
