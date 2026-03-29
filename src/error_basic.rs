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

fn error() {
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

fn unwrap_and_expect_usage() {
    // Unwrap and expect for Option -------------------------------

    // Unwrap
    /*
        // Unwrap is equivalent to this
        let value = match x {
            Some(value) => value,
            None => panic!()
        }
    */

    let x = Some(32);
    // This panics
    // let x: Option<i32> = None;
    let value = x.unwrap();
    println!("value is {value}");

    // Expect
    // Same as unwrap, only now the panic comes with a message
    /*
        // Expect is equivalent to this
        let value = match x {
            Some(value) => value,
            None => panic!("Expect ka message")
        }
    */

    // This panics
    // let x: Option<i32> = None;
    let value = x.expect("Expect ka message");
    println!("value is {value}");

    // Unwrap and Expect for Result --------------------------------

    // Unwrap

    // unwrap for option doesn't print any error because for option the other thing is just None
    // But for result, it is Err(some message here)
    // So with unwrap, that message from Err gets printed
    let x: Result<i32, String> = Err("fut gyi".to_string());
    let v = x.clone().unwrap();
    println!("value is {v}");
    // The above statement is equivalent
    // let v = match x {
    //     Ok(val) => val,
    //     Err(err) => panic!("error:- {:?}", err)
    // };
    // println!("value is {v}");

    // Expect
    // With this, we display both, Err's own message + expect's error string.

    let v = x.expect("expect ka alag error message");
    println!("Result's expect usage:- {v}");
}
