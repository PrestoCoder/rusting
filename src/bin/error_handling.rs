#![allow(unused)]

// =========================================================================
// PART 1: panic, Option, Result basics + unwrap/expect/? operator
// =========================================================================

fn div(x: i32, y: i32) -> i32 {
    x / y // panics if y == 0
}

fn better_div(x: i32, y: i32) -> Result<i32, String> {
    match y {
        0 => Err("0 denominator mein kyun rakha bosdi".to_string()),
        _ => Ok(x / y),
    }
}

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

fn basics() {
    // Simplest approach: let the program crash
    // panic!("chud gye guru");

    // Option — safe array access
    let arr = [1, 2, 3];
    // arr[9]; // compile error — Rust won't even allow this
    let element = arr.get(0); // returns Option<&i32>
    match element {
        Some(value) => println!("value is {value}"),
        None        => println!("out of bounds"),
    }

    // Result — structured error
    match best_div(2, 0) {
        Ok(val)  => println!("result = {val}"),
        Err(err) => println!("Error = {err:?}"),
    }
}

fn unwrap_and_expect() {
    // unwrap — get the inner value or PANIC
    // equivalent to: match x { Some(v) => v, None => panic!() }
    let x = Some(32_i32);
    let value = x.unwrap();
    println!("unwrap: {value}");

    // expect — same as unwrap but with a custom panic message
    let value = x.expect("x was None — that's a bug");
    println!("expect: {value}");

    // unwrap on Result — prints the Err message in the panic
    let r: Result<i32, String> = Ok(99);
    let v = r.unwrap();
    println!("Result unwrap: {v}");

    // expect on Result — shows both Err message AND your custom string
    let r: Result<i32, String> = Ok(42);
    let v = r.expect("r should always be Ok here");
    println!("Result expect: {v}");
}

// =========================================================================
// PART 2: ? operator and error propagation
// =========================================================================

fn returns_err() -> Result<u32, String> {
    Err("gande phatti".to_string())
}

fn returns_ok() -> Result<u32, String> {
    Ok(2)
}

fn returns_bool_err() -> Result<u32, bool> {
    Ok(3)
}

// ? does: if Ok(v) → unwrap to v; if Err(e) → return Err(e) early
// It "bubbles up" errors instead of panicking.
fn question_operator_usage() -> Result<(), String> {
    let res = returns_err()?; // Err propagates up; we never reach println!
    println!("res = {res}");
    Ok(())
}

// Manual version of what ? does — spelled out with match
fn f_manual_propagation() -> Result<u32, String> {
    let x1 = match returns_err() {
        Ok(v)    => v,
        Err(err) => return Err(err),
    };
    let x2 = match returns_ok() {
        Ok(v)    => v,
        Err(err) => return Err(err),
    };
    Ok(x1 + x2)
}

// Same thing with ? — much cleaner
fn f_question_operator() -> Result<u32, String> {
    let x1 = returns_err()?;
    let x2 = returns_ok()?;
    Ok(x1 + x2)
}

// ? only works when error types match (or are convertible).
// returns_bool_err() returns Result<u32, bool> — bool ≠ String, so ? fails.
// Must pattern-match manually and convert.
fn f_mixed_errors() -> Result<u32, String> {
    let x1 = returns_err()?;
    let x2 = returns_ok()?;
    let x3 = match returns_bool_err() {
        Ok(v)  => v,
        Err(_) => return Err("bool error from returns_bool_err".to_string()),
    };
    Ok(x1 + x2 + x3)
}

// =========================================================================
// PART 3: Multiple error types with Box<dyn Error>
// =========================================================================

#[derive(Debug)]
enum MathErr {
    DivByZero,
}

#[derive(Debug)]
enum ParseErr {
    InvalidInt,
}

fn math_fail() -> Result<u32, MathErr> {
    Err(MathErr::DivByZero)
}

fn parse_fail() -> Result<u32, ParseErr> {
    Err(ParseErr::InvalidInt)
}

// Problem: math_fail and parse_fail return different error types,
// so ? can't automatically convert between them.
//
// Solution: Box<dyn std::error::Error> — a type-erased error trait object.
// Any type that implements Error can be boxed and returned uniformly.
fn combined() -> Result<(), Box<dyn std::error::Error>> {
    math_fail()?;
    parse_fail()?;
    Ok(())
}

// To use Box<dyn Error>, the error types must implement std::error::Error.
// Error requires Display + Debug. Debug is derived; Display must be written by hand.
impl std::error::Error for MathErr {}
impl std::error::Error for ParseErr {}

impl std::fmt::Display for MathErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error: {:?}", self)
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {:?}", self)
    }
}

fn main() {
    println!("=== basics ===");
    basics();

    println!("\n=== unwrap / expect ===");
    unwrap_and_expect();

    println!("\n=== ? operator ===");
    let z = question_operator_usage();
    println!("question_operator_usage() = {:?}", z);

    let z = f_question_operator();
    println!("f_question_operator() = {:?}", z);

    println!("\n=== Box<dyn Error> ===");
    let z = combined();
    println!("combined() = {:?}", z);
}
