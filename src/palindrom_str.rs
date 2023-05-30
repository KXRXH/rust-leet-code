use crate::main::init;

init!();

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_ascii_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<Vec<char>>();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}