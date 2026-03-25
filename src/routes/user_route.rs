
use crate::models::user_model::print_user_model;
use super::health_route::print_health_route;

// Here, we shorten the paths on print_user_route with 'use'
pub fn print_user_route_with_use() {
    println!("user_route");
    print_user_model();
    print_health_route();
}

pub fn print_user_route() {
    println!("user_route");
    crate::models::user_model::print_user_model();

    // crate::routes::health_route::print_health_route();
    // Instead of the above, we can do this
    // Refer to the module tree
    super::health_route::print_health_route();
}