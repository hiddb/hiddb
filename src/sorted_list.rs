use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortedList<K: PartialOrd, V> {
    data: Vec<(K, V)>,
}

impl<K: PartialOrd, V> SortedList<K, V>
where
    K: Copy,
    // V: Copy,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_sorted_vec(vec: Vec<(K, V)>) -> Self {
        Self { data: vec }
    }

    pub fn to_vec(self) -> Vec<(K, V)> {
        self.data
    }

    pub fn get_data(&self) -> &Vec<(K, V)> {
        &self.data
    }

    pub fn get_keys(&self) -> Vec<K> {
        self.data.iter().map(|x| x.0).collect()
    }

    pub fn insert(&mut self, key_value_pair: (K, V)) {
        let idx = match self.data.binary_search_by(|entry| {
            entry.0.partial_cmp(&key_value_pair.0).unwrap() // unwrap for NaN
        }) {
            Ok(idx) => idx,  // insertion next to element with same distance
            Err(idx) => idx, // insertion next to element with different distance
        };
        self.data.insert(idx, key_value_pair);
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.data.pop()
    }

    pub fn pop_idx(&mut self, idx: usize) -> (K, V) {
        self.data.remove(idx)
    }

    pub fn first(&self) -> &(K, V) {
        &self.data[0]
    }

    pub fn n_first(&self, n: usize) -> &[(K, V)] {
        let len = self.data.len();
        if n > len {
            return &self.data[..len];
        }
        &self.data[..n]
    }

    pub fn last(&self) -> &(K, V) {
        &self.data.last().unwrap()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<K: PartialOrd, V, Idx> std::ops::Index<Idx> for SortedList<K, V>
where
    Idx: std::slice::SliceIndex<[(K, V)]>,
{
    type Output = Idx::Output;

    fn index(&self, idx: Idx) -> &Self::Output {
        &self.data[idx]
    }
}

impl<K: PartialOrd, V> FromIterator<(K, V)> for SortedList<K, V>
where
    K: Copy,
    V: Copy,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut c = SortedList::new();

        for i in iter {
            c.insert(i);
        }
        c
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_all() {
        let mut sorted_list = SortedList::<f64, u64>::new();
        sorted_list.insert((0.5, 0));
        assert_eq!(sorted_list[0].1, 0);

        sorted_list.insert((0.1, 1));
        assert_eq!(sorted_list[0].1, 1);
        assert_eq!(sorted_list[1].1, 0);

        sorted_list.insert((1.2, 2));
        assert_eq!(sorted_list[0].1, 1);
        assert_eq!(sorted_list[1].1, 0);
        assert_eq!(sorted_list[2].1, 2);
        assert_eq!(sorted_list[1..][0].1, 0);
        assert_eq!(sorted_list[1..][1].1, 2);
        assert_eq!(sorted_list.n_first(2)[0].1, 1);
        assert_eq!(sorted_list.n_first(2)[1].1, 0);

        assert_eq!(sorted_list.first().1, 1);
        assert_eq!(sorted_list.last().1, 2);
        assert_eq!(sorted_list.len(), 3);

        assert_eq!(sorted_list.pop().unwrap().1, 2);
        assert_eq!(sorted_list.len(), 2);
        assert_eq!(sorted_list.pop().unwrap().1, 0);
        assert_eq!(sorted_list.len(), 1);
        assert_eq!(sorted_list.pop().unwrap().1, 1);
        assert_eq!(sorted_list.len(), 0);
        assert_eq!(sorted_list.pop(), None);
    }
}
