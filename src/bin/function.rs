#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    return x + y;
    // or just x+y (no semicolon and at the end, implicit return)
}

fn print() {
    println!("no output");
}

// Diverging functions - they never return
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("crash");
}

fn main() {
    // implicit return
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    // No output
    print();

    // diverge
    forever();
    crash();
}
