// problem 10
// ------------------------------------------------------
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.
// ------------------------------------------------------

use std::time::{Instant};


fn main() {
    let start = Instant::now();

    // -------------------------------

    let primes: Vec<u64> = get_primes(2_000_000);

    println!("sum of all primes under 4,000,000, is {}", primes.iter().sum::<u64>());

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}


fn get_primes(n:i32) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![2];
    for i in 3..n {
        let mut is_prime = true;
        for p in &primes {
            if ((i as f64).sqrt() as u64) < *p {
                break;
            }
            if (i as u64) % p == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            primes.push(i as u64);
        }
    }
    return primes;
}