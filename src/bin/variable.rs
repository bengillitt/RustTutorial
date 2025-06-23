#![allow(unused)]

fn main() {
    let x: i32 = -123;
    // x += 1; This will not compile since variables are by default immutable

    let mut y: i32 = 123;
    y += 1;

    let z = -123; // Don't need to explicitaly set the type, rust is usually smart enough to work this out

    // let w: () = 123; Debugging what type a variable should be

    const NUM: u32 = 1;

    let x: i32 = -1;
    let x: bool = true; // This is called shadowing

    let v: Vec<_> = vec![1, 2, 3]; // _ is a type placeholder, normally should be something like i32 or u32
}
