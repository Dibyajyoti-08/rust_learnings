fn main() {
    let mut count = 1;

    // loop
    loop {
        println!("I am still looping! {} time", count);

        if count == 3
        {
            break;
        }
        count += 1;
    }
    // while
    while count <= 5
    {
        println!("Count is: {}", count);
        count += 1;
    }
    // break while
    while count <= 10
    {
        if count == 4
        {
            break;
        }   
        println!("Count is: {}", count);
        count += 1;
    }
    // continue while
    while count <= 10
    {
        if count == 6
        {
            count += 1;
            continue;
        }
        println!("Count is: {}", count);
        count += 1;
    }
    // for
    for i in 1..6
    {
        println!("i is: {}", i);
    }
    // for inclusive
    for i in 1..=6
    {
        println!("i is: {}", i);
    }

}
