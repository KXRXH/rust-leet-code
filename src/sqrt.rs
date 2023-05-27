use crate::main::init;

init!();

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut r: i64 = x as i64;
        while r * r > x as i64 {
            r = (r + x as i64 / r) / 2;
        }
        return r as i32;
    }
}
