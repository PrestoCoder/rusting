#![allow(unused)]

// Mutabitlity
fn mutability() {
    // In rust, binding is analogous to variable in other languages
    // Binding can be thought of as, for x = 3, x is bound to the memory which holds 3.
    let a = 1;

    // Immutability for binding
    /*
        Immutability in binding means, 
        1. The binding can't be bound to a different address, 
        2. It can't be used directly(using the binding itself) or as a borrow (mutable borrow), to change the value its bound to
        
        For example:- (Stack)

        let a = 2;
        Now, a is the binding, that is bound to the memory address on the stack, which holds the value 2.
        By definition, we can't do a = 3;, because that would try and create a new location in memory for storing the value 3, and bind a to that address
        And the second point also, there is way for literals to be changed without the binding be mutable, so this is taken care of by itself. And obviously we can't do let b = &mut a, because binding 'a' is immutable

        For example:- (Heap)

        let s = String::from("abc");
        Now, s is a binding to the String struct that lives on the stack, which holds 
        {
            Address of heap where string bytes are stored,
            string metadata: {length of the string, capacity of the string}
        }
        
        According         

     */
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
