#![allow(unused)]

fn main() {
    // unwrap and expect
    // let x: Option<i32> = Some(3);
    // let x: Option<i32> = None;

    // let v = x.unwrap();
    // // This is what unwrap essentially does
    // match x {
    //     Some(val) => println!("val = {val}"),
    //     None => panic!("none"),
    // }

    // println!("val = {v}");

    let x: Option<i32> = Some(3);

    let v = x.expect("x is none");

    println!("val = {v}");

    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Err("div by 0".to_string());

    let v = z.unwrap();
    println!("val = {v}");
}
