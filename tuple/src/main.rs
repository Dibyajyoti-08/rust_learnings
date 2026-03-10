fn main() {
    // Access tuple
    let person = ("dj", 25, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    // Unpack tuple
    let (naam, umar, zinda_he) = person;
    println!("Name: {}", naam);
    println!("Age: {}", umar);
    println!("Is active: {}", zinda_he);
}
