#![allow(unused)]

fn main() {
    let t: (bool, i32, char) = (true, 1, 'c');

    // Destructuring
    let (a, b, c) = t;
    println!("a={a}\nb={b}\nc={c}\nt={:?}", t);

    // Selective destructuring
    let (_, b, _) = t;

    // Empty tuple = unit type ~ void
    let unit = ();

    // Nested tuple
    let nested: ((f64, char), (bool, u8, char), ()) = ((1.23, 'a'), (true, 1u8, 'b'), ());

    // Accessing elements with dot notation
    println!("0th element:- {:?}", nested.0);
    println!("1st element:- {:?}", nested.1);
    println!("2nd element:- {:?}", nested.2);
    println!("nested's 0th element's 0th element:- {}", nested.0.0);
}
