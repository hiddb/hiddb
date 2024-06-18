extern crate num_cpus;

pub mod dbtypes;
use dbtypes::*;

use crate::hnsw::key::*;
use crate::hnsw::*;
use crate::sorted_list::SortedList;

use std::path::Path;

use std::collections::HashMap;

use rocksdb::{ColumnFamily, DBWithThreadMode, Error, MultiThreaded, Options, SingleThreaded};

pub type DB = DBWithThreadMode<MultiThreaded>;

pub struct RocksDB {
    pub db: DB,
    pub options: Options,
}

fn init_db(db: &mut DB, options: &Options) {
    // println!("{:?}", DB::list_cf(options, "./HIDDBrocksdb"));
}

impl RocksDB {
    pub fn init(path: &str) -> Self {
        let new_db = !Path::new(path).exists();

        let mut options = Options::default();
        options.increase_parallelism(num_cpus::get() as i32);
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        options.enable_statistics();

        let cf_names = ["default", "neighbors"];
        let mut db = DB::open_cf(&options, path, &cf_names).unwrap();

        if new_db {
            init_db(&mut db, &options);
        }
        Self { db, options }
    }
}

impl RocksDB {
    pub fn get_by_key(&self, cf: &str, key: &Key) -> Result<Option<Vec<u8>>, Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        self.db.get_cf(&cf, key.to_vec())
    }

    pub fn get_by_prefix(&self, cf: &str, prefix: &Vec<u8>) -> Result<Vec<Vec<u8>>, Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        let mut results: Vec<Vec<u8>> = Vec::new();
        for (k, v) in self.db.prefix_iterator_cf(&cf, prefix) {
            if k[..prefix.len()] != prefix[..] {
                break;
            }
            results.push((*v).to_vec());
        }
        Ok(results)
    }

    pub fn get_by_prefix_key_value(&self, cf: &str, prefix: &Vec<u8>) -> Result<Vec<(Key, Vec<u8>)>, Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        let mut results: Vec<(Key, Vec<u8>)> = Vec::new();
        for (k, v) in self.db.prefix_iterator_cf(&cf, prefix) {
            if k[..prefix.len()] != prefix[..] {
                break;
            }
            results.push((Key::from_slice(&*k), (*v).to_vec()));
        }
        Ok(results)
    }

    pub fn put(&self, cf: &str, key: &Key, value: &Vec<u8>) -> Result<(), Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        self.db.put_cf(&cf, key.to_vec(), value)
    }

    pub fn delete(&self, cf: &str, key: &Key) -> Result<(), Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        self.db.delete_cf(&cf, key.to_vec())
    }

    pub fn delete_by_prefix(&self, cf: &str, prefix: &Vec<u8>) -> Result<(), Error> {
        let cf = self.db.cf_handle(cf).unwrap();
        for (k, _) in self.db.prefix_iterator_cf(&cf, prefix) {
            if k[..prefix.len()] != prefix[..] {
                break;
            }
            self.db.delete_cf(&cf, k).unwrap();
        }
        Ok(())
    }
}

impl RocksDB {
    pub fn insert_collection(&self, collection_id: &[u8; 8], collection: &Collection) -> Result<(), Error> {
        let mut key = Key::new();
        key.set_type(COLLECTION);
        key.set_collection_id(collection_id);

        // TODO: create collections
        self.put("default", &key, &collection.to_binary())
    }

    pub fn insert_index(&self, collection_id: &[u8; 8], field_id: &[u8; 8], index: &Index) -> Result<(), Error> {
        let mut key = Key::new();
        key.set_type(INDEX);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        // TODO: create index on field
        self.put("default", &key, &IndexDB::from_hnsw_type(index).to_binary())
    }

    pub fn insert_document(&self, collection_id: &[u8; 8], document_id: &[u8; 8], document: &Document) -> Result<(), Error> {
        let mut key = Key::new();
        key.set_type(DOCUMENT);
        key.set_collection_id(collection_id);
        key.set_document_id(&document_id);

        // TODO: create index on field
        self.put("default", &key, &document.clone().to_binary())
    }

    pub fn insert_value(&self, collection_id: &[u8; 8], field_id: &[u8; 8], document_id: &[u8; 8], value: &Vec<f64>) -> Result<(), Error> {
        let mut key = Key::new();
        key.set_type(VALUE);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        key.set_document_id(document_id);

        // TODO: create index on field
        self.put("default", &key, &value.clone().to_binary())
    }

    pub fn insert_neighbors(
        &self,
        collection_id: &[u8; 8],
        field_id: &[u8; 8],
        layer_id: u8,
        document_id: &[u8; 8],
        neighbors: &SortedList<f64, [u8; 8]>,
    ) -> Result<(), Error> {
        let mut key = Key::new();
        key.set_type(NEIGHBORS);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        key.set_layer(layer_id);
        key.set_document_id(&document_id);

        // TODO: create index on field
        self.put("neighbors", &key, &neighbors.to_binary())
    }
}

