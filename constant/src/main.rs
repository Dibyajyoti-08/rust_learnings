fn main() {
    const X: i32 = 5;
    println!("Before: x = {}", X);
    //X = 10; // This will cause an error because constants are immutable
    println!("After: x = {}", X);
}
