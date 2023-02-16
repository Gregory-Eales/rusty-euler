// problem 6
// ------------------------------------------------------
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?
// ------------------------------------------------------

use std::time::{Instant};


fn get_primes(n:usize) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![2];
    let mut i : i32 = 3;
    while primes.len() < n {
        let mut is_prime = true;
        for p in &primes {
            if ((i as f64).sqrt() as i32) < *p {
                break;
            }
            if i % p == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i+=1;
    }
    return primes;
}

fn main() {
    let start = Instant::now();

    // -------------------------------

    let mut primes: Vec<i32> = get_primes(10_001);
    println!("the 10,001st prime is {}", primes[primes.len()-1]);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}