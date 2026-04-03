#![allow(unused)]

mod func;

// Borrowing
fn main() {
    let s = String::from("abc");
    /*
       NOTE:-
       The s is a struct, with address of the string on heap, length and capacity.
       When its non mutable, we can't alter the address of string it points to, length or capacity.
       => We can't change the string, as that'll change its length, nor assign the owner to a new string because that changes the address/length

       When mutable, because it can alter all those things, and thus the string it points to, or make it point to a new string.
    */

    // Immutable borrow
}
