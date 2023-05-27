use std::i32::MIN;

use crate::main::init;

init!();

// Binary Exponentiation
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == i32::MIN && x > 1.0 {
            return 0.0;
        }
        let mut res = 1.0;
        let mut b = i32::abs(n);
        let mut x = x;
        while b > 0 {
            if b % 2 == 1 {
                res *= x;
            }
            x *= x;
            b /= 2; // b >>= 1
        }
        if n < 0 {
            res = 1.0 / res
        }
        res
    }
}
