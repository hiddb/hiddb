use std::convert::TryInto;

pub const COLLECTION: u8 = 'c' as u8;
pub const DOCUMENT: u8 = 'd' as u8;
pub const VALUE: u8 = 'v' as u8;
pub const INDEX: u8 = 'i' as u8;
pub const NEIGHBORS: u8 = 'n' as u8;
pub const REVERSE_NEIGHBORS: u8 = 'r' as u8;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Key([u8; 26]);

impl Key {
    pub fn new() -> Self {
        Key([0; 26])
    }

    pub fn from(bytes: [u8; 26]) -> Self {
        Key(bytes)
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Key(bytes.try_into().unwrap())
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Key(bytes.try_into().unwrap())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn get_type(&self) -> u8 {
        self.0[0]
    }

    pub fn set_type(&mut self, prefix: u8) {
        self.0[0] = prefix
    }

    pub fn get_collection_id(&self) -> [u8; 8] {
        self.0[1..9].try_into().unwrap()
    }

    pub fn set_collection_id(&mut self, id: &[u8; 8]) {
        self.0[1..9].copy_from_slice(id);
    }

    pub fn get_field_id(&self) -> [u8; 8] {
        self.0[9..17].try_into().unwrap()
    }

    pub fn set_field_id(&mut self, id: &[u8; 8]) {
        self.0[9..17].copy_from_slice(id);
    }

    pub fn get_layer(&self) -> u8 {
        self.0[17]
    }

    pub fn set_layer(&mut self, prefix: u8) {
        self.0[17] = prefix
    }

    pub fn get_document_id(&self) -> [u8; 8] {
        self.0[18..26].try_into().unwrap()
    }

    pub fn set_document_id(&mut self, id: &[u8; 8]) {
        self.0[18..26].copy_from_slice(id);
    }
}

impl<Idx> std::ops::Index<Idx> for Key
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx]
    }
}

pub struct Prefix(Vec<u8>);

impl Prefix {
    pub fn new() -> Self {
        Self(Vec::with_capacity(26))
    }

    pub fn prefix_type(mut self, prefix_type: u8) -> Self {
        assert!(self.0.len() == 0);
        self.0.push(prefix_type);
        self
    }

    pub fn collection(mut self, collection_id_hash: &[u8; 8]) -> Self {
        assert!(self.0.len() == 1);
        self.0.append(&mut collection_id_hash.to_vec());
        self
    }

    pub fn field(mut self, field_id_hash: &[u8; 8]) -> Self {
        assert!(self.0.len() == 9);
        self.0.append(&mut field_id_hash.to_vec());
        self
    }

    pub fn layer(mut self, layer: u8) -> Self {
        assert!(self.0.len() == 17);
        self.0.push(layer);
        self
    }

    pub fn document(mut self, document_id_hash: &[u8; 8]) -> Self {
        assert!(self.0.len() == 18);
        self.0.append(&mut document_id_hash.to_vec());
        self
    }

    pub fn finish(self) -> Vec<u8> {
        self.0
    }
}
