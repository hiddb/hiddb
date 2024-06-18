pub mod collection;
pub mod document;
pub mod index;

#[cfg(test)]
mod tests {

    use crate::db::dbtypes::*;
    use crate::distance;
    use crate::hnsw::IndexBuilder;
    use crate::index_store;
    use rand::distributions::Uniform;
    use rand::prelude::*;

    use crate::db::RocksDB;
    use crate::hnsw::document::*;
    use crate::hnsw::*;
    use serde_json::*;

    use crate::hiddb;
    use seahash::hash;

    #[test]
    fn test_search_and_insert_small_scale() {
        let db_options;
        {
            let db = &RocksDB::init("./build/small_scale.rdb");
            db_options = db.options.clone();

            let index_store = index_store::init(db);

            let collection_name = "test_collection".to_owned();
            let field_name = "vector";

            let test_collection = Collection {
                collection_id: collection_name.clone(),
                n_documents: 0,
            };

            // First insert should work
            assert_eq!(
                hiddb::collection::create(db, &collection_name).unwrap().collection_id,
                test_collection.collection_id
            );

            // Second insert should fail
            assert_eq!(
                hiddb::collection::create(db, &collection_name),
                Err(hiddb::collection::Error::AlreadyExists)
            );

            assert_eq!(
                hiddb::index::create(db, &index_store, "collection_does_not_exist", field_name, 3),
                Err(hiddb::index::Error::CollectionDoesNotExist)
            );

            let index = hiddb::index::create(db, &index_store, &collection_name, field_name, 3).unwrap();

            assert_eq!(
                hiddb::index::create(db, &index_store, &collection_name, field_name, 3),
                Err(hiddb::index::Error::AlreadyExists)
            );

            assert_eq!(index.field_id, field_name.to_owned());
            assert_eq!(index.collection_id, collection_name);
            assert_eq!(index.dimension, 3);
            assert_eq!(index.entry_point, None);

            // 1st insert
            let document = json!({"id": "1", "vector": vec![1.0f64, 2.0f64, 3.0f64]});

            hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();

            assert_eq!(
                hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]),
                Err(hiddb::document::Error::AlreadyExists {
                    collection_name: test_collection.collection_id.clone(),
                    document_id: document["id"].as_str().unwrap().to_owned()
                })
            );

            let hnsw_index: IndexDB = hiddb::index::get(db, &test_collection.collection_id, field_name).unwrap();
            let d_1: Document = hiddb::document::get_by_id(db, &test_collection.collection_id, "1").unwrap();

            // First element will always be inserted into first layer
            assert_eq!(hnsw_index.entry_point, Some(d_1.id_hash));
            assert_eq!(hnsw_index.n_layers, 1);
            assert_eq!(hnsw_index.n_elements, 1);
            assert_eq!(&d_1.data, &document);

            // 2st insert
            let document = json!({"id": "2", "vector": vec![1.0f64, 2.0f64, 5.0f64]});

            hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();

