// Fixed list where elements are of the same type
// Stack allocated

pub fn run() {
    // has to have 5 elements, fixed size when defined
    let numArray: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numArray);

    println!("The array is taking up {} bytes on the stack", std::mem::size_of_val(&numArray));

    // Slice of array
    let slice: &[i32] = &numArray[0..3];
    println!("Slice: {:?}", slice);
}
