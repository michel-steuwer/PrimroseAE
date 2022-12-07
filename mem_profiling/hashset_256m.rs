use std::collections::{HashSet};
use primrose::traits::Container;
use primrose::tools::gen_dataset_256;


fn hashset_insertion_256m() {
    let s: &mut dyn Container<u32> = &mut HashSet::new();
    let data = gen_dataset_256();
    for val in data.iter() {
        s.insert(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    hashset_insertion_256m();
}