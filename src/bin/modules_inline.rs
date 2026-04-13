#![allow(unused)]

// main.rs is the root of the module tree.
// It "owns" every module declared here, so sibling modules can't see
// each other directly — they must go through a common ancestor via
// `super::` (parent) or `crate::` (root).

mod outer_module_2 {
    pub fn print() {
        println!("outer_module_2 - name of the module");
    }
}

mod outer_module {
    // `use super::outer_module_2` brings the sibling into scope.
    // Both modules are visible from main.rs (their common parent),
    // but NOT directly from each other without this import.
    use super::outer_module_2;

    pub fn fn_for_outer_module_2() {
        outer_module_2::print();
    }

    pub fn print() {
        println!("outer_module - name of the module");
    }

    // Private function — callable within this module, not outside
    fn priv_fn() {
        nested_module::print_2();
    }

    // Nested module — must be `pub` to be reachable from outside outer_module
    pub mod nested_module {
        // `super::super` climbs two levels: nested_module -> outer_module -> root
        use super::super::outer_module_2::print;

        // Must be `pub` to be called outside nested_module (even from its parent)
        pub fn print_2() {
            println!("nested module - name of the module");

            // NestedModuleStruct2 is private — only usable inside this module
            let s2 = NestedModuleStruct2 {
                id: 25,
                name: "bandi".to_string(),
            };
        }

        // Public struct — accessible outside nested_module
        #[derive(Debug)]
        pub struct NestedModuleStruct {
            pub id: i32,       // pub fields — readable outside
            pub name: String,
        }

        // Private struct — only usable inside nested_module
        struct NestedModuleStruct2 {
            id: i32,
            name: String,
        }
    }
}

fn main() {
    outer_module::print();
    outer_module::nested_module::print_2();
    outer_module_2::print();

    // Accessing a public struct from a nested module
    let s = outer_module::nested_module::NestedModuleStruct {
        id: 28,
        name: "Rohan".to_string(),
    };
    println!("s = {:?}", s);
}
