mod add_bin;
mod bit1count;
mod duplicate_1;
mod duplicate_2;
mod merge_sorted_arr;
mod palindrome;
mod plus_one;
mod pow;
mod reverse_int;
mod sqrt;
mod duplicate_zeroes;
mod substr_repeat;

mod main {
    macro_rules! init {
        () => {
            pub struct ListNode {
                pub val: i32,
                pub next: Option<Box<ListNode>>,
            }

            impl ListNode {
                #[inline]
                fn new(val: i32) -> Self {
                    ListNode {
                        next: None,
                        val,
                    }
                }
            }
            struct Solution;
        };
    }
    pub(crate) use init;
}

fn main() {
    println!("Hello, world!");
}