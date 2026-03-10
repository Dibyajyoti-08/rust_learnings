enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {
    let my_direction = Direction::Up;
    println!("We are going up!");

    // Match on enum values
    match my_direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }

    // Enums with data
    let result1 = LoginStatus::Success(String::from("Welcome onboard!"));
    let result2 = LoginStatus::Error(String::from("Invalide credentials"));

    match result1 {
        LoginStatus::Success(message) => println!("Login successful: {}", message),
        LoginStatus::Error(error) => println!("Login failed: {}", error),
    }
}
