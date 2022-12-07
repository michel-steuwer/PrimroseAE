use std::collections::{BTreeSet};
use primrose::traits::Container;
use primrose::tools::gen_dataset_512;


fn btreeset_insertion_512m() {
    let s: &mut dyn Container<u32> = &mut BTreeSet::new();
    let data = gen_dataset_512();
    for val in data.iter() {
        s.insert(*val);
    }
    println!("Contains 1024? {}", s.contains(&1024));
}

fn main() {
    btreeset_insertion_512m();
}