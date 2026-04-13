#![allow(unused)]

// Named field struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Associated function (static method) — called as Point::zero()
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // &mut self — borrow mutably, don't take ownership
    // Without &, point.move_to() would move ownership into self and
    // the caller could never use point again.
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    // &self — borrow immutably for reading
    fn dist(&self) -> f32 {
        // sqrt is baked onto f32 at the compiler level (not a real struct method)
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Tuple struct — fields accessed as p.0, p.1, p.2
struct Point3d(f32, f32, f32);

// Unit struct — no fields, useful as marker types
struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // Creating structs
    let c = Circle {
        center: Point { x: 1_f32, y: 2_f32 },
        radius: 2_u32,
    };
    println!("circle:- {c:?}");

    let p = Point3d(32_f32, 32_f32, 32.0);
    println!("3d points:- {} {} {}", p.0, p.1, p.2);

    let empty = Empty;

    // Shorthand init when variable names match field names
    let x = 2_f32;
    let y = 3_f32;
    let p2 = Point { x, y };   // instead of { x: x, y: y }

    // Struct update syntax — copy remaining fields from another instance
    let p3 = Point { ..p2 };
    let mut p4 = Point { x: 3.3, ..p2 };
    p4.x += 2.0;
    p4.y += 3.0;

    // Methods
    let mut p = Point::zero();
    println!("{p:?}");

    p.move_to(2.0, 3.0);
    println!("{p:?}");

    let d = p.dist();
    println!("distance from origin: {d}");
}
