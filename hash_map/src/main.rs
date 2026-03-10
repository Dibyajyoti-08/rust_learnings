// Import HashMap from the standard library
use std::collections::HashMap;

fn main() {
    // Create a HashMap
    let mut capital_cities = HashMap::new();

    capital_cities.insert("Odisha", "Bhubaneswar");
    capital_cities.insert("Karnataka", "Bangalore");
    capital_cities.insert("Tamil Nadu", "Chennai");
    println!("Capital Cities: {:?}", capital_cities);

    // Access values
    if let Some(city) = capital_cities.get("Odisha") {
        println!("The capital of Odisha is {}", city);
    } else {
        println!("Odisha is not in the map");
    }

    // Update values
    capital_cities.insert("Odisha", "Puri");
    println!("Updated Capital Cities: {:?}", capital_cities);

    // Remove values
    capital_cities.remove("Tamil Nadu");
    println!("Capital Cities after removal: {:?}", capital_cities);

    // Looping through the HashMap
    for (state, city) in &capital_cities {
        println!("The capital of {} is {}", state, city);
    }
}
