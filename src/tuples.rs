// Tuples group together elements of different types, max of 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Nick", "New York", 19);
    println!("{} is from {} and is {} years old", person.0, person.1, person.2);
}