#![allow(unused)]

// String and str
fn take_string(s: String) {}
fn borrow_string(s: &String) {}
fn make_string() -> String {
    "".to_string()
}

fn mut_string(s: &mut String) {
    s.push_str("?");
}

fn borrow_str(s: &str) {}
fn take_str(s: &str) {}

fn main() {
    // String
    // struct containing a vector<u8>
    // It is owned
    // mutable and growable
    // allocated on the heap
    // &String can be coerced into &str
    let s = String::from("rust");
    take_string(s);
    // println!("{s}");

    // mut String
    let mut s = String::from("rust");
    s += "!";

    // &String
    let s = String::from("rust");
    borrow_string(&s);
    println!("{s}");

    // &String can be coerced into &str
    borrow_str(&s);

    // &mut String
    let mut s = String::from("rust");
    let s1: &mut String = &mut s;
    mut_string(s1);
    println!("&mut String: {s}");

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size of the type not known at compile time
    // - Compiler needs to know the size of each type
    // let a: str = "hello";
    // let b: str = "hello rust";

    // &str
    // - size known at compile time (pointer)
    // - immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("{s}");

    // &mut str
    let mut s = String::from("hello");
    let r: &mut str = &mut s;
}
