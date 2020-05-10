/*
Primitives:
- Integers
    - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    - (How many bits they take up)
- Floats
    - f32, f64 (How many bits they take up)
- Boolean
    - bool
- Characters
    - char
- Tuples
- Arrays
*/

pub fn run() {
    // Default int is i32
    let x = 1;

    // defautl float is f64
    let y = 2.5;

    // Explicit type
    let z: i64 = 30358505;

    // Max size of a variable
    println!("Max i32: {}", std::i32::MAX );
    println!("Max i64: {}", std::i64::MAX );

    // Boolean
    let is_active: bool = true;
    let is_greater: bool = 10 > 5;

    // Character
    let char1 = 'a';
    let faceEmoji = '\u{1F600}';

    println!("{:?}", (x, y, z, faceEmoji ));
}