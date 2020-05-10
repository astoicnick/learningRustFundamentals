// Resizable Arrays


pub fn run() {
    // has to have 5 elements, fixed size when defined
    let mut numVector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numVector);

    println!("The vector is taking up {} bytes on the stack", std::mem::size_of_val(&numVector));
    numVector.push(6);
    numVector.push(19);
    println!("The vector is taking up {} bytes after adding those numbers", std::mem::size_of_val(&numVector));

    // Slice of array
    let slice: &[i32] = &numVector[0..3];
    println!("Slice: {:?}", slice);

    // For loop, no mutation
    for num in numVector.iter() {
        println!("Number: {}", num);
    }
    // For loop, with mutation
    for num in numVector.iter_mut() {
        println!("Number Original: {}", num);
        *num += 1;
        println!("Number New: {}", num);
    }

}
