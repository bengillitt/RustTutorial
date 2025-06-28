#![allow(unused)]

// Monomorphization
struct Point<T> {
    x: T,
    y: T,
}

fn get_x<T>(p: Point<T>) -> T {
    p.x
}

fn main() {
    // Monomorphization increases compile time and size of binary, so generic methods aren't preferable
    // upside is that there is no runtime overhead
    let p0: Point<u32> = Point { x: 0, y: 0 };
    let p1: Point<i32> = Point { x: 0, y: 0 };

    let x = get_x(p0);
    let x = get_x(p1);
}
