use std::str::FromStr;

// Structs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // STATIC METHOD
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // METHODS
    // mut so that we can mutate self
    // &mut so that when we call this function, the ownership doesn't get transferred to the self here.
    // for ex: point_1.move_to(2.0, 3.0); - will transfer ownership from point_1 to self.
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    // I think it might seldom happen we'd have to pass self as self and not &self, unless we want access to that object forever.
    fn dist(&self) -> f32 {
        // The function sqrt is being called on the f32 being return in the calculation below.
        // Rust allows implementing primitives too, just that we can't do it ourselves, but rust does it for us.
        // Primitives are not implemented as structs or anything, implementation is baked at the compiler level.
        // I think it means that when sqrt is called, unlike something like java, it won't call a struct method.
        // But the method that was baked onto the f32 primitive.
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Refer to the values inside it like for tuples
// Which is why called tuple structs
struct Point3d(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn arr_fn(arr: &[u8]) {
    println!("arr's 0th element:-{}", arr[0]);
}

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn fun1() {
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

fn fun2() {
    let v: Vec<_> = vec!["1"];
    let x: _ = ();
    let x = 2;
    let x = 4;

    let i32_min = i32::MIN;
    let u8_max = u8::MAX;

    println!("i32 ka min:- {i32_min}");
    println!("u8 ka max:- {u8_max}");

    let char_min = char::MIN;

    println!("char ka min:- {char_min}");

    let c: char = 'a';
    let num_char = c as i32;
    println!("a ka numchar hai:- {num_char}");

    let a = u8::MAX;
    let b = a + 1;
    let c = u8::checked_add(a, 1);
    let d = u8::wrapping_add(a, 1);

    println!("{a}");
    println!("{b}");
    println!("{:?}", c);
    println!("{d}");
}

fn fun3() {
    let t: (bool, i32, char) = (true, 1, 'c');
    // destructuring
    let (a, b, c) = t;
    println!("a={a}\nb={b}\nc={c}\nt={:?}", t);

    // selective destructuring
    let (_, b, _) = t;

    // Empty tuple = unit type ~ void
    let t = ();

    // nested tuple
    let nested: ((f64, char), (bool, u8, char), ()) = ((1.23, 'a'), (true, 1u8, 'b'), ());

    // accessing elements of tuples
    println!("0th element:- {:?}", nested.0);
    println!("1st element:- {:?}", nested.1);
    println!("2nd element:- {:?}", nested.2);

    println!("nested's 0th element's 0th element:- {}", nested.0.0);
}

fn fun4() {
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[2] = {}", arr[2]);

    // TO be able to mutate array
    let mut arr: [u8; _] = [1, 2, 3];
    arr[0] = 2;

    println!("arr's length = {}", arr.len());

    // array slices
    let mut arr_slice_0: &[u8; 3] = &arr;
    println!("arr_slice_0's length = {}", arr_slice_0.len());
    let arr_slice_1: &[u8] = &arr;
    println!("arr_slice_1's length = {}", arr_slice_1.len());
    let arr_slice_2: &[u8] = &arr[0..1];
    println!("arr_slice_2's length = {}", arr_slice_2.len());
    let arr_slice_3: &[u8] = &arr[0..2];
    let mut arr_slice_4: &[u8] = &arr[0..3];
    arr_slice_4 = &arr[..1];
}

fn fun5() {
    // String: vector of u8(Vec<u8>), valid UTF-8
    // Usage: mutate or data needs to be owned
    // &str: slice of u8 (&[u8]) valid UTF-8
    // Usage: read only

    let my_string = String::from("Aa gya ni ohi bilo tam");
    let s: &str = &my_string[0..];

    let len_str: usize = s.len();
    let s2 = &my_string[0..5];
    let s2_len = s2.len();

    // Hard coded inside binary (tbs: inside read-only memory part of the binary)
    let s3 = "hello rust";

    // multi-line string literal
    let multi_line_str = r#"
        pehli line
        dusri line
        teesri line
    "#;
    println!("{multi_line_str}");

    // Deref coercion
    let a = String::from("ek do teen");
    // b is a pointer to the struct String on stack, which contains len, cap, pointer to actual string on heap
    let b: &String = &a;
    // This is a usual string slice pointer to the actual string on the heap.
    // Note actual type of the data underlying beneath should be same
    let c: &str = &a;

    // Adding &str to String
    let mut msg = "Hello Rust".to_string();
    msg += "!";
    println!("{}", msg);

    // String interpolation
    let lang = "Rust";
    let emoji = ":)";
    // Same kaam, bas print ni kar ra turant, jaise println! kar deta
    let msg = format!("Hello {lang} {emoji}");
    println!("{msg}")
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    // tuple variant
    Green(i32, String, String),
    // Struct variant
    Blue { a: u32, b: u8 },
}

struct Sample {
    a: i32,
    b: u32,
    c: Option<Color>,
}
fn fun6() {
    // ENUMS - finite options
    // The options are variants, the names are essentially just identifiers.
    // At compile time, they become option 1, 2, 3,...

    let red_color: Color = Color::Red;
    let green_color: Color =
        Color::Green(2, "balle green oye".to_string(), "kuchh nai".to_string());
    let blue_color: Color = Color::Blue { a: 1, b: 2 };
    // Debug trait
    println!("{green_color:?}");

    // PartialEq
    // For enums and structs, when all fields are not comparable,
    // For example, if there is a f32, that is not comparable, because float can take value NaN
    // And NaN == NaN is false.

    // Otherwise we can use Eq.

    // This makes more sense when we require function parameters to implement the traits, so that no parameter which let's say doesn't implement Eq => has partialEq
    // makes it to the function as an argument.

    // Note: partialEq still gives false for NaN == NaN, but atleast allows it.
    println!(
        "{}",
        Color::Red == Color::Green(1, "1".to_string(), "2".to_string())
    );

    // ---------------------Option-------------------------
    // Mental model - think like a road branching into two. -> for enums too
    // And the data type inside it is the content you find on that branch.
    // nested enums/options/results => roads keep branching out.

    // Some(value) | None
    let a: Option<String> = Some("oye hoye".to_string());
    let b: Option<String> = None;
    let c: Option<Option<Option<Option<Sample>>>> = Some(Some(Some(None)));
    let d: Option<Option<Sample>> = Some(Some(Sample {
        a: 2,
        b: 3 as u32,
        c: Some(Color::Blue { a: 2, b: 3 }),
    }));

    // -------------------Result-----------------------------
    // Note: Err is just a variant of enum result, nothing else

    let res: Result<f32, String> = Ok(32 as f32);
    let res2: Result<f32, String> = Err("chud gye guru".to_string());

    println!("{res:?}");
}

fn fun7() {
    // Structs-1
    let c: Circle = Circle {
        center: Point { x: 1_f32, y: 2_f32 },
        radius: 2_u32,
    };
    println!("circle:- {c:?}");

    let p = Point3d(32_f32, 32 as f32, 32.0);
    println!("3d points are:- {} {} {}", p.0, p.1, p.2);

    let empty = Empty;

    // Shortcut
    let x = 2_f32;
    let y = 3_f32;
    // instead of {x: x, y: y}
    let p_2 = Point { x, y };

    // Copy - copies values, deep copy
    let p_3 = Point { ..p_2 };
    let mut p_4 = Point { x: 3.3, ..p_2 };

    // Update
    p_4.x += 2.0;
    p_4.y += 3.0;
}

fn fun8() {
    // Struct methods
    // Associated functions - static methods - associate with the struct itself - no separate copy for separate instance
    // Methods -  associated with the struct instance - separate for each instanstiation

    let mut p = Point::zero();
    println!("{p:?}");

    p.move_to(2.0, 3.0);
    println!("{p:?}");

    let d = p.dist();
    println!("{d}");
}

fn fun9() {
    // IF ELSE

    let x = 2;
    if (x % 2 == 0) {
        println!("{x} is even");
    } else {
        println!("{x} is odd");
    }

    // If else can also be used to return values
    // {..} are expressions in rust, they always evaluate and hence return something.

    let z = if x > 0 {
        10
    } else if x == 3 {
        12
    } else {
        34
    };

    println!("{z}");
}

fn fun10() {
    // LOOPS

    // loop - similar to while(1) - breaks only when we tell it to
    println!("first loop - loop");
    let mut i = 0;
    loop {
        if (i == 5) {
            break;
        }
        println!("{i}");
        i += 1;
    }

    // while loop - standard while loop
    println!("Second loop - while");
    let mut i = 0;
    while (i <= 5) {
        println!("{i}");
        i += 1;
    }

    // for loop - 5 exclusive
    // For inclusive - 0..=5
    println!("3rd loop - for");
    for i in 0..5 {
        println!("{i}");
    }

    // for loop - array
    // looping over all elements of the array
    println!("4th loop - for loop array");
    let arr = [1, 2, 3];
    for a in arr {
        println!("{a}");
    }

    // for loop - array
    // with index
    // length of array, strings, vectors is always at max usize
    println!("5th loop - for loop array with index");
    let n: usize = arr.len();
    for i in 0..arr.len() {
        println!("element at index {i} = {}", arr[i]);
    }

    // for loop - vector
    // The only issue below is when we do for x in v, v gets passed on to some function, that helps iterate over it.
    // And because we're passing vector as is, ownership gets transferred.
    // We could use clone or reference to make it work here.
    println!("6th loop - for loop vector");
    let v = vec![1, 2, 3, 4];
    for x in &v {
        println!("{x}");
    }

    for x in &v {
        println!("{x}");
    }

    // for x in v ~ for x in v.into_iter(), which consumes v, &v is not explicit, cloning is wasteful.
    // .iter() is explicit, it borrows each element of the vector.
    // also provides more functionality with .iter_mut, although for the latter, the vector itself must be declared mutable.
    // NOTE:- Any data structure that implements iter can be iterated, like hashmaps, arrays and vectors.
    println!("7th loop - for loop vector with iter");
    for x in v.iter() {
        println!("{x}");
    }

    // Return a value from a loop
    // Can only do it with "loop"
    println!("8th loop - returning a value with \"loop\" ");
    let mut i = 0;
    let out = loop {
        if i == 3 {
            // Means, when loop breaks, it returns 99, which gets stored in out.
            break 99;
        }
        i += 1;
    };
    println!("return loop {out}");

    // Labels - useful when have multiple loops and we need to break out of an outer loop
    // Can use label of outermost loop to break out of it
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            if (i + j < 4) {
                println!("{i} - {j}");
            } else {
                break 'outer;
            }
        }
    }
}

fn fun11() {
    // MATCH statement
    // In these kinds of cases, match statement works better
    let x = 1;
    if x == 1 {
        println!("one");
    } else if x == 2 {
        println!("two");
    } else if x == 3 {
        println!("three");
    } else {
        println!("other");
    };

    // With Match it becomes
    // For other, use _, as shown below
    let x = 4;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    };

    // If lets say, for x = 1, 2, 3, we want the same output.
    // Defining it 3 times is redundant,
    // for this, we can use 1 | 2 | 3
    // for range - 0..10, or 0..=10
    // And to know exactly which number matched from range - use 'i @ '
    match x {
        1 | 2 | 3 => {
            println!("sahi hai");
            println!("aage badho");
        }
        i @ 4..=10 => {
            println!("Between 4 and 10 inclusive, matched - {i}");
        }
        _ => {
            println!("galat hai");
            println!("dafa ho");
        }
    }

    // Return value from match
    // Similar to if-else, expressions inside return value
    // Also match enforces us to cover all cases, like for enums, it will force covering all variants.
    enum Animal {
        Cat,
        Dog,
        Duck,
        Mouse,
    }

    let animal = Animal::Cat;
    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Duck => "quack",
        _ => "laude na bhojyam",
    };
    println!("{animal_sound}");

    // Match with Option and Result
    // It can be used to destructure enums, like Option and result
    // For ex.
    let opt: Option<(&str, i32, f64, Option<i32>)> = Some(("yoyo", 345, 2.01, Some(32)));
    match opt {
        Some((s, i, f, o)) => match o {
            Some(v) => println!("thak gye {s} {i} {f} {v}"),
            None => println!("khatam, tata, bye bye"),
        },
        None => {
            println!("arey bc, None hai");
        }
    }

    // The above Option is a composite option - option within an option
    // We can do it another way too, instead of handling two cases for nested option after matching Some once.
    // Instead we can handle all three cases at the same level, that is the first level itself.

    match opt {
        Some((s, i, f, Some(v))) => {
            println!("pehle case");
        }
        Some((s, i, f, None)) => {
            println!("doosra case");
        }
        None => println!("teesra cases"),
    }

    // Similarly can handle result
    // let r: Result<i32, String> = Ok(32);
    let r: Result<i32, String> = Err("maa chud gyi".to_string());
    match r {
        Ok(v) => println!("value hai {v}"),
        Err(e) => println!("kya kiya bhosadi - {e}"),
    }
}

