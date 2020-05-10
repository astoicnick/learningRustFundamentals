pub fn run() {
    // let means the variable can't be immutable
    let name = "Brad";
    // mut makes age mutable
    let mut age = 18;
    println!("My name is {0} and I am {1} years old", name, age);
    age = 19;
    println!("My age is now {}", age);

    // constant
    const ID: i32 = 001;
    println!("ID: {0}", ID);

    // assign multiple variables at once
    let ( my_name, my_age ) = ("Nick", 19);
    println!("{} {}", my_name, my_age);
}