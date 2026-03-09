fn main() {
    let mut n = [1, 2, 3, 4, 5]; // This is an array of integers with a fixed size of 5
    n[0] = 10; // This will work because we declared the array as mutable
    println!("The first element of the array is: {}", n[0]); // This will print the first element of the array, which is 10
    println!("The length of the array is: {}", n.len()); // This will print the length of the array, which is 5
}
