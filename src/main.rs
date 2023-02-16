// problem 585
// ------------------------------------------------------
/*

Consider the term √(x + √y + √z) that is representing a nested square root. x, y, and z are positive integers and y and z are not allowed to be perfect squares, so the number below the outer square root is irrational. Still, it can be shown that for some combinations of x, y, and z, the given term can be simplified into a sum and/or difference of simple square roots of integers, actually denesting the square roots in the initial expression.

Here are some examples of this denesting:
√(3 + √2 + √2) = √2 + √1 = √2 + 1
√(8 + √15 + √15) = √5 + √3
√(20 + √96 + √12) = √9 + √6 + √3 - √2 = 3 + √6 + √3 - √2
√(28 + √160 + √108) = √15 + √6 + √5 - √2

As you can see, the integers used in the denested expression may also be perfect squares resulting in further simplification.

Let F(n) be the number of different terms √(x + √y + √z), that can be denested into the sum and/or difference of a finite number of square roots, given the additional condition that 0 < x ≤ n. That is,
√(x + √y + √z) = ∑(i=1 to k) si * √ai
with k, x, y, z and all ai being positive integers, all si = ±1 and x ≤ n. Furthermore, y and z are not allowed to be perfect squares.

Nested roots with the same value are not considered different, for example √(7 + √3 + √27), √(7 + √12 + √12), and √(7 + √27 + √3), that can all three be denested into 2 + √3, would only be counted once.

You are given that F(10) = 17, F(15) = 46, F(20) = 86, F(30) = 213, F(100) = 2918 and F(5000) = 11134074.
Find F(5,000,000).

*/
// ------------------------------------------------------

use indicatif::ProgressIterator;


fn main() {
    println!("counted {} nested terms!", f2(10000));
}

fn f(n:u32) -> u32 {

    let mut sum_a = 0.0;

    for x in (0..n).progress() {
   
        for y in 2..x*4 {

            if is_square(y) {
                continue;
            }

            for z in 2..x*4 {

                if is_square(z) {
                    continue;
                }

                // calulate the sum of root values
                sum_a = ((x as f64) + (y as f64).sqrt() + (z as f64).sqrt()).sqrt();

                for a in 0..x {
                    for b in 0..a {
                        for c in 0..a {
                            for d in 0..a {
                                if sum_a == (a as f64).sqrt() + (b as f64).sqrt() + (c as f64).sqrt() + (d as f64).sqrt() {
                                    println!("sqrt({} + sqrt({}) + sqrt({})) = sqrt({}) +  sqrt({}) + sqrt({}) + sqrt({})", x, y, z, a, b, c, d);
                                } 
                            }
                        }                   
                    }
                }
            }
        }
    }
    
    return sum_a as u32;
}

fn f2(n:u32) -> u32 {

    let mut count = 0;

    for x in (0..n).progress() {

        for b in 0..(x as f32).sqrt() as u32 {

            for c in 0..((x as f32).sqrt() as u32) {

                let mut a = (((c as u32)*(b.pow(2) as u32)) as f32).sqrt() as u32;

                if (a as f32).sqrt().fract() != 0.0 {
                    continue;
                }
                count += 1;
            }
        }
    }
    return count;
}


fn is_square(n: u32) -> bool {
    let sqrt_n = (n as f32).sqrt() as u32;
    sqrt_n * sqrt_n == n
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_f() {
        assert_eq!(f(10), 17);
        assert_eq!(f(15), 46);
        assert_eq!(f(20), 86);
        //assert_eq!(f(30), 213);
        //assert_eq!(f(100), 2918);
        //assert_eq!(f(5000), 11134074);
    }
}


