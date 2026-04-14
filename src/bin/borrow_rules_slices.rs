#[allow(unused)]
use std::mem;

fn borrow(s: &[i32]) {
    println!("borrow {:?}", s);
}

fn main() {
    // This array lives on the stack, so its like any literal.
    // The symbol table has arr name and address where its stored on stack
    // When compiled, arr symbol is stripped, and only that remains is the actual address
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // This should be a fat pointer, which has the address of the bytes (Starting) and the length
    let s = &arr[0..2];
    // Because this doesn't take ownership, we can print the slice later
    borrow(s);
    println!("s:- {:?}", s);

    // mut here means,
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
}
