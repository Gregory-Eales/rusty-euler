// problem 3
// ------------------------------------------------------
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
// ------------------------------------------------------


use std::time::{Instant};

fn main() {
    let start = Instant::now();

    // -------------------------------
    let mut max = 0;
    for i in 100..1000 {
        for j in i..1000 {
            if is_palindrom(i*j) {
                if i*j > max {
                    max = i*j;
                }
            }
        }
    }
    println!("max palindrome is {}", max);
    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn is_palindrom(n: u32) -> bool {
    // finds if a number is a palindrome
    if n.to_string() == n.to_string().chars().rev().collect::<String>() {
        return true;
    }

    return false;
}