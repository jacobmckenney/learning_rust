use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::Hasher;
use std::{hash::Hash, marker::PhantomData};

pub struct HashMap<K: Hash, V: PartialOrd + Copy> {
    size: usize,
    vec: Vec<Option<Box<LinkedList<V>>>>,
    key_type: PhantomData<K>,
}

impl<K: Hash, V: PartialOrd + Copy> HashMap<K, V> {
    pub fn new(initial_size: usize) -> HashMap<K, V> {
        HashMap {
            size: 0,
            vec: vec![None; initial_size],
            key_type: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn put(&mut self, key: K, value: V) -> bool {
        // TODO: check if map already contains key
        let hashed_key: usize = self.get_index(key) as usize;
        match self.vec.get(hashed_key) {
            Some(list) => {
                let test = list.as_mut().unwrap();
                if test.contains(&value) {
                    return false;
                }
                test.push_front(value);
                return true;
            }
            None => {
                let mut new_list: LinkedList<V> = LinkedList::new();
                new_list.push_front(value);
                self.vec[hashed_key] = Some(Box::new(new_list));
                return true;
            }
        }
    }

    fn get_index(&self, key: K) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return hasher.finish() % self.vec.len() as u64;
    }
}

#[cfg(test)]
mod hashmap_tests {
    use crate::hashmap::HashMap;
    #[test]
    fn new() {
        let hm: HashMap<i32, i32> = HashMap::new(10);
        assert_eq!(hm.size(), 0);
    }
}
