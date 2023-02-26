// problem 51
// ------------------------------------------------------
/*
By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.
*/

use::std::time::Instant;
use::std::collections::HashMap;


fn main() {

    let start = Instant::now();

    // -------------------------------

    let mut primes = get_primes(1_000_000);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time {:?}", duration);
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