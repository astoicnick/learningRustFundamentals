// Reference pointers - Point to a resource in memory
// Left off at 1:21:41

pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // FOR NON-PRIMITIVES : If you assign another variable to a piece of data,
    // the first variable won't have the value anymore. You'll need to reference it with &
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2) );
}