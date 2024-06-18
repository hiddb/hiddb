use rand::prelude::*;
use std::cmp::min;
use std::collections::HashSet;

use crate::distance;
use crate::sorted_list::SortedList;
use crate::reverse_sorted_list::ReverseSortedList;

use crate::db::dbtypes::BinaryConverison;
use crate::db::RocksDB;
use crate::hnsw::key::*;
use crate::hnsw::{Document, Index};
use std::convert::TryInto;

impl Index {
    pub fn insert_in_layer(&mut self, db: &RocksDB, document: &Document, layer_id: u8) {
        // TODO: do this in collection wide: When multiple fields to index are present this is done multiple times
        let document_vector = document.get_field_vector(&self.field_id);

        db.insert_value(&self.collection_hash, &self.field_hash, &document.id_hash, &document_vector)
            .unwrap();
        // db.insert_document(&self.collection_hash, &document.id_hash, &document).unwrap();

        // if element is already present and the value is different
        // delete the element before insertion
        let k = self.k;
        // if let Some(d) = self.get_document(db, &document.id_hash) {
        //     if float_vector_comp(&document.vector, &d.vector) {
        //         // println!("Element already present... do nothing.");
        //         return;
        //     } else {
        //         println!("Replace document {:?}", d);
        //         self.remove(&document.id_hash).unwrap(); // TODO: implement remove element
        //                                                  // println!("Remove element {}", &document.id_hash);
        //         self.n_elements -= 1;
        //     }
        // }

        match self.entry_point {
            Some(_) => {
                // entry point is set
                let mut random_level_idx = layer_id;
                let entry_point = self.entry_point.unwrap(); // Update entry point

                let mut entry_point_document_id = entry_point;

                for level_idx in ((random_level_idx + 1)..self.n_layers).rev() {
                    let new_entry_point_id = self
                        .search_level(
                            &db,
                            &db.get_value(&self.collection_hash, &self.field_hash, &entry_point_document_id).unwrap(),
                            level_idx,
                            &entry_point_document_id,
                        )
                        .first()
                        .1;
                    entry_point_document_id = new_entry_point_id;
                }

                for level_idx in (0..=min(random_level_idx, self.n_layers - 1)).rev() {
                    let nearest_neighbors = self.search_level(&db, &document_vector, level_idx, &entry_point_document_id);

                    // Select neighbors
                    let nn_neighbors = nearest_neighbors.n_first(k);

                    // Add bidirectional connections from neighbors to q
                    for &(_, nn_id) in nn_neighbors.iter() {
                        let nn_vector = db.get_value(&self.collection_hash, &self.field_hash, &nn_id).unwrap();
                        let distance_to_nn = distance::euclidean(&nn_vector, &document_vector);

                        // let nn_from_map = self.neighbor_map[level_idx].get_mut(&nn_id).unwrap();
                        // let mut nn_from_db = self.get_neighbors_from_level(db, &nn_id, &level_idx).unwrap().clone();
                        let mut nn_from_db = db
                            .get_neighbors(&self.collection_hash, &self.field_hash, level_idx, &nn_id)
                            .unwrap()
                            .clone();

                        // TODO: if distance_to_nn is further away than furthest nn do nothing
                        nn_from_db.insert((distance_to_nn, document.id_hash));
                        // Shrink connections if needed
                        if nn_from_db.len() > k {
                            // nn.nearest_neighbors =
                            //     SortedList::from_sorted_vec(nn.nearest_neighbors[..k].to_vec());
                            nn_from_db.pop().unwrap();
                        }

                        db.insert_neighbors(&self.collection_hash, &self.field_hash, level_idx as u8, &nn_id, &nn_from_db)
                            .unwrap();
                    }
                    db.insert_neighbors(
                        &self.collection_hash,
                        &self.field_hash,
                        level_idx as u8,
                        &document.id_hash,
                        &SortedList::from_sorted_vec(nn_neighbors.to_vec()),
                    )
                    .unwrap();

                    entry_point_document_id = nearest_neighbors.first().1;
                }

                // document.level = random_level_idx;
                if random_level_idx >= self.n_layers {
                    // Create new layer with document as a single entry
                    self.entry_point = Some(document.id_hash);
                    random_level_idx = self.n_layers;
                    self.n_layers += 1;

                    db.insert_neighbors(
                        &self.collection_hash,
                        &self.field_hash,
                        random_level_idx,
                        &document.id_hash,
                        &SortedList::new(),
                    )
                    .unwrap();
                }
            }
            None => {
                // No entry point set: Index must be empty
                assert_eq!(self.n_layers, 1);
                assert_eq!(self.n_elements, 0);

                self.entry_point = Some(document.id_hash);

                db.insert_neighbors(
                    &self.collection_hash,
                    &self.field_hash,
                    (self.n_layers - 1) as u8,
                    &document.id_hash,
                    &SortedList::new(),
                )
                .unwrap();
            }
        }
        self.n_elements += 1;
        db.insert_index(&self.collection_hash, &self.field_hash, &self).unwrap();
    }

