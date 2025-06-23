use super::foo;
pub fn call_foo() {
    foo::print();
}

pub fn print() {
    f();
    println!("my");
}

fn f() {
    // This is a private function. Can't be called outside module
    a::print();
}

pub mod a;
