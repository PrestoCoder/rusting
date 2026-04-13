#![allow(unused)]

fn main() {
    // loop — like while(true); only breaks when you say so
    println!("--- loop ---");
    let mut i = 0;
    loop {
        if i == 5 { break; }
        println!("{i}");
        i += 1;
    }

    // while
    println!("--- while ---");
    let mut i = 0;
    while i <= 5 {
        println!("{i}");
        i += 1;
    }

    // for with a range (5 exclusive; use 0..=5 for inclusive)
    println!("--- for range ---");
    for i in 0..5 {
        println!("{i}");
    }

    // for over an array (moves elements — fine for Copy types)
    println!("--- for array ---");
    let arr = [1, 2, 3];
    for a in arr {
        println!("{a}");
    }

    // for over array with index
    println!("--- for array with index ---");
    for i in 0..arr.len() {
        println!("arr[{i}] = {}", arr[i]);
    }

    // for over a vector
    // `for x in v` calls v.into_iter(), which CONSUMES v (ownership moves in).
    // Use `&v` or `.iter()` to borrow instead.
    println!("--- for &vec ---");
    let v = vec![1, 2, 3, 4];
    for x in &v {
        println!("{x}");
    }
    // v is still usable here because we borrowed

    // .iter() is the explicit borrow form; also unlocks .iter_mut()
    println!("--- for vec.iter() ---");
    for x in v.iter() {
        println!("{x}");
    }

    // Return a value from loop with break
    // Only `loop` (not while/for) can return a value this way
    println!("--- loop return value ---");
    let mut i = 0;
    let out = loop {
        if i == 3 { break 99; }
        i += 1;
    };
    println!("loop returned: {out}");

    // Labels — break/continue a specific outer loop by name
    println!("--- labeled loops ---");
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            if i + j < 4 {
                println!("{i} - {j}");
            } else {
                break 'outer;
            }
        }
    }
}
