// problem 39
// ------------------------------------------------------
/*
If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?
*/

use::std::time::Instant;
use::std::collections::HashMap;


fn main() {

    let start = Instant::now();
    let mut map : HashMap<u32, u32> = HashMap::new();

    // -------------------------------

    let mut sum: u32;
    for a in 1..1000 {
        for b in 1..1000 {

            let c_squared: f32 = ((a as f32).powf(2.0) + (b as f32).powf(2.0)).sqrt();
            if c_squared.fract() != 0.0 {
                continue;
            }
            let c: u32 = c_squared as u32;
            println!("not skipping: {}", c);

            sum = a + b + c;
            if sum > 1000 {
                continue;
            }
            println!("{} {} {} - {}", a, b, c, sum);
            if !map.contains_key(&sum) {
                map.insert(sum, 0);
            }
            map.insert(sum, &map[&sum]+1);
        }
    }

    let mut max_key = 0;
    let mut max_val = 0;

    for key in map.keys() {
        if map[key] > max_val {
            max_key = *key;
            max_val = map[key];
        }
    }

    println!("perimter is {} | counted {} times", max_key, max_val);

    // -------------------------------

    let duration = start.elapsed();
    println!("Execution time {:?}", duration);
}
