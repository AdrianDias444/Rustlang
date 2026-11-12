use lib::{sum, multiply};

fn main() {
    let a: i32 = 11;
    let b: i32 = 3;

    println!("The sum of {} and {} is {}", a, b , sum(a,b));
    print!("The multiply of {} and {} is {}", a, b, multiply(a, b));
}
