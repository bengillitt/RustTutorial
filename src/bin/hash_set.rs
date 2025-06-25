#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // HashSet - only allows unique entries, if entry already in hashset, it won't be inserted
    let v: Vec<u32> = vec![1, 2, 3, 1];
    let map: HashMap<u32, bool> = HashMap::new();

    let mut set: HashSet<u32> = HashSet::new();

    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");

    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");

    let contains: bool = set.contains(&1);
    println!("contains 1?: {contains}");

    let contains: bool = set.contains(&2);
    println!("contains 2?: {contains}");

    let contains: bool = set.contains(&3);
    println!("contains 3?: {contains}");
}
