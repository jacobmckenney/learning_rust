use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const LOAD_MAX: f64 = 0.75;

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
        return HashMap {
            size: 0,
            buckets: HashMap::initialize_buckets(initial_size),
        };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn search(&mut self, key: K, value: Option<V>) -> Option<V> {
        let bucket: &mut Bucket<K, V> = self.get_bucket(&key, self.buckets.len());
        let index = bucket.items.iter().position(|(k, _)| *k == key);
        if value.is_some() {
            bucket.items.push((key, value.unwrap()));
        }
        if index.is_some() {
            let (_, old_value) = bucket.items.remove(index.unwrap());
            return Some(old_value);
        }
        return None;
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        return self.search(key, None);
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let load_factor = self.size() as f64 / self.buckets.capacity() as f64;
        if load_factor > LOAD_MAX {
            self.resize();
        }
        let val: Option<V> = self.search(key, Some(value));
        if val.is_none() {
            self.size += 1;
        }
        return val;
    }

    fn resize(&mut self) {
        let new_size = self.buckets.capacity() * 2;
        let mut new_buckets: Vec<Bucket<K, V>> =
            HashMap::initialize_buckets(self.buckets.capacity() * 2);
        for bucket in self.buckets.drain(0..self.buckets.len()) {
            for item in bucket.items.into_iter() {
                let new_hash = HashMap::<K, V>::get_index(&item.0, new_size);
                new_buckets[new_hash].items.push(item);
            }
        }
        self.buckets = new_buckets;
    }

    pub fn remove(&mut self, key: K) -> bool {
        let bucket: &mut Bucket<K, V> = self.get_bucket(&key, self.buckets.len());
        for (i, &mut (ref k, _)) in bucket.items.iter_mut().enumerate() {
            if k == &key {
                bucket.items.remove(i);
                self.size -= 1;
                return true;
            }
        }
        return false;
    }

    pub fn contains(&mut self, key: K) -> bool {
        return self.search(key, None).is_some();
    }

    fn initialize_buckets(new_size: usize) -> Vec<Bucket<K, V>> {
        let mut buckets: Vec<Bucket<K, V>> = Vec::with_capacity(new_size);
        (0..new_size).for_each(|_i| buckets.push(Bucket::new()));
        return buckets;
    }

    fn get_bucket(&mut self, key: &K, num_buckets: usize) -> &mut Bucket<K, V> {
        let hashed_key: usize = HashMap::<K, V>::get_index(&key, num_buckets);
        let bucket_ref: &mut Bucket<K, V> = &mut self.buckets[hashed_key];
        return bucket_ref;
    }

    fn get_index(key: &K, num_buckets: usize) -> usize {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() % num_buckets as u64) as usize;
    }
}

#[cfg(test)]
mod hashmap_tests {
    use crate::hashmap::HashMap;
    #[test]
    fn new() {
        let hm: HashMap<i32, i32> = HashMap::new(10);
        assert_eq!(hm.size(), 0);
        assert!(hm.is_empty());
    }

    #[test]
    fn put_contains() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        let key = 1;
        let value = 100;
        hm.put(key, value);
        assert!(hm.contains(key));
        assert_eq!(hm.size(), 1);
    }

    #[test]
    fn put_remove() {
        let mut hm: HashMap<i32, i32> = HashMap::new(10);
        let key = 1;
        let value = 100;
        hm.put(key, value);
        assert!(hm.remove(key));
        assert!(!hm.contains(key));
        assert!(hm.is_empty());
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

    #[test]
    fn resize() {
        let mut hm: HashMap<i32, i32> = HashMap::new(2);
        for x in 0..1000 {
            hm.put(x, x);
        }
        for x in 0..1000 {
            assert!(hm.contains(x));
        }
    }
}
