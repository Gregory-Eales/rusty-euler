// problem 14
// ------------------------------------------------------
/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

use std::time::Instant;
use std::collections::HashMap;
use indicatif::ProgressIterator;

fn main() {
    let start = Instant::now();

    // -------------------------------

    let mut table : HashMap<u64, u64> = HashMap::new();

    let mut greatest_length = 0;
    let mut greatest_value = 0;

    let mut length;

    for i in (1..1_000_000).progress() {
        length = collatz_length(i as u64, &table);
        table.insert(i as u64, length);
        if length > greatest_length {
            greatest_length = length;
            greatest_value = i as u64;
        }
    }

    println!("collatz length of {} is {}", greatest_value, greatest_length);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn collatz_length(mut n: u64, table: &HashMap<u64, u64>) -> u64{

    let mut count = 1;
    while n != 1 {

        if table.contains_key(&n) {
            return &table[&n] + count;
        }

        if n % 2 == 0 {
            n = n / 2;
        }

        else {
            n = 3*n + 1;
        }
        count += 1;
    }

    return count;
}


