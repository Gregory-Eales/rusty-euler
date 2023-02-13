// problem 1

fn main() {

    let mut n = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            n += i;
        }
    }

    println!("sum of all numbers divisable by 3 or 5 less than 1,000 is {}", n);
}