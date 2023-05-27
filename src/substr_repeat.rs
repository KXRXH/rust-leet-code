use crate::main::init;

init!();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut set = std::collections::HashSet::new();
        let mut left: usize = 0;
        let mut res: i32 = 0;
        for right in 0..s.len() {
            if !set.contains(&s[right]) {
                set.insert(s[right]);
                res = res.max((right - left + 1) as i32);
            } else {
                while s[left] != s[right] {
                    set.remove(&s[left]);
                    left += 1;
                }
                set.remove(&s[left]);
                left += 1;
                set.insert(s[right]);
            }
        }
        res
    }
}