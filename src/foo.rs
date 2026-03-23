#![allow(unused)]

// main.rs owns both outer_module and outer_module_2, it can see inside them
// Can imagine it as an outer module encapsulating both modules
// However, sibling modules can't access each other,
// they need to access via super, or crate or some commonaccess point.

mod outer_module_2 {
    pub fn print() {
        println!("outer_module_2 - name of the module");
    }
}

mod outer_module {
    // Goes to level above outer_module, sees if there is a module called outer_module_2.
    // The thing to notice is, both modules, are visible to the main function.\
    // Without using "use", its only for the visibility of one module in another
    use super::outer_module_2;

    pub fn fn_for_outer_module_2() {
        // In order to use this module in the outer_module module, need to import it, using super
        outer_module_2::print();
    }

    pub fn print() {
        println!("my - name of the module");
    }

    // Can call this private function inside the function without any problem.
    // But will have to call using pub if we want to use it outside.
    fn priv_fn() {
        nested_module::print_2();
    }

    // Nested module
    // To use this module outside the parent module, need to make it public
    pub mod nested_module {
        // To be able to use print from outer_module_2
        use super::super::outer_module_2::print;

        // To be used outside this module, even in the parent module, needs to be public
        pub fn print_2() {
            println!("nested module - name of the module");

            let s2 = NestedModuleStruct2 {
                id: 25,
                name: "bandi".to_string(),
            };
        }

        // To use outside nested_module, need to declare public
        #[derive(Debug)]
        pub struct NestedModuleStruct {
            // To access fields outside struct, need to be public
            pub id: i32,
            pub name: String,
        }

        // Coz this is not public, its accessible only in the outer module
        // Also coz its fields are not public, they're accessible only in this outerModule
        struct NestedModuleStruct2 {
            // To access fields outside struct, need to be public
            id: i32,
            name: String,
        }
    }
}
