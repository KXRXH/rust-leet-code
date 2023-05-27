use std::collections::HashMap;

use crate::main::init;

init!();

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if map.contains_key(&num) {
                if i - map[&num] <= k as usize {
                    return true;
                }
            }
            map.insert(num, i);
        } 
        false
    }
}