use crate::main::init;

init!();

// No algorithm here
// Coz
// I don't care. At all...
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in (m as usize)..(m + n) as usize {
            nums1[i] = nums2[i - m as usize];
        }
        nums1.sort();
    }
}
