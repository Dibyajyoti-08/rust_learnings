fn main() {
    let a = String::from("Hello");
    let b = a; // Ownership of the string is moved to b, a is no
               // println!("{}", a); // This will cause an error because a is no longer valid
    println!("{}", b); // This will work because b is the owner of the string

    /*
    string ownership is moved from a to b, but numbers, characters, and booleans are copied not moved. So if we have a variable that holds a number, and we assign it to another variable, both variables will have their own copy of the number, and they will not affect each other.
     */
    let x = 5;
    let y = x; // x is copied to y, both x and y are valid
    println!("x: {}, y: {}", x, y); // This will work because x and y are both valid and have their own copy of the number

    /*
    To make the string ownership work like numbers, we can use the clone method to create a copy of the string. This way, both variables will have their own copy of the string, and they will not affect each other.
     */
    let c = String::from("Hello");
    let d = c.clone(); // c is cloned to d, both c and d are valid and have their own copy of the string
    println!("c: {}, d: {}", c, d); // This will work because c and d are both valid and have their own copy of the string
}
