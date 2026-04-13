#![allow(unused)]

fn main() {
    // String: Vec<u8>, valid UTF-8 — use when you need to own or mutate
    // &str:   &[u8],  valid UTF-8 — use for read-only access

    let my_string = String::from("Aa gya ni ohi bilo tam");
    let s: &str = &my_string[0..];
    let s2 = &my_string[0..5];

    println!("full slice len = {}", s.len());
    println!("partial slice = {s2}, len = {}", s2.len());

    // String literal — baked into the binary's read-only memory
    let s3 = "hello rust";

    // Multi-line raw string literal
    let multi_line_str = r#"
        pehli line
        dusri line
        teesri line
    "#;
    println!("{multi_line_str}");

    // Deref coercion
    let a = String::from("ek do teen");
    let b: &String = &a;   // pointer to the String struct on stack
    let c: &str = &a;      // pointer straight to the heap bytes

    // Appending &str to String
    let mut msg = "Hello Rust".to_string();
    msg += "!";
    println!("{}", msg);

    // String interpolation with format!
    let lang = "Rust";
    let emoji = ":)";
    let msg = format!("Hello {lang} {emoji}");
    println!("{msg}");
}
