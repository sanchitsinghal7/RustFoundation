#![allow(unused_variables)]
#![allow(dead_code)]

mod foundation; // Import the foundation module

fn main() {
    println!("Hello, world!");
    foundation::basic_types_and_vars::boolean_demo();
    foundation::basic_types_and_vars::unsigned_integers_demo();
    foundation::basic_types_and_vars::signed_integers_demo();
    foundation::basic_types_and_vars::platform_specific_and_string_types_demo();
    foundation::collection_types_and_vars::tuple_demo();
    foundation::collection_types_and_vars::array_demo();
    foundation::collection_types_and_vars::slice_demo();
    foundation::collection_types_and_vars::hashmap_demo();
    foundation::object_types_and_vars::struct_ownership_demo();
}