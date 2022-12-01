use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/17.input")
    };

    if part == 1 {
        Solution::I64(0)
    } else {
        Solution::I64(0)
    }
}
