#![allow(unused)]

fn f(s: String) {}

fn take(s: String) {}

fn copy(i: i32) {}

fn main() {
    // let s = String::from("rust");
    // f(s);
    // println!("{s}");

    // Memory - stack and heap
    // Stack
    // Stores data of fixed size at compile time
    // Fast
    // LIFO
    // Heap
    // Stores data of unknown size at compile time
    // Slower than stack
    // Data managed by ownership and borrowing rules
    //
    // Ownership rules - ensure memory safety
    // 1. Each value has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scop, the value will be dropped

    // 1. Each value has an owner
    // Owner of "rust" is s
    let s = String::from("rust");
    // Owner of -1 is i
    let i: i32 = -1;

    // 2. There can only be one owner at a time
    let s = String::from("rust");
    // Owner of "rust" is s1
    let s1 = s;
    println!("{s1}");
    // println!("{s}"); // This won't work because ownership has been transferred
    // Owner of "rust" is s2
    let s2 = s1;
    println!("{s2}");

    // Owner of -1 is i
    let i: i32 = -1;
    // Owner of -1 is i1
    let i1: i32 = i;
    // Ownership isn't transferred, it's copied over
    println!("{i}");

    // 3. When the owner goes out of scope, the value will be dropped
    let s = String::from("rust");
    if (true) {
        s;
    } // s is dropped
      // println!("{s}"); // This doesn't work because the value has been dropped

    let s = String::from("rust");
    take(s);
    // println!("{s}"); // s is dropped so doesn't work

    let i: i32 = -1;
    copy(i);
    println!("{i}");
}
