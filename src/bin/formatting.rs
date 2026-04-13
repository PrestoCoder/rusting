#![allow(unused)]

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    let x = 2;
    let ans = format!("{0} x {0} = {1}", x, x * x);
    println!("{}", ans);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.28".to_string(),
    };
    println!("{:?}", lang);
    println!("{:#?}", lang);
}
