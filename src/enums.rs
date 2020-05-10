enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // match, similar to switch in c#
    match m {
        Movement::Up => println!("Avatar Moved Up"),
        Movement::Down => println!("Avatar Moved Down"),
        Movement::Left => println!("Avatar Moved Left"),
        Movement::Right => println!("Avatar Moved Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
}
