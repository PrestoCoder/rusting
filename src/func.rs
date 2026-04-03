#![allow(unused)]

fn take_ownership(s_new: String) {
    // the function signature does this:- let s_new = s(passed argument)
    println!("{}", s_new);
}

// Ownership
fn main_fn() {
    /*
       Data can be stored at 2 places
       Stack and heap
       # Stack
       - Fixed memory allocated before runtime => Stores fixed size data whose size is known at compile time
       - Fast, LIFO

       # Heap
       - Stores data of unknown size at compile time/size that can change
       - Slower than stack (ob)
       - This is the data that's actually managed by ownership and borrowing rules
       - Things like i32, etc simply get their values copied
    */

    // Ownership rules
    // 1. Each value has an owner
    // 2. There can only be 1 owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // 1.
    let s0 = String::from("rust");

    // 2.
    // transfers ownership (bcoz there can only be 1 owner)
    let s2 = s0;
    // Gives error
    // println!("{}", s);

    // Owner of the value is now s3
    let s3 = s2;

    /*
        What actually happens is:-
        s(on stack) -> points to string (on heap)
        when s1 = &s
        s1(on stack) -> points to s(on stack)
    */

    // 3.
    let s = String::from("ho hohaha");
    {
        let s2 = s;
    }
    // Error - when s2 got dropped, because it got ownership from s, the string it owned became lavaris
    // println!("{}", s);

    // NOTE:- The ownership also gets transferred when the owner itself is passed into a function
    // For ex:-
    let s = String::from("time pass");
    take_ownership(s);
    // Below gives error
    // println!("{}", s);
}
