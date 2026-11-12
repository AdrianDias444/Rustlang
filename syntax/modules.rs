mod functions;

use functions::{sum, multiply};


fn main()
{
    let a: i32 = 3;
    let b: i32 = 7;

    println!("The sum of {} and {} is {}", a, b, sum(a, b));
    println!("The multiply of {} and {} is {}", a, b, multiply(a, b));
}
