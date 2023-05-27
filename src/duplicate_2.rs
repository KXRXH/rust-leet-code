use crate::main::init;

init!();

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        return nums
            .windows(k as usize)
            .any(|w| w.iter().all(|x| nums.contains(x)));
    }
}
