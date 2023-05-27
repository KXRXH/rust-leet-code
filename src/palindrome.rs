use crate::main;

main::init!();

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        return x
            .to_string()
            .chars()
            .zip(x.to_string().chars().rev())
            .all(|(a, b)| a == b);
    }
}
