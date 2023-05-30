use std::collections::HashSet;

use crate::main::init;

init!();

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut h = (-1, -1);
        s.chars().for_each(|c| match c.to_digit(10) {
            None => {}
            Some(v) => {
                let v = v as i32;
                if v > h.1 {
                    h.0 = h.1;
                    h.1 = v;
                } else if v > h.0 && v != h.1 {
                    h.0 = v;
                }
            }
        });
        h.0
    }
}