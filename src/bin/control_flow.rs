#![allow(unused)]

fn main() {
    // =========================================================================
    // IF / ELSE
    // =========================================================================
    let x = 2;
    if x % 2 == 0 {
        println!("{x} is even");
    } else {
        println!("{x} is odd");
    }

    // if-else is an expression — it evaluates to a value
    let z = if x > 0 {
        10
    } else if x == 3 {
        12
    } else {
        34
    };
    println!("z = {z}");

    // =========================================================================
    // MATCH
    // =========================================================================
    let x = 4;

    // Basic match — _ is the catch-all
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    };

    // OR patterns, ranges, binding with @
    match x {
        1 | 2 | 3 => {
            println!("sahi hai");
        }
        i @ 4..=10 => {
            println!("Between 4 and 10 inclusive, matched - {i}");
        }
        _ => {
            println!("galat hai");
        }
    }

    // match is also an expression — returns a value
    // It also enforces exhaustiveness: must cover all variants
    enum Animal { Cat, Dog, Duck, Mouse }
    let animal = Animal::Cat;
    let sound = match animal {
        Animal::Cat  => "meow",
        Animal::Dog  => "woof",
        Animal::Duck => "quack",
        _            => "...",
    };
    println!("{sound}");

    // Destructuring inside match (Option and nested Option)
    let opt: Option<(&str, i32, f64, Option<i32>)> = Some(("yoyo", 345, 2.01, Some(32)));

    // Two-level match
    match opt {
        Some((s, i, f, o)) => match o {
            Some(v) => println!("thak gye {s} {i} {f} {v}"),
            None    => println!("khatam"),
        },
        None => println!("arey bc, None hai"),
    }

    // Flatten nested Option into one match
    match opt {
        Some((s, i, f, Some(v))) => println!("pehle case: {s} {i} {f} {v}"),
        Some((s, i, f, None))    => println!("doosra case"),
        None                     => println!("teesra case"),
    }

    // Matching Result
    let r: Result<i32, String> = Err("maa chud gyi".to_string());
    match r {
        Ok(v)  => println!("value hai {v}"),
        Err(e) => println!("error: {e}"),
    }

    // =========================================================================
    // IF LET / LET ELSE
    // =========================================================================
    let x = Some(123);

    // match version
    match x {
        Some(v) => println!("Some {v}"),
        _       => {}
    }

    // if let — only handle the case you care about, skip the rest
    if let Some(v) = x {
        println!("if let {v}");
    } else {
        println!("None nikla");
    }

    // let-else — the else MUST diverge (return / panic / break / continue)
    // This guarantees v is always bound after this point.
    let Some(v) = x else {
        panic!("All hell broke loose");
    };
    println!("let-else bound v = {v}");

    /*
    NOTE:
    'if let' is optional binding — if the pattern doesn't match, v is never
    bound, so there's no problem.

    'let ... else' is unconditional binding — if the pattern doesn't match
    we MUST diverge so v is never left in an undefined state.
    */
}
