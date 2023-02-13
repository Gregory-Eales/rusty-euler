// problem 1
// ------------------------------------------------------
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
// ------------------------------------------------------

fn main() {

    let mut n = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            n += i;
        }
    }

    println!("sum of all numbers divisable by 3 or 5 less than 1,000 is {}", n);
}