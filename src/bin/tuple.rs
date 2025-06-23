#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Tuple
    let t: (bool, u32, char) = (true, 1, 'c');

    // Destructure
    let (a, b, c) = t;

    // ignore with _
    let (_, b, _) = t;

    // Empty tuple - unit type
    let t = (); // This is a unit type, this can be a way of returning nothing from a function i think

    // Nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());

    let t: (bool, u32, char) = (true, 1, 'c');
    println!("t = {}, {}, {}", { t.0 }, { t.1 }, { t.2 });

    println!("float = {}", nested.0 .0);
    println!("u32 = {}", nested.1 .1);
}
