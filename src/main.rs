fn main() {
    println!("Hello, world")
}


extern crate linked_hash_map;

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::fmt;
use std::hash::{Hash, BuildHasher};
use linked_hash_map::LinkedHashMap;

#[cfg(feature = "heapsize_impl")]
mod heapsize;

#[derive(Clone)]
pub struct LruCache<K: Eq + Hash, V, S: BuildHasher = RandomState> {
    map: LinkedHashMap<K, V, S>,
    max_size: usize,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            map: LinkedHashMap::new(),
            max_size: capacity,
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> LruCache<K, V, S> {
    pub fn with_hasher(capacity: usize, hash_builder: S) -> Self {
        LruCache {
            map: LinkedHashMap::with_hasher(hash_builder),
            max_size: capacity,
        }
    }

    pub fn contains_key<Q: ? Sized>(&mut self, key: &Q) -> bool
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
//        self.get_mut
    }
}

