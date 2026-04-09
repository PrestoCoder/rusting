fn main() {
    // =========================================================================
    // SETUP
    // =========================================================================
    // Heap-allocated value (String stores ptr+len+cap on stack, bytes on heap)
    let mut s1 = String::from("hello"); // stack: [ptr|5|5] -> heap: [h,e,l,l,o]
    let mut s2 = String::from("world"); // stack: [ptr|5|5] -> heap: [w,o,r,l,d]

    // Stack-only value (i32 is Copy, lives entirely on the stack, no heap)
    let mut n1: i32 = 10; // stack: [10]
    let mut n2: i32 = 20; // stack: [20]

    // =========================================================================
    // CASE 1: let b = &val  (immutable binding + shared ref)
    //
    // b is fixed to this reference. the reference is read-only.
    // nothing can change through b. fully locked.
    // =========================================================================
    {
        let b = &s1;
        // b = &s2;           // ERROR: b is not mut, can't rebind
        // b.push_str("!");   // ERROR: &String has no mutation rights
        println!("1a: {}", b); // "hello" — read only

        let b = &n1;
        // b = &n2;           // ERROR: can't rebind
        // *b = 99;           // ERROR: can't write through shared ref
        println!("1b: {}", b); // 10 — read only
    }

    // =========================================================================
    // CASE 2: let b = &mut val  (immutable binding + mutable ref)
    //
    // b is fixed — can't rebind it to a different reference.
    // BUT the reference itself carries write permission to the target.
    //
    // the binding's immutability only governs "can b point elsewhere?"
    // the &mut governs "can b modify what it points to?"
    // these are two independent axes.
    // =========================================================================
    {
        let b = &mut s1;
        // b = &mut s2;       // ERROR: b is not mut, can't rebind
        b.push_str("!");      // OK: &mut String permits mutation of the heap bytes
        println!("2a: {}", b); // "hello!" — mutated through b

        let b = &mut n1;
        // b = &mut n2;       // ERROR: can't rebind
        *b += 1;              // OK: &mut i32 permits writing the stack slot directly
        println!("2b: {}", b); // 11 — mutated the i32 in place on the stack
    }

    // =========================================================================
    // CASE 3: let mut b = &val  (mutable binding + shared ref)
    //
    // b can be rebound to point at a different value.
    // BUT the reference is shared (&), so no mutation of the target.
    //
    // you can swap which thing you're looking at,
    // but you can't change any of them.
    // =========================================================================
    {
        let mut b = &s1;
        println!("3a: {}", b); // "hello!" (still mutated from case 2)
        // b.push_str("x");   // ERROR: & doesn't grant write access
        b = &s2;               // OK: mut binding, can rebind
        println!("3a: {}", b); // "world"

        let mut b = &n1;
        println!("3b: {}", b); // 11
        // *b = 99;            // ERROR: can't write through &
        b = &n2;               // OK: rebind
        println!("3b: {}", b); // 20
    }

    // =========================================================================
    // CASE 4: let mut b = &mut val  (mutable binding + mutable ref)
    //
    // full access. b can be rebound, AND it can mutate the target.
    // =========================================================================
    {
        let mut b = &mut s1;
        b.push_str("!");       // OK: &mut grants write access -> heap bytes change
        println!("4a: {}", b); // "hello!!" — second mutation
        b = &mut s2;           // OK: mut binding, rebind to s2
        b.push_str("!");       // OK: now mutating s2's heap bytes
        println!("4a: {}", b); // "world!"

        let mut b = &mut n1;
        *b += 100;             // OK: overwrite the i32 on the stack (11 -> 111)
        println!("4b: {}", b); // 111
        b = &mut n2;           // OK: rebind
        *b += 100;             // OK: overwrite n2 on the stack (20 -> 120)
        println!("4b: {}", b); // 120
    }

    // =========================================================================
    // FINAL STATE — proof that mutations went through to the originals
    // =========================================================================
    println!("\nfinal s1: {}", s1); // "hello!!" — mutated twice via refs
    println!("final s2: {}", s2);   // "world!"  — mutated once via ref
    println!("final n1: {}", n1);   // 111       — mutated twice via refs
    println!("final n2: {}", n2);   // 120       — mutated once via ref

    // =========================================================================
    // SUMMARY
    // =========================================================================
    // let     b = &    val  →  can't rebind b, can't mutate val
    // let     b = &mut val  →  can't rebind b, CAN  mutate val
    // let mut b = &    val  →  CAN  rebind b,  can't mutate val
    // let mut b = &mut val  →  CAN  rebind b,  CAN  mutate val
    //
    // "let mut" controls the BINDING  (can b point elsewhere?)
    // "&mut"    controls the REFERENCE (can b write to the target?)
    //
    // for stack values (i32):  mutation overwrites the stack slot directly
    // for heap values (String): mutation modifies the heap allocation
    //                           (the stack metadata ptr/len/cap may update too)
}