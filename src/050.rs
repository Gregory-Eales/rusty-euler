// problem 50
// ------------------------------------------------------
/*
The prime 41, can be written as the sum of six consecutive primes:

41 = 2 + 3 + 5 + 7 + 11 + 13
This is the longest sum of consecutive primes that adds to a prime below one-hundred.

The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

Which prime, below one-million, can be written as the sum of the most consecutive primes?
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