fn fun12() {
    // IF-LET
    let x = Some(123);
    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }

    // if let - Do something similar to above
    // Only that it doesn't force you to handle all cases
    // It doesn't do anything for the None case.
    // Also, the below could be viewed as destructuring of some sort.
    // Just like we can do it for tuples

    let a = (1, 2, 3);
    let (g, h, j) = a;

    // However, for tuples we are sure that we can get a destructured value.
    // But with enums, we don;t know for sure, because enum has different variants, so it could be Some or None.x
    // Bcoz of which we could use match statement or if let.
    // Can use else with it like normally.
    if let Some(v) = x {
        println!("if let {v}");
    } else {
        println!("None nikla");
    }

    // let else - the else needs to diverge, => should return or panic
    let Some(v) = x else {
        panic!("All hell broke lose");
    };

    /*

    **NOTE**
    In Rust, when we do 'let a = ..', a needs to get a value/binding to the value.
    With let Some(v) = x else ..., if we don't panic, it could be that we end up with v
    which is undefined. To avoid this, we diverge, that is panic or return.

    But with 'if let ... ', the binding is optional, because of the if.
    So, if x is not of the type Some, v will never get bound in the first place.

    */
}

fn fun_that_panics() {
    loop {}
}

fn fun_that_never_returns() -> ! {
    panic!()
}

fn fun13() {
    // FUNCTIONS
    /*
    The new thing in the tutorial was returning !
    In rust, '!' is a type, called 'never' type
    For example, u32 has so many values it can take.
    bool can take two values true and false.
    unit type, (), has one value, ().
    '!' has no value, hence the name never return type.
    Which means, its presence means, that thing/function will never return.

    In practice, ! shows up as the return type of:

        panic!()
        return
        break
        continue
        loop {} (infinite loop with no break)
        std::process::exit()

    Which is why, the never return type can be used anywhere.
    Because its presence can be sure that the flow of the code won't return anything,
    And instead diverge, or never return.

    For example
    match x {
        Some(v) => println!("balle"),
        None => panic!
    }

    So, if the flow of code is from match, to then checking what x is, if its some, flow of code will return the value to v, and then move on.
    However, if its None, panic will abort the thread itself,
    Which makes me think, '!' means it never returns, and thus ends up panicking or aborting the thread.
    Its more like absence of a return type.
    */
}
