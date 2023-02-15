// problem 5
// ------------------------------------------------------
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
// ------------------------------------------------------


use std::time::{Instant};
use std::collections::HashMap;

fn main() {
    let start = Instant::now();

    // -------------------------------

    // initialize data structs
    let mut primes: Vec<u32>  = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let mut map: HashMap<u32, u32> = HashMap::new();

    // populate the hasmap with primes and count 0
    for p in &primes {
        map.insert(*p, 0);
    }

    // loop through each number and then check
    // the max times a prime can divide through
    for i in 2..21 {
        for p in &primes {
            let mut count = 0;
            let mut n = i;

            while n % p == 0 {
                count += 1;
                n = n / p;
            }

            if count > 0 && map[p] < count {
                map.insert(*p, count);
            }
        }
    }
    
    // multiply all primes by their
    // max number of divisions
    let mut n = 1;
    for k in map.keys() {
        n *= k.pow(map[k]); 
    }

    println!("answer is is {}", n);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

