pub fn boolean_demo() {
    let is_rust_fun: bool = true;  // Used in conditions and logical operations
    if is_rust_fun {
        println!("Rust is awesome!");
    }
}

pub fn unsigned_integers_demo() {
    let small_number: u8 = 255; // Max value for u8
    let medium_number: u16 = 65_535; // Max value for u16
    let large_number: u32 = 4_294_967_295; // Max value for u32
    let very_large_number: u64 = 18_446_744_073_709_551_615; // Max value for u64
    let huge_number: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // Max value for u128

    println!(
        "u8: {}\nu16: {}\nu32: {}\nu64: {}\nu128: {}",
        small_number, medium_number, large_number, very_large_number, huge_number
    );
}

pub fn signed_integers_demo() {

    // Signed Integers (can be negative)
    let small_signed: i8 = -128; // Min value for i8
    let medium_signed: i16 = -32_768; // Min value for i16
    let large_signed: i32 = -2_147_483_648; // Min value for i32
    let very_large_signed: i64 = -9_223_372_036_854_775_808; // Min value for i64
    let huge_signed: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // Min value for i128

    println!("\nSigned Integers:");
    println!(
        "i8: {}\ni16: {}\ni32: {}\ni64: {}\ni128: {}",
        small_signed, medium_signed, large_signed, very_large_signed, huge_signed
    );
}

pub fn platform_specific_and_string_types_demo() {
    // Platform-Specific Integers
    let unsigned_size: usize = 9_223_372_036_854_775_807; // Max for 64-bit platform
    let signed_size: isize = -9_223_372_036_854_775_808; // Min for 64-bit platform

    // Character Type (Holds a single Unicode character)
    let character: char = '🦀'; // Rust's mascot emoji (Unicode support)

    // String Types
    let string_slice: &str = "Hello, Rust!"; // String slice (borrowed, immutable)
    let owned_string: String = String::from("I am an owned String!"); // Owned heap-allocated string

    println!("Platform-Specific Integers:");
    println!("usize: {}", unsigned_size);
    println!("isize: {}", signed_size);

    println!("\nCharacter:");
    println!("char: {}", character);

    println!("\nString Types:");
    println!("&str: {}", string_slice);
    println!("String: {}", owned_string);
}



// 4. Main Function
fn main() {
    boolean_demo();
    unsigned_integers_demo();
    signed_integers_demo();
    platform_specific_and_string_types_demo();
}  