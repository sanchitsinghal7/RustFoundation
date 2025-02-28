#![allow(unused_variables)]
#![allow(dead_code)]

mod foundation; // Import the foundation module

fn main() {
    println!("Hello, world!");
    foundation::basic_types_and_vars::boolean_demo();
    foundation::basic_types_and_vars::unsigned_integers_demo();
    foundation::basic_types_and_vars::signed_integers_demo();
    foundation::basic_types_and_vars::platform_specific_and_string_types_demo();}