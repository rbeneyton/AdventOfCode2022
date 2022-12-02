use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/05.input")
    };

    if part == 1 {
        Solution::U64(0)
    } else {
        Solution::U64(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    #[allow(unused)]
    fn part_1_sample() {
        assert_eq!(solve(1, r"..."), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_2_sample() {
        assert_eq!(solve(2, r"..."), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(0));
    }
}
