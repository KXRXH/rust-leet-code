use crate::main::init;

init!();


impl Solution {
    pub fn is_valid(s: String) -> bool {
        let l = s.len() / 2;
        let mut stack: Vec<char> = Vec::with_capacity(l);
        let s = s.chars();
        for ch in s {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                _ => {
                    match stack.pop() {
                        None => return false,
                        Some(c) => {
                            match c {
                                '(' => if ch != ')' { return false; },
                                '{' => if ch != '}' { return false; },
                                '[' => if ch != ']' { return false; },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        return stack.len() == 0;
    }
}