fn main() {
    // Vectors are resizable arrays, and are provided by the standard library as the Vec<T> type.
    let mut demog = vec!["age", "gender", "relegion"];
    demog[0] = "height";
    println!("First element: {}", demog[0]);
    demog.push("education");
    println!("After update: {:?}", demog);
    // Remove vectors
    demog.pop();
    println!("After pop: {:?}", demog);
    // And or remove of a specific index
    demog.insert(1, "income");
    println!("After insert: {:?}", demog);
    demog.remove(1);
    println!("After remove: {:?}", demog);
    // loop through vectors
    for items in &demog {
        println!("Item: {}", items);
    }
}
