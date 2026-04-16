#[allow(unused)]
use std::mem;

fn borrow(s: &[i32]) {
    println!("borrow {:?}", s);
}

fn borrow_mut(s: &mut [i32]) {
    println!("borrow mut {:?}", s);
    s[0] = 1231;
}
/*
    Rust enforces usage of usize for array indices
*/
fn split_at(s: &[i32], i: usize) -> (&[i32], &[i32]) {}

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

    // Mutable slice
    // mut here means, we can do arr = [2, 3, 4], or mutate the actual arr via arr binding, like this a[0] = 20;
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Now we can do both of these
    // arr[0] = 23;
    // Note when doing the below, size should be same as before (because type of arr is still the same [i32, 5])
    // arr = [12, 123, 123, 4, 1];

    // Again, this means we can't do s = &mut arr2[..], but we can do s[0] = 3
    // But let s: &mut = &mut arr[..] will let us do both the things
    let s = &mut arr[0..3];
    // Because it was borrow, we still have access to s
    borrow_mut(s);
    println!("{:?}", s);

    // Split slice
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
}