            assert_eq!(
                hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]),
                Err(hiddb::document::Error::AlreadyExists {
                    collection_name: test_collection.collection_id.clone(),
                    document_id: document["id"].as_str().unwrap().to_owned()
                })
            );

            let hnsw_index: IndexDB = hiddb::index::get(db, &test_collection.collection_id, field_name).unwrap();
            let d_2: Document = hiddb::document::get_by_id(db, &test_collection.collection_id, "2").unwrap();

            assert!(hnsw_index.entry_point == Some(d_1.id_hash) || hnsw_index.entry_point == Some(d_2.id_hash));
            assert_eq!(hnsw_index.n_elements, 2);
            assert_eq!(&d_2.data, &document);

            // 3st insert
            let document = json!({"id": "3", "vector": vec![2.0f64, 1.9f64, 5.0f64]});

            hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();

            let hnsw_index: IndexDB = hiddb::index::get(db, &test_collection.collection_id, field_name).unwrap();
            let d_3: Document = hiddb::document::get_by_id(db, &test_collection.collection_id, "3").unwrap();

            assert_eq!(hnsw_index.n_elements, 3);
            assert_eq!(&d_3.data, &document);

            // 4st insert
            let document = json!({"id": "4", "vector": vec![1.2f64, 5.0f64, 3.0f64]});

            hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();

            let hnsw_index: IndexDB = hiddb::index::get(db, &test_collection.collection_id, field_name).unwrap();
            let d_4: Document = hiddb::document::get_by_id(db, &test_collection.collection_id, "4").unwrap();

            assert_eq!(hnsw_index.n_elements, 4);
            assert_eq!(&d_4.data, &document);

            assert_eq!(
                hiddb::document::search_ann(
                    &db,
                    &index_store,
                    &collection_name,
                    &json!({"field_name": "vector", "vectors": vec![vec![0.0f64, 0.0f64, 0.0f64]]})
                )
                .unwrap(),
                vec![vec![d_1.id_user.clone(), d_2.id_user.clone(), d_3.id_user.clone(), d_4.id_user.clone()]]
            );

            assert_eq!(
                hiddb::document::search_ann(
                    &db,
                    &index_store,
                    &collection_name,
                    &json!({"field_name": "vector", "vectors": vec![vec![1.3f64, 5.1f64, 2.9f64]]})
                )
                .unwrap(),
                vec![vec![d_4.id_user, d_1.id_user, d_2.id_user, d_3.id_user]]
            );
        }
        RocksDB::destroy(&db_options, "./build/small_scale.rdb");
    }

    #[test]
    fn test_search_and_insert_large_scale_1() {
        let db_options;
        {
            let db = &RocksDB::init("./build/large_scale_1.rdb");
            db_options = db.options.clone();

            let index_store = index_store::init(db);

            let collection_name = "test_collection".to_owned();
            let field_name = "vector";

            let test_collection = Collection {
                collection_id: collection_name.clone(),
                n_documents: 0,
            };

            assert_eq!(
                hiddb::collection::create(db, &collection_name).unwrap().collection_id,
                test_collection.collection_id
            );

            hiddb::index::create(db, &index_store, &collection_name, field_name, 200).unwrap();

            let mut rng = rand::thread_rng();
            for idx in 0..100 {
                let range = Uniform::new(-100.0, 100.0);

                let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();

                let document = json!({"id": idx.to_string(), "vector": vector});
                hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();
            }

            let range = Uniform::new(-100.0, 100.0);
            let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();

            // Check if consecutive in-layer search yields better and better results

            let hnsw_index: Index = hiddb::index::get(db, &collection_name, field_name).unwrap().to_hnsw_type();
            let entry_point = hnsw_index.entry_point.unwrap();

            let collection_hash = hash(collection_name.as_bytes()).to_be_bytes();
            let distance_to_vector_initial = distance::euclidean(
                &vector,
                &db.get_document(&collection_hash, &entry_point)
                    .unwrap()
                    .get_field_vector(&hnsw_index.field_id),
            );

            let mut entry_point_document_new = db.get_document(&collection_hash, &entry_point).unwrap();

            for level_idx in (0..hnsw_index.n_layers).rev() {
                let new_entry_point_id = hnsw_index
                    .search_level(&db, &vector, level_idx, &entry_point_document_new.id_hash)
                    .first()
                    .1;
                entry_point_document_new = db.get_document(&hnsw_index.collection_hash, &new_entry_point_id).unwrap();

                let distance_new = distance::euclidean(
                    &vector,
                    &db.get_document(&hnsw_index.collection_hash, &new_entry_point_id)
                        .unwrap()
                        .get_field_vector(&hnsw_index.field_id),
                );
                assert!(distance_new <= distance_to_vector_initial);
            }
        }
        RocksDB::destroy(&db_options, "./build/large_scale_1.rdb");
    }

    #[test]
    fn test_search_and_insert_large_scale_2() {
        let db_options;
        {
            let db = &RocksDB::init("./build/large_scale_2.rdb");
            db_options = db.options.clone();

            let index_store = index_store::init(db);

            let collection_name = "test_collection".to_owned();
            let field_name = "vector";

            let test_collection = Collection {
                collection_id: collection_name.clone(),
                n_documents: 0,
            };

            assert_eq!(
                hiddb::collection::create(db, &collection_name).unwrap().collection_id,
                test_collection.collection_id
            );

            hiddb::index::create(db, &index_store, &collection_name, field_name, 200).unwrap();
            let mut rng = rand::thread_rng();
            for idx in 0..100 {
                let range = Uniform::new(-100.0, 100.0);

                let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();

                let document = json!({"id": idx.to_string(), "vector": vector});
                hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();
            }

            let range = Uniform::new(-100.0, 100.0);
            let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();

            let document = json!({"id": 200.to_string(), "vector": vector});
            hiddb::document::insert(db, &index_store, &test_collection.collection_id, &vec![document.clone()]).unwrap();

            // Check if best vector is found in data set
            let best_document_id = hiddb::document::search_ann(
                &db,
                &index_store,
                &collection_name,
                &json!({"field_name": "vector", "vectors": vec![vector.clone()]}),
            )
            .unwrap()[0][0]
                .clone();
            let best_vector: Vec<f64> = hiddb::document::get_by_id(&db, &collection_name, &best_document_id)
                .unwrap()
                .data
                .get("vector")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .collect();
            assert!(1e-8 > distance::euclidean(&best_vector, &vector));
        }
        RocksDB::destroy(&db_options, "./build/large_scale_2.rdb");
    }
}
