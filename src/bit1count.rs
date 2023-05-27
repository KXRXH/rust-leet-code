use crate::main::init;

init!();

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut count: u32 = 0;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        return count as i32;
    }
}
