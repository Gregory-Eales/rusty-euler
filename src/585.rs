// problem 5
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
Find F(5000000).

*/
// ------------------------------------------------------


fn main() {
    test_f();
}

fn f(n:u32) -> u32 {

    for x in 0..n {
        for y in 0..n {

            if is_square(y) {
                continue;
            }

            for z in 0..n {

                if is_square(z) {
                    continue;
                }

            }
        }
    }
    
    return n;
}

fn is_square(n: u32) -> bool {
    let sqrt_n = (n as f32).sqrt() as u32;
    sqrt_n * sqrt_n == n
}


fn test_f() {
    assert_eq!(f(100000), 17);
    assert_eq!(f(15), 46);
    assert_eq!(f(20), 86);
    assert_eq!(f(30), 213);
    assert_eq!(f(100), 2918);
    assert_eq!(f(5000), 11134074);
}


/*
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
*/