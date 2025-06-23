#![allow(unused)]

// Modules
use hello_rust::my;

fn main() {
    my::print();
    my::a::print();
    let s = my::a::S {
        id: 15,
        name: "hello".to_string(),
    };

    let id = s.id.to_string();
    let name = s.name;

    println!("My Id is {id} and my name is {name}");

    my::call_foo();
    my::a::call_foo();
}
