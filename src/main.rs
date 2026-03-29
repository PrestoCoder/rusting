mod error_basic;

fn f1() -> Result<u32, String> {
    println!("f1");
    // Ok(1)
    return Err("gande phatti".to_string());
}

// For anything to be returned by main function, it must implemetn Termination trait.
// u32 doesn't implement it, => Result<u32, _> can't be returned from it.
// This trait decides the exit status code using the output of the main function.
fn main() -> Result<(), String> {
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
