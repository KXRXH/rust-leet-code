use crate::main::init;

init!();

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut carry = 1;
        for i in digits.iter().rev() {
            let digit = (*i + carry) % 10;
            carry = (*i + carry) / 10;
            result.insert(0, digit);
        }
        if carry > 0 {
            result.insert(0, carry);
        }
        result
    }
}
