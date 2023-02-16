// problem 9
// ------------------------------------------------------
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
// ------------------------------------------------------


fn main() {
    for b in 1..1000 {
        let a = 500 * (2 * b - 1000) / (b - 1000);
        if a < 1 {
            continue;
        }
        let c = ((a as f64).powf(2.0) + (b as f64).powf(2.0)) as f64;
        if c.sqrt().fract() != 0.0 || a as i32 != a {
            continue;
        }
        if a + b + c.sqrt() as i32 == 1000 {
            println!("product of a,b,c is {}", a * b * c.sqrt() as i32);
            break;
        }
    }
}