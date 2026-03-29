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

fn f1() -> Result<u32, String> {
    println!("f1");
    // Ok(1)
    return Err("gande phatti".to_string());
}

// For anything to be returned by main function, it must implemetn Termination trait.
// u32 doesn't implement it, => Result<u32, _> can't be returned from it.
// This trait decides the exit status code using the output of the main function.

fn question_operator_usage() -> Result<(), String> {
    // Question operator
    // A bit different from unwrap and expect
    // unwrap/expect - if ok/some, get the internal value
    // If not, panic
    // ? - ok/some behaviour same, but for Err/None, it returns Err, instead of panicking
    // Which is why this can be thought of as something to bubble up the error.

    // let res = f1();
    // let ans: Result<u32, String> = match res {
    //     Ok(val) => Ok(val),
    //     Err(err) => return Err(err),
    // };

    // Ok(())

    // ? operator does what's done above
    // Its not even doing something error specific like panic or anything.
    // Just printing the error

    let res = f1()?;
    println!("res={res}");
    Ok(())
}

fn f2() -> Result<u32, String> {
    println!("f2");
    return Ok(2);
}

fn f3() -> Result<u32, bool> {
    println!("f3");
    Ok(3)
}

fn f_match() -> Result<u32, String> {
    let res_1 = f1();
    let res_2 = f2();

    let x_1 = match res_1 {
        Ok(v) => v,
        Err(err) => return Err(err),
    };

    let x_2 = match res_2 {
        Ok(v) => v,
        Err(err) => return Err(err),
    };

    return Ok(x_1 + x_2);
    // Could also be written like down below
    // But because we want to exemplify using ?, we do it as above
    // match res_1 {
    //     Ok(res_1) => match res_2 {
    //         Ok(res_2) => return Ok(res_1 + res_2),
    //         Err(err) => return Err(err),
    //     },
    //     Err(err) => return Err(err),
    // }
}

// Re-writing the above function with question operator
// But this works only when ? returns same error type, or error types are interconvertible
// This is exemplified in the function after this function
fn f_match_question_operator() -> Result<u32, String> {
    let x_1 = f1()?;
    let x_2 = f2()?;
    return Ok(x_1 + x_2);
}

fn f_match_question_operator_new() -> Result<u32, String> {
    let x_1 = f1()?;
    let x_2 = f2()?;
    // This gives error
    // Because boolean type can't be converted to a String automatically
    // let x_3 = f3()?;
    // One way to fix this is to return String and not bool
    // Another way is to pattern match
    let x_3 = match f3() {
        Ok(v) => v,
        Err(err) => return Err("string error for f3".to_string()),
    };
    return Ok(x_1 + x_2 + x_3);
}

fn main_2() {
    let z = match f_match_question_operator() {
        Ok(val) => println!("value is {val}"),
        Err(err) => println!("Error hai: {err}"),
    };
}
