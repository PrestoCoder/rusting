#![allow(unused)]

fn main() {
    let i32_min = i32::MIN;
    let u8_max = u8::MAX;

    println!("i32 ka min:- {i32_min}");
    println!("u8 ka max:- {u8_max}");

    let char_min = char::MIN;
    println!("char ka min:- {char_min}");

    let c: char = 'a';
    let num_char = c as i32;
    println!("a ka numchar hai:- {num_char}");

    // Overflow handling
    let a = u8::MAX;
    // checked_add: returns None on overflow
    let c = u8::checked_add(a, 1);
    // wrapping_add: wraps around on overflow
    let d = u8::wrapping_add(a, 1);

    println!("u8::MAX = {a}");
    println!("checked_add(MAX, 1) = {:?}", c); // None
    println!("wrapping_add(MAX, 1) = {d}");    // 0
}
