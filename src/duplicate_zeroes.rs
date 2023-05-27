use crate::main::init;

init!();

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i: usize = 0;
        let len = arr.len();
        while i < arr.len() {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                i += 1;
            }
            i += 1
        }
        arr.resize(len, 0);
    }
}