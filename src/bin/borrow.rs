#![allow(unused)]

fn main() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;

    // Borrow - temporarily use a value without taking ownership
    // - Creates a reference (either mutable or immutable)
    // - Doesn't move ownership

    // Immutable borrow
    let s = String::from("rust");
    let s1 = &s;
    // any number of read-only access to a value
    let s2 = &s;
    let s3 = s2;

    // Mutable borrow
    // only one mutable reference at a time
    let mut s = String::from("rust");
    let s0 = &mut s;
    s0.push_str(" crustacean");
    let s1 = &mut s;
    // let s2 = &mut s; // Won't compile due to two mutable references
    s1.push_str("🦀");
    println!("{s}");

    // Cannot borrow immutable and mutable simultaneously
    let mut s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    println!("{s1} {s2}");
    // println!("{s3}");

    // Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    // {
    //     let s1 = s;
    // } // s1 and therefore s is dropped

    // std::mem::drop(s);

    println!("{s1}"); // This reference no longer exists
}

// fn f(s: String) -> &String {
//     &s // Ownership moves into this scope, s is dropped so doesn't compile
// }
