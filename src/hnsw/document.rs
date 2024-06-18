use crate::hnsw::Document;
use serde_json::Value;

use seahash::hash;

use bincode::{deserialize, serialize};

impl Document {
    pub fn new(id_user: String, data: Value) -> Self {
        let id_hash = hash(id_user.as_bytes()).to_be_bytes();
        Self { id_user, id_hash, data }
    }

    pub fn id_hash(&self) -> [u8; 8] {
        self.id_hash
    }

    pub fn id_user(&self) -> &String {
        &self.id_user
    }

    pub fn get_field_vector(&self, field_id: &str) -> Vec<f64> {
        self.data[field_id].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id_hash == other.id_hash
    }
}

impl Document {
    pub fn to_binary(self) -> Vec<u8> {
        serialize(&serde_json::to_string(&document_to_value(self)).unwrap()).unwrap()
    }

    pub fn from_binary(document: Vec<u8>) -> Document {
        value_to_document(serde_json::from_str(deserialize(&document).unwrap()).unwrap())
    }
}

pub fn value_to_document(value: Value) -> Document {
    // TODO: handle unwraps
    let id_user: String = value.get("id").unwrap().as_str().unwrap().to_owned();
    Document::new(id_user, value)
}

pub fn document_to_value(document: Document) -> Value {
    // TODO: handle unwraps
    document.data
}
