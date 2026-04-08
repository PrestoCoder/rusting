#![allow(unused)]

// Mutabitlity
fn mutability() {
    // binding immutable
    // a is binding, its bound to value 1 on stack
    // binding immutable means, a can't be bound to another value
    let a = 1;
    // s is bound to the string struct on the stack
    // That struct is a fat pointer, which has the actual heap address where the bytes of the string are present
    // It also has the capacity and length of the string

    // This means that s binding is immutable, which means that it can be bound to a different String struct
    // It also means that either s, or any borrow of it, can't be used to mutate the value of the String it points to.
    // Put simply, s can't be changed or used to change.
    // let s = String::from("abc");

    // This implies, the below fail
    // In place fails
    // s.push_str("abc");

    // This fails -> s can't be bound to a new address (because the below creates a new string on heap, which in turn gets address stored on stack)
    // s = String::from("abcc");

    // This also - because s is immutable, we shouldn't be able to borrow it mutably
    // let s2 = &mut s;

    // However, this is possible
    // let mut s3 = &s;

    // What does this mean,
    // This means, unlike above, when s3 binding is immutable, which means below should fail (it can't be bound to a different address on stack)
    // let s3 = &s;
    let s_1 = String::from("abc");
    // s3 = &s_1;
}

fn main() {
    // Borrow in effect is exactly like a pointer in c++
    // Also, unless specified explicitly, it is stored on the stack
    // Creates a reference
    // Doesn't move ownership
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // Above are immutable, read only borrows.
    // Because they aren't mut, they can't be used to mutate

    // Immutable borrow -----------------
    let s = String::from("rust");
    let s1 = &s;

    // We can have any number of read-only access to a value (immutable borrow)
    // Below array has &s as each of its 10 elements
    let mut borrow_arr: [&String; 10] = [&s; 10];

    // Because pointer is on stack, "=" copies the value
    let s3 = s2;
    println!("{s2}");

    // Mutable borrow ---------------
    // write access
    // Can only be provided if the original binding is mutable
    let mut s = String::from("abc");
    let s1 = &mut s;
    s1.push_str("balle balle");
}
