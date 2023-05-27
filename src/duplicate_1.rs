use crate::main::init;

init!();

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = std::collections::HashSet::with_capacity(nums.len());
        for num in nums {
            if map.contains(&num) {
                return true;
            }
            map.insert(num);
        }
        return false;
    }
}
