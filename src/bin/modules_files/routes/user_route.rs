use crate::models::user_model::print_user_model;
use super::health_route::print_health_route;

// Using full crate:: path
pub fn print_user_route() {
    println!("user_route");
    crate::models::user_model::print_user_model();
    // `super::` goes up one level (to routes/) then into the sibling module
    super::health_route::print_health_route();
}

// Same thing but with `use` to shorten the paths
pub fn print_user_route_with_use() {
    println!("user_route");
    print_user_model();
    print_health_route();
}