    pub fn insert(&mut self, db: &RocksDB, document: &Document) {
        let random_level_idx = self.random_level(self.reverse_size);
        self.insert_in_layer(db, document, random_level_idx);
    }

    pub fn knn_search(&self, db: &RocksDB, vector: &Vec<f64>, max_neighbors: usize) -> Vec<[u8; 8]> {
        if self.n_elements == 0 {
            return vec![];
        }

        let entry_point = self.entry_point.unwrap();

        let mut entry_point_document_new = db.get_document(&self.collection_hash, &entry_point).unwrap();

        for level_idx in (1..self.n_layers).rev() {
            let new_entry_point_id = self.search_level(&db, &vector, level_idx, &entry_point_document_new.id_hash).first().1;
            entry_point_document_new = db.get_document(&self.collection_hash, &new_entry_point_id).unwrap();
        }

        let nearest_neighbors = self.search_level(db, &vector, 0, &entry_point_document_new.id_hash);

        // Select neighbors
        let nearest_neighbors = nearest_neighbors.n_first(max_neighbors);
        nearest_neighbors.iter().map(|x| x.1).collect()
    }

    pub fn search_level(&self, db: &RocksDB, vector: &Vec<f64>, level_idx: u8, entry_point: &[u8; 8]) -> SortedList<f64, [u8; 8]> {
        // let entry_point: Document = db.get_document(&self.collection_hash, &entry_point).unwrap();
        let entry_point_vector: Vec<f64> = db.get_value(&self.collection_hash, &self.field_hash, &entry_point).unwrap();

        let distance_to_entry_point: f64 = distance::euclidean(&entry_point_vector, vector);

        let mut candidates: ReverseSortedList<f64, [u8; 8]> = ReverseSortedList::new();
        candidates.insert((distance_to_entry_point, entry_point.clone()));

        let mut nearest_neighbors: SortedList<f64, [u8; 8]> = SortedList::new();
        nearest_neighbors.insert((distance_to_entry_point, entry_point.clone()));

        let mut visited: HashSet<[u8; 8]> = HashSet::new();
        visited.insert(entry_point.clone());

        while candidates.len() > 0 {
            // TODO: instead of pop_idx reverse order of list to increase efficiency
            let nearest_candidate = candidates.pop().unwrap();
            let mut furthest_nearest_neighbor = nearest_neighbors.last();

            if distance::squared_euclidean(
                &db.get_value(&self.collection_hash, &self.field_hash, &nearest_candidate.1).unwrap(),
                vector,
            ) > distance::squared_euclidean(
                &db.get_value(&self.collection_hash, &self.field_hash, &furthest_nearest_neighbor.1)
                    .unwrap(),
                vector,
            ) {
                break;
            }
            let neighbor_ids: Vec<[u8; 8]> = db
                .get_neighbors(&self.collection_hash, &self.field_hash, level_idx, &nearest_candidate.1)
                .unwrap()
                .get_data()
                .iter()
                .map(|d| d.1)
                .collect();

            for &neighbor_id in neighbor_ids.iter() {
                if visited.contains(&neighbor_id) {
                    continue;
                }
                visited.insert(neighbor_id);
                furthest_nearest_neighbor = nearest_neighbors.last();

                let neighbor_vector = db.get_value(&self.collection_hash, &self.field_hash, &neighbor_id).unwrap();
                let neighbor_distance = distance::euclidean(vector, &neighbor_vector);

                if neighbor_distance < furthest_nearest_neighbor.0 || nearest_neighbors.len() < self.k {
                    candidates.insert((neighbor_distance, neighbor_id));
                    nearest_neighbors.insert((neighbor_distance, neighbor_id));

                    if nearest_neighbors.len() > self.k {
                        nearest_neighbors.pop();
                    }
                }
            }
        }

        return nearest_neighbors
            .to_vec()
            .iter()
            .map(|distance_neighbor_pair| (distance_neighbor_pair.0, distance_neighbor_pair.1)) // TODO: more efficient implementation necessay: Don't call insert all the time!
            .collect();
    }

    pub fn remove(&mut self, id: &u64) -> Option<u64> {
        Some(*id)
    }

    fn random_level(&mut self, reverse_size: f64) -> u8 {
        (-(self.rng.gen::<f64>()).ln() * reverse_size) as u8
    }
}

impl Index {
    pub fn get_entry_point(&self) -> Option<[u8; 8]> {
        self.entry_point
    }

    pub fn n_layer(&self) -> u8 {
        self.n_layers
    }

    pub fn n_elements(&self) -> u64 {
        self.n_elements
    }

    // pub fn remove(&mut self, id: &u64) -> Option<Document> {
    //     for level in self.data.iter_mut() {
    //         match level.remove(id) {
    //             Some(d) => return Some(d),
    //             None => {}
    //         }
    //     }
    //     None
    // }

    // pub fn remove_from_level(&mut self, id: &u64, level_id: &usize) -> Option<Document> {
    //     self.data[*level_id].remove(id)
    //     // TODO: Missing updates
    // }
}

pub fn get_index_hash(collection_hash: [u8; 8], field_hash: [u8; 8]) -> [u8; 16] {
    [collection_hash, field_hash].concat().try_into().unwrap()
}
