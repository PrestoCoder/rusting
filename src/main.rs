#![allow(unused)]
mod error_basic;

#[derive(Debug)]
enum MathError {
    DivByZero,
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

fn f1() -> Result<u32, MathError> {
    return Err(MathError::DivByZero);
}

fn f2() -> Result<u32, ParseError> {
    return Err(ParseError::InvalidInt);
}

// The problem below is, f1, f2 return 2 different types of errors.
// Without pattern matching one of them to be able to return error same as the other,
// There is no other way to use ?, so what do we do?
// fn f3() -> Result<(), ()> {
//     f1()?;
//     f2()?;
//     return Ok(());
// }

// The hack is to use the below
// This means that the function can return anything which implements std::error::Error
fn f3() -> Result<(), Box<dyn std::error::Error>> {
    f1()?;
    f2()?;
    return Ok(());
}

// Below asks to implement Display/Debug for these enums
// Because it is required if something implements Error
// Debug is derived, but Display can't be
impl std::error::Error for MathError {}
impl std::error::Error for ParseError {}

// For now, just accept this is how Display can be implemented
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}

fn main() {
    // If we were to use ? down below, would have to return dyn err from main too
    // Result<(), Box<(dyn std::error::Error + 'static)>>  to be exact
    let z = f3();
    println!("z = {:?}", z);
}
