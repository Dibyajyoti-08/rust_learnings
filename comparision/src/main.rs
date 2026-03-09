fn main() {
    let age = 24;
    // let can_party = age >= 21;
    // println!("Can party? {}", can_party);
    let can_party = if age >= 21 {
        "Yes"
    } else {
        "No"
    };
    /*
    Simplified version will be
    let can_party = if age >= 21 { "Yes" } else { "No" };
    */
    println!("Can party? {}", can_party);
}
