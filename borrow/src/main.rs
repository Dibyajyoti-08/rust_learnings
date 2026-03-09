fn main() {
    let a = String::from("Rust");
    let b = &a; // b is a reference to a, it does not take ownership of the string
    println!("a: {}, b: {}", a, b); // This will work because a is still the owner of the string and b is just a reference to it

    // Mutable referencing
    let mut c = String::from("Programming");
    let d = &mut c; // d is a mutable reference to c, it allows us to modify the string through d
    d.push_str(" in Rust"); // This will modify the string that c owns through the mutable reference d
    println!("d: {}", d); // This will work because c is still the owner of the string and d is just a mutable reference to it
}
