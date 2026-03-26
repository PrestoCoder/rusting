mod error_basic;

fn main() {
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
    let v = x.unwrap();
    println!("value is {v}");    
    // The above statement is equivalent
    // let v = match x {
    //     Ok(val) => val,
    //     Err(err) => panic!("error:- {:?}", err)
    // };
    // println!("value is {v}");

    // Expect
    


}
