/*
    Primitive Types:
    - Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
    - Floating Point: f32, f64
    - Boolean: bool
    - Character: char
    - Tuples
    - Arrays
    - Slices
    - String
    - References
    - Functions
*/

// Rust is a statically typed language, and must know the types at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 455567;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;
    println!("Boolean: {}", is_active);

    //Get boolean from expression
    let _is_greater = 10 > 0;

    //Char

    let _a1 = "c";
    let face = "\u{1F600}";

    //printing all variables

    println!("{:?}", (x, y, z, is_active,face));
}
