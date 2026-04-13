#![allow(unused)]

fn take_ownership(s: String) {
    // The function signature does: let s = passed_argument
    // Ownership of the String transfers here.
    println!("{}", s);
} // s goes out of scope here — String is dropped (heap freed)

fn main() {
    // =========================================================================
    // WHERE DATA LIVES
    // =========================================================================
    //
    // Stack — fixed-size data known at compile time (i32, bool, char, ...)
    //   Fast, LIFO, automatically reclaimed when the frame pops.
    //
    // Heap  — dynamic/growable data (String, Vec, Box, ...)
    //   Managed manually in C/C++; in Rust, managed through OWNERSHIP.
    //
    // String on stack: [ptr | len | cap]  →  heap: [h, e, l, l, o, ...]
    // i32   on stack: [42]                    (no heap involvement)

    // =========================================================================
    // OWNERSHIP RULES
    // 1. Each value has exactly one owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped.
    // =========================================================================

    // Rule 1 — every value has an owner
    let s0 = String::from("rust");

    // Rule 2 — only one owner; assignment MOVES ownership
    let s1 = s0;       // s0's ownership moves to s1
    // println!("{s0}"); // ERROR: s0 no longer owns the value

    let s2 = s1;       // s1's ownership moves to s2
    // println!("{s1}"); // ERROR: same reason

    // Rule 3 — owner going out of scope drops the value
    let s = String::from("ho hohaha");
    {
        let s_inner = s;  // ownership moves into the inner scope
    } // s_inner dropped here — heap memory freed
    // println!("{s}"); // ERROR: s moved into s_inner, which is gone

    // Ownership also transfers when passing to a function
    let s = String::from("time pass");
    take_ownership(s);
    // println!("{s}"); // ERROR: ownership moved into take_ownership()

    // i32 is a Copy type — it lives on the stack and is duplicated, not moved
    let n = 42_i32;
    let n2 = n;        // copy, not move
    println!("both still valid: n={n}, n2={n2}");
}
