use std::collections::hash_map::DefaultHasher;
use std::hash::{self, Hasher};
use std::{hash::Hash, marker::PhantomData};

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

impl<K: Hash + Eq, V> Bucket<K, V> {
    pub fn new() -> Bucket<K, V> {
        Bucket { items: Vec::new() }
    }
}
pub struct HashMap<K: Hash + Eq, V> {
    size: usize,
    buckets: Vec<Bucket<K, V>>,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new(initial_size: usize) -> HashMap<K, V> {
        let mut buckets: Vec<Bucket<K, V>> = Vec::with_capacity(initial_size);
        for _i in 0..initial_size {
            buckets.push(Bucket::new())
        }
        return HashMap { size: 0, buckets };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // TODO: check if map already contains key and resize
        let hashed_key: usize = self.get_index(&key);
        let bucket = &mut self.buckets[hashed_key];
        for &mut (ref k, ref mut v) in bucket.items.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }
        bucket.items.push((key, value));
        return None;
    }

    fn resize(&mut self) {}

    fn get_index(&self, key: &K) -> usize {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() % self.buckets.len() as u64) as usize;
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
