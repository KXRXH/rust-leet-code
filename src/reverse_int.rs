use crate::main::init;

init!();

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0;
        while x != 0 {
            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }
            result = result * 10 + x % 10;
            x /= 10;
        }
        if x < 0 {
            return -result;
        }
        result
    }
}
