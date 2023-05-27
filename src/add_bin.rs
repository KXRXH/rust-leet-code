use crate::main::init;

init!();

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a.len() < b.len() {
            return Solution::add_binary(b, a);
        }
        let mut chars_a: Vec<u8> = a
            .chars()
            .rev()
            .map(|a| a.to_digit(10).unwrap() as u8)
            .collect();
        let mut chars_b: Vec<u8> = b
            .chars()
            .rev()
            .map(|a| a.to_digit(10).unwrap() as u8)
            .collect();
        let mut result = String::new();
        let mut carry = 0;
        for i in 0..a.len() {
            let a_val = *chars_a.get(i).unwrap();
            let mut b_val = 0;
            if i < b.len() {
                b_val = *chars_b.get(i).unwrap();
            }
            let sum = a_val + b_val + carry;
            carry = sum / 2;
            result.push((sum % 2).to_string().as_str().parse().unwrap());
        }
        if carry > 0 {
            result.push('1');
        }
        return result.chars().rev().collect();
    }
}
