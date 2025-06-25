#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Initialize
    let mut scores: HashMap<String, u32> = HashMap::new();
    // Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);

    // Get
    let val: Option<&u32> = scores.get("green");
    println!("val = {:?}", val);

    let val: Option<&u32> = scores.get("yellow");
    println!("val = {:?}", val);
    // Override
    scores.insert("green".to_string(), 200);
    println!("val = {:?}", scores.get("green"));

    // Upsert - Insert if val doesn't exist or update it if value for key already exists
    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;
    println!("Blue Value: {v}");

    let val: Option<&u32> = scores.get("blue");
    println!("Blue Value: {:?}", val);
}
