fn main() {
    // Defining struct
    struct Person {
        name: String,
        age: u32,
        can_drink: bool,
    }

    // Create a person object
    let mut user = Person {
        name: String::from("dj"),
        age: 25,
        can_drink: true,
    };
    println!(
        "Name: {}, Age: {}, Can Drink: {}",
        user.name, user.age, user.can_drink
    );

    // Change a field value
    user.age = 26;
}
