

// problem 21
// ------------------------------------------------------
/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

use::std::time::Instant;
use::std::collections::HashMap;


fn main() {

    let start = Instant::now();

    // -------------------------------

    // get all the primes needed for factorization
    let mut hashmap : HashMap<u32, u32> = HashMap::new();
    let mut sum;
    for i in 1..10_000-1 {
        sum = 0;
        for j in 1..i {
            if i % j == 0 {
                sum += j
            }
        }
        hashmap.insert(i, sum);
    }

    sum = 0;

    let keys : Vec<u32> = hashmap.keys().cloned().collect();

    for k in keys {

        if !hashmap.contains_key(&k) {
            continue;
        }

        let value = hashmap[&k].clone();

        if !hashmap.contains_key(&value) {
            continue;
        }

        if hashmap[&value] == k {
            if &value == &k {
                continue;
            }
            
            sum += &value + &k;
            hashmap.remove(&value);
            hashmap.remove(&k);

            println!("found amicable pair {} and {}", &value, &k);
            
        }
    }

    println!("sum of amicable numbers under 10,000 is {}", sum);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time {:?}", duration);
}
