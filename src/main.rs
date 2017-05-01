mod problems;
mod problems_2;

use std::io;
use problems::problem1;

fn main() {
    println!("Enter n and array");

    let mut n: String = String::new();
    let mut height_string: String = String::new();

    // read in the inputs from the stdin :: console
    io::stdin().read_line(&mut n).expect("Err");
    io::stdin().read_line(&mut height_string).expect("Err");

    let n: i32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    problem1::bday_candles(n, &height_string);
    problem1::bday_candles_hashmap(n, &height_string);

    let v: Vec<i32> = vec![1, 2, 3, 4];

    println!("sum = {}", problems_2::array_sum(&v));
}
