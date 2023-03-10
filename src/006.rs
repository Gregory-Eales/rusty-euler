// problem 6
// ------------------------------------------------------
// The sum of the squares of the first ten natural numbers is 1^2 + 2^2 + ... + 10^2 = 385.
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)^2 = 55^2 = 3025.
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
// ------------------------------------------------------


use std::time::{Instant};

fn sum_squares(n:u64) -> u64 {
    let mut sum = 0;
    for i in 0..n+1 {
        sum += i.pow(2);
    }
    return sum;
}

fn square_sum(n:u64) -> u64 {
    let mut sum = 0;
    for i in 0..n+1 {
        sum += i;
    }
    return sum.pow(2);
}

fn main() {
    let start = Instant::now();

    // -------------------------------

    let n = 100;
    println!("answer: {}", square_sum(n) - sum_squares(n));

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}