impl RocksDB {
    pub fn get_collection(&self, collection_id: &[u8; 8]) -> Option<Collection> {
        let mut key = Key::new();
        key.set_type(COLLECTION);
        key.set_collection_id(collection_id);
        return match self.get_by_key("default", &key).unwrap() {
            Some(c) => Some(Collection::from_binary(&c)),
            _ => None,
        };
    }

    pub fn get_indices_in_collection(&self, collection_id: &[u8; 8]) -> Result<Vec<Vec<u8>>, Error> {
        let prefix = Prefix::new().prefix_type(INDEX).collection(collection_id).finish();
        self.get_by_prefix("default", &prefix)
    }

    pub fn get_index(&self, collection_id: &[u8; 8], field_id: &[u8; 8]) -> Option<Index> {
        let mut key = Key::new();
        key.set_type(INDEX);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        let index = match self.get_by_key("default", &key).unwrap() {
            Some(i) => IndexDB::from_binary(&i).to_hnsw_type(),
            _ => return None,
        };
        Some(index)
    }

    pub fn get_document(&self, collection_id: &[u8; 8], document_id: &[u8; 8]) -> Option<Document> {
        let mut key = Key::new();
        key.set_type(DOCUMENT);
        key.set_collection_id(collection_id);
        key.set_field_id(&[0u8; 8]);
        key.set_document_id(document_id);
        match self.get_by_key("default", &key).unwrap() {
            Some(document) => Some(Document::from_binary(document)),
            None => None,
        }
    }

    pub fn get_value(&self, collection_id: &[u8; 8], field_id: &[u8; 8], document_id: &[u8; 8]) -> Option<Vec<f64>> {
        let mut key = Key::new();
        key.set_type(VALUE);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        key.set_document_id(document_id);
        match self.get_by_key("default", &key).unwrap() {
            Some(value) => Some(Vec::<f64>::from_binary(&value)),
            None => None,
        }
    }

    pub fn get_neighbors(
        &self,
        collection_id: &[u8; 8],
        field_id: &[u8; 8],
        layer_id: u8,
        document_id: &[u8; 8],
    ) -> Option<SortedList<f64, [u8; 8]>> {
        let mut key = Key::new();
        key.set_type(NEIGHBORS);
        key.set_collection_id(collection_id);
        key.set_field_id(&field_id);
        key.set_layer(layer_id);
        key.set_document_id(document_id);

        match self.get_by_key("neighbors", &key).unwrap() {
            Some(neighbors) => Some(SortedList::<f64, [u8; 8]>::from_binary(&neighbors)),
            None => None,
        }
    }

    pub fn get_reverse_neighbors(
        &self,
        collection_id: &[u8; 8],
        field_id: &[u8; 8],
        layer_id: u8,
        document_id: &[u8; 8],
    ) -> Result<Option<Vec<u8>>, Error> {
        let mut key = Key::new();
        key.set_type(REVERSE_NEIGHBORS);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        key.set_layer(layer_id);
        key.set_document_id(document_id);

        // TODO: create index on field
        self.get_by_key("neighbors", &key)
    }
}

impl RocksDB {
    pub fn delete_collection(&self, collection_id: &[u8; 8]) -> Result<(), Error> {
        // Delete all associated indicies, documents, neighbors and reverse neighbors
        self.delete_by_prefix("default", &Prefix::new().prefix_type(INDEX).collection(collection_id).finish())
            .unwrap();
        self.delete_by_prefix("default", &Prefix::new().prefix_type(DOCUMENT).collection(collection_id).finish())
            .unwrap();
        self.delete_by_prefix("neighbors", &Prefix::new().prefix_type(NEIGHBORS).collection(collection_id).finish())
            .unwrap();
        self.delete_by_prefix(
            "neighbors",
            &Prefix::new().prefix_type(REVERSE_NEIGHBORS).collection(collection_id).finish(),
        )
        .unwrap();

        // Delete collection itself
        let mut key = Key::new();
        key.set_type(COLLECTION);
        key.set_collection_id(collection_id);
        self.delete("default", &key)
    }

    pub fn delete_index(&self, collection_id: &[u8; 8], field_id: &[u8; 8]) -> Result<(), Error> {
        // Delete all associated indicies, documents, neighbors and reverse neighbors
        self.delete_by_prefix("default", &Prefix::new().prefix_type(DOCUMENT).collection(collection_id).finish())
            .unwrap();
        self.delete_by_prefix("neighbors", &Prefix::new().prefix_type(NEIGHBORS).collection(collection_id).finish())
            .unwrap();
        self.delete_by_prefix(
            "neighbors",
            &Prefix::new().prefix_type(REVERSE_NEIGHBORS).collection(collection_id).finish(),
        )
        .unwrap();

        // Delete index itself
        let mut key = Key::new();
        key.set_type(INDEX);
        key.set_collection_id(collection_id);
        key.set_field_id(field_id);
        self.delete("default", &key)
    }
}

