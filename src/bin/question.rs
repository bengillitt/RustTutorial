#![allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
    // Err("f1 error".to_string())
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

fn f_match() -> Result<u32, String> {
    let res_1 = f1();
    let x1 = match res_1 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    let res_2 = f2();
    let x2 = match res_2 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(x1 + x2);
}

fn f_question() -> Result<u32, String> {
    let res_1 = f1()?;

    let res_2 = f2()?;

    Ok(res_1 + res_2)
}

fn main() -> Result<(), String> {
    // let res = f1();
    // match res {
    //     Ok(x) => println!("{x}"),
    //     Err(err) => println!("err = {err}"),
    // }

    let z = f_question();
    match z {
        Ok(x) => println!("z = {x}"),
        Err(err) => println!("err = {err}"),
    }

    let x = f1()?;

    println!("x = {x}");

    Ok(())
}
