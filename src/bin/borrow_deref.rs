#![allow(unused)]

fn modify(s: &mut String) {
    // This doesn't take ownership
    *s += "?";
}

fn main() {
    // Deref
    let mut s = String::from("rust");
    let s1 = &mut s;
    *s1 += "?";
    println!("{s}");

    let mut s = String::from("rust");
    modify(&mut s);
    println!("{s}");

    // Deref coercion
    // Automatically dereferenced in some situations
    let x = 1;
    let y = &x;
    let z = &x;
    let w = *y + *z;
    let w = y + z; // This doesn't make sense because you can't add two references. Rust knows this so does it automatically for us
    println!("w = {w}");
}
