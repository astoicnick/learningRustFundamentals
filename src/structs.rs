// Used to create custom data types

// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    // Construct a person
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, new_last: &str) {
        self.last_name = new_last.to_string();
    }
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    // let mut c: Color = Color {
    //     red: 255,
    //     green: 150,
    //     blue: 150
    // };
    // c.green = 9;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut cTuple= Color(255, 0, 0);
    // cTuple.2 = 180;
    // println!("Color: {} {} {}", cTuple.0, cTuple.1, cTuple.2);
    let mut p = Person::new("Nick", "Perry");
    p.set_last_name("Berry");
    println!("Person's name is {}", p.full_name());
}
