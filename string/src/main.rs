fn main() {
    let text1 = "rust learning".to_string();
    println!("The text is: {}", text1);
    let text2 = String::from("rust learning");
    println!("The text is: {}", text2);

    // Change in string
    let mut text3 = String::from("Greeting");
    text3.push_str(" from Rust");
    println!("The text is: {}", text3);
}
