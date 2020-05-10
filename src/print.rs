pub fn run() {
    println!("Hello from print.rs!");
    // Base formatting
    println!("Num1: {}, Num 2: {}", 1,2);
    // Positional 
    println!("{0} like {1} and I like {2} but remember, I like {1}.", "Nick", "Fencing", "Coding");
    // Named
    println!("{name} likes to play {activity}",
    name = "John",
    activity = "Baseball");
    // Placeholder
    println!("Binary: {:b} | Hex: {:x} | Octal: {:o} ", 10, 10, 10);
    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    // math
    println!("10 + 10 = {}", 10 + 10);
}