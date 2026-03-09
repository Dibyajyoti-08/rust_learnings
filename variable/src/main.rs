fn main() {
    /*
    rust varibales are immutable by default, it can be made mutable by using the mut keyword.
    */
    let mut x = 5;
    println!("Before: x = {}", x);
    x = 10;
    println!("After: x = {}", x);
}
