#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green(i32, String, String), // tuple variant
    Blue { a: u32, b: u8 },     // struct variant
}

struct Sample {
    a: i32,
    b: u32,
    c: Option<Color>,
}

fn main() {
    // ENUMS — finite options
    // Variants are essentially named options; at compile time they become 0, 1, 2...

    let red_color: Color = Color::Red;
    let green_color = Color::Green(2, "balle green oye".to_string(), "kuchh nai".to_string());
    let blue_color = Color::Blue { a: 1, b: 2 };
    println!("{green_color:?}");

    // PartialEq — allows == comparisons
    // Use Eq when ALL fields are fully comparable (no f32/NaN risk)
    // PartialEq still returns false for NaN == NaN, but at least compiles
    println!("{}", Color::Red == Color::Green(1, "1".to_string(), "2".to_string()));

    // ---------------------Option-------------------------
    // Mental model: a road branching into Some(value) or None
    // Nested Option = road keeps branching

    let a: Option<String> = Some("oye hoye".to_string());
    let b: Option<String> = None;
    let c: Option<Option<Option<Option<Sample>>>> = Some(Some(Some(None)));
    let d: Option<Option<Sample>> = Some(Some(Sample {
        a: 2,
        b: 3,
        c: Some(Color::Blue { a: 2, b: 3 }),
    }));

    // -------------------Result-----------------------------
    // Err is just a variant of the Result enum — nothing magic about it

    let res: Result<f32, String> = Ok(32_f32);
    let res2: Result<f32, String> = Err("chud gye guru".to_string());
    println!("{res:?}");
    println!("{res2:?}");
}
