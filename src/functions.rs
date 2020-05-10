pub fn run() {
    greetUser("Nick");

    // Closure basically an arrow function
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    
}

fn greetUser(name: &str) {
    println!("Hey, {}! Nice to meet you.", name);
}
fn addNums(num1: &i32, num2: &i32) -> i32 {
    return num1 + num2;
}