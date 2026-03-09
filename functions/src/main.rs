fn add(a: i32, b:i32)
{
    println!("The sum of {} and {} is {}", a, b, a+b);
}

fn mul(a: i32, b: i32) -> i32
{
    return a * b;
}

fn main() 
{
    add(5, 10);
    let product = mul(5, 5);
    println!("The product of 5 and 5 is {}", product);
}
