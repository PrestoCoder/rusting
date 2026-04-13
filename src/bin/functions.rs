#![allow(unused)]

// The ! (never) return type means this function NEVER returns.
// It must diverge: panic, infinite loop, process::exit, etc.
fn diverges_with_panic() -> ! {
    panic!("this never returns")
}

fn diverges_with_loop() -> ! {
    loop {}
}

fn main() {
    // ! is a TYPE in Rust, called the "never" type.
    //
    // Type      | Possible values
    // ----------|-----------------
    // u32       | 0 .. 4_294_967_295
    // bool      | true, false
    // ()        | ()        (exactly one value)
    // !         | (none)    (no value — the function never produces one)
    //
    // Because ! has no value, it is compatible with EVERY type.
    // That's why you can use panic!() (which returns !) in any branch of
    // a match, even one that expects a String or an i32:
    //
    //   let s: String = match opt {
    //       Some(v) => v,
    //       None    => panic!("!"),   // ! coerces to String — fine
    //   };
    //
    // Functions that return ! in practice:
    //   panic!()
    //   return / break / continue   (in diverging positions)
    //   loop {}                     (infinite loop with no break)
    //   std::process::exit()
    //
    // Because the thread/process terminates before returning,
    // the compiler accepts ! wherever any type is expected.

    println!("functions binary — see source for ! (never type) notes");

    // Uncomment to see divergence in action (will panic):
    // diverges_with_panic();
}
