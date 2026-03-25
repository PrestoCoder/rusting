#![allow(unused)]

// would panic if y = 0
fn div(x: i32, y: i32) -> i32 {
    x / y
}

// Even better would be to return error as enums(fixed number)
fn better_div(x: i32, y: i32) -> Result<i32, String> {
    match y {
        0 => Err("0 denominator mein kyun rakha bosdi".to_string()),
        _ => Ok(x / y),
    }
}

// Note: Right now, Result and Option don't make a lot of sense.
// Because how are they any different from creating our own enums, and handling them ourselves all the time??
// Their use case becomes more evident, when we start using std library for handling results and options.
// Functions like unwrap, expect, ?, ..., what rust provides out of the box, are the ones that make Result and Option more useful. 

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    Other,
}
fn best_div(x: i32, y: i32) -> Result<i32, MathError> {
    match y {
        0 => Err(MathError::DivisionByZero),
        _ => Ok(x / y),
    }
}

fn main() {
    // Error
    // Simplest thing is to let the program crash - using panic!
    // If panic in main, entire program terminates
    // If in a thread, only that thread terminates
    // panic!("chud gye guru");

    // Option or Result
    let arr = [1, 2, 3];
    // Rust compiler won't even let this compile, let alone panic
    // arr[9];

    // Option
    // Which is why there is a better function for this.
    // P.S.: get function returns pointer, because it can be used to return a slice as well
    // ex: arr.get(1..4)
    let element = arr.get(0);
    match element {
        Some(value) => println!("value is {value}"),
        None => println!("Oops! tie tie fish"),
    };

    // Result
    div(2, 1);
    match best_div(2, 0) {
        Ok(val) => println!("result = {val}"),
        Err(err) => println!("Error = {err:?}"),
    }
}