impl RocksDB {
    pub fn get_options(&self) -> Options {
        self.options.clone()
    }

    pub fn destroy<P: AsRef<Path>>(db_options: &Options, path: P) {
        DB::destroy(db_options, path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_in_rocksdb() {
        let db_options;
        {
            let db = RocksDB::init("test_rocksdb.rdb");
            db_options = db.get_options();
            let mut key_document = Key::new();
            key_document.set_type(DOCUMENT);
            db.put("default", &key_document, &vec![0, 0, 0]).unwrap();

            let mut key_index = Key::new();
            key_index.set_type(INDEX);
            db.put("default", &key_index, &vec![0, 0, 1]).unwrap();

            let mut key_collection = Key::new();
            key_collection.set_type(COLLECTION);
            db.put("default", &key_collection, &vec![0, 0, 2]).unwrap();

            assert_eq!(db.get_by_key("default", &key_document).unwrap(), Some(vec![0, 0, 0]));
            assert_eq!(db.get_by_key("default", &key_index).unwrap(), Some(vec![0, 0, 1]));
            assert_eq!(db.get_by_key("default", &key_collection).unwrap(), Some(vec![0, 0, 2]));

            // Test collection deletion
            db.delete("default", &key_collection).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection).unwrap(), None);

            let mut key_collection_1 = Key::new();
            key_collection_1.set_type(COLLECTION);
            let collection_id = &[0, 0, 0, 0, 1, 1, 1, 2];
            key_collection_1.set_collection_id(collection_id);
            db.put("default", &key_collection_1, &vec![1, 2, 3]).unwrap();
            let mut key_collection_2 = Key::new();
            key_collection_2.set_type(COLLECTION);
            let collection_id = &[1, 5, 2, 0, 1, 1, 1, 2];
            key_collection_2.set_collection_id(collection_id);
            db.put("default", &key_collection_2, &vec![2, 2, 3]).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection_1).unwrap(), Some(vec![1, 2, 3]));
            assert_eq!(db.get_by_key("default", &key_collection_2).unwrap(), Some(vec![2, 2, 3]));
            db.delete_by_prefix("default", &Prefix::new().prefix_type(COLLECTION).finish()).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection_1).unwrap(), None);
            assert_eq!(db.get_by_key("default", &key_collection_2).unwrap(), None);

            let mut key_collection_1 = Key::new();
            key_collection_1.set_type(COLLECTION);
            let collection_id_1 = &[0, 0, 0, 0, 1, 1, 1, 2];
            key_collection_1.set_collection_id(collection_id_1);
            db.put("default", &key_collection_1, &vec![1, 2, 3]).unwrap();
            let mut key_collection_2 = Key::new();
            key_collection_2.set_type(COLLECTION);
            let collection_id_2 = &[1, 5, 2, 0, 1, 1, 1, 2];
            key_collection_2.set_collection_id(collection_id_2);
            db.put("default", &key_collection_2, &vec![2, 2, 3]).unwrap();
            let mut key_index_2 = key_collection_2.clone();
            key_index_2.set_type(INDEX);
            db.put("default", &key_index_2, &vec![2, 2, 8]).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection_1).unwrap(), Some(vec![1, 2, 3]));
            assert_eq!(db.get_by_key("default", &key_collection_2).unwrap(), Some(vec![2, 2, 3]));
            assert_eq!(db.get_by_key("default", &key_index_2).unwrap(), Some(vec![2, 2, 8]));
            db.delete_collection(collection_id_1).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection_1).unwrap(), None);
            assert_eq!(db.get_by_key("default", &key_collection_2).unwrap(), Some(vec![2, 2, 3]));
            assert_eq!(db.get_by_key("default", &key_index_2).unwrap(), Some(vec![2, 2, 8]));
            db.delete_collection(collection_id_2).unwrap();
            assert_eq!(db.get_by_key("default", &key_collection_1).unwrap(), None);
            assert_eq!(db.get_by_key("default", &key_collection_2).unwrap(), None);
            assert_eq!(db.get_by_key("default", &key_index_2).unwrap(), None);

            key_document.set_layer(1);
            db.put("default", &key_document, &vec![0, 1, 0]).unwrap();
            key_document.set_layer(2);
            db.put("default", &key_document, &vec![0, 2, 0]).unwrap();

            assert_eq!(
                db.get_by_prefix("default", &vec![DOCUMENT]).unwrap(),
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 2, 0]]
            );
        }
        RocksDB::destroy(&db_options, "test_rocksdb.rdb");
    }
}
