// problem 3
// ------------------------------------------------------
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600,851,475,143 ?
// ------------------------------------------------------

const N : u64 = 600_851_475_143;

fn main() {
    let mut primes = get_primes();
    let mut lpf = 0;
    for p in primes {
        if N % (p as u64) == 0 {
            lpf = p;
        }
    }
    println!("highest prime factor is {}", lpf);
}

fn get_primes() -> Vec<u32> {
    let mut primes = vec![2];
    let max = (N as f64).sqrt() as u32;
    for i in 3..max {
        let mut is_prime = true;
        for p in primes.clone() {
            if (p as f64).sqrt() > i.into() {
                break;
            }
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    return primes;
}
