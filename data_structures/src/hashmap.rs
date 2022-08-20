use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

impl<K: Hash + Eq, V: Copy> Bucket<K, V> {
    pub fn new() -> Bucket<K, V> {
        Bucket { items: Vec::new() }
    }
}
pub struct HashMap<K: Hash + Eq, V: Copy> {
    size: usize,
    buckets: Vec<Bucket<K, V>>,
}

impl<K: Hash + Eq, V: Copy> HashMap<K, V> {
    pub fn new(initial_size: usize) -> HashMap<K, V> {
        let mut buckets: Vec<Bucket<K, V>> = Vec::with_capacity(initial_size);
        (0..initial_size).for_each(|_i| buckets.push(Bucket::new()));
        return HashMap { size: 0, buckets };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn search(&mut self, key: K, value: Option<V>) -> Option<V> {
        let bucket: &mut Bucket<K, V> = self.get_bucket(&key);
        for &mut (ref k, ref mut v) in bucket.items.iter_mut() {
            if k == &key {
                if value.is_some() {
                    return Some(std::mem::replace(v, value.unwrap()));
                }
                return Some(*v);
            }
        }
        if value.is_some() {
            bucket.items.push((key, value.unwrap()));
        }
        return None;
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        return self.search(key, None);
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // TODO: check if map already contains key and resize
        return self.search(key, Some(value));
    }

    pub fn remove(&mut self, key: K) -> bool {
        let bucket: &mut Bucket<K, V> = self.get_bucket(&key);
        for (i, &mut (ref k, _)) in bucket.items.iter_mut().enumerate() {
            if k == &key {
                bucket.items.remove(i);
                return true;
            }
        }
        return false;
    }

    pub fn contains(&mut self, key: K) -> bool {
        return match self.search(key, None) {
            Some(_) => true,
            None => false,
        };
    }

    fn resize(&mut self) {}

    fn get_bucket(&mut self, key: &K) -> &mut Bucket<K, V> {
        let hashed_key: usize = self.get_index(&key);
        let bucket_ref: &mut Bucket<K, V> = &mut self.buckets[hashed_key];
        return bucket_ref;
    }

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

    #[test]
    fn put_contains() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        let key = 1;
        let value = 100;
        hm.put(key, value);
        assert!(hm.contains(key));
    }

    #[test]
    fn put_remove() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        let key = 1;
        let value = 100;
        hm.put(key, value);
        assert!(hm.remove(key));
        assert!(!hm.contains(key))
    }

    #[test]
    fn get_empty() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        assert_eq!(hm.get(1), None);
    }

    #[test]
    fn get() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        let key = 1;
        let value = 100;
        assert_eq!(hm.put(key, value), None);
        assert_eq!(hm.get(key).unwrap(), value);
    }
}
