// Primitive str is immutable, fixed-length string on the stack
// String = heap-allocated structure, use when I want to modify or keep string data

pub fn run() {
    // This turns into str, the primitive
    // let mut hello = "Hello";
    // This is the string data structure
    let mut hello = String::from("Hello");
    // Print length of the string
    println!("Length: {}", hello.len());
    // adds char to hello
    hello.push('W');
    // appends the string to original
    hello.push_str("orld!"); 

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
}