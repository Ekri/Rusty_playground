fn main(){
    println!("Hello, world")
}


extern crate linked_hash_map;

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::fmt;
use std::hash::{Hash, BuildHasher};
use linked_hash_map::LinkedHashMap;


mod heapsize;