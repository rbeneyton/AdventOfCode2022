use crate::Solution;
use std::ops::ControlFlow;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/06.input")
    };

    const A : usize = 'a' as usize;
    const Z : usize = 'z' as usize;
    const N : usize = Z - A + 1;
    let depth : usize = if part == 1 { 4 } else { 14 };

    let mut i = input
        .chars()
        .enumerate();
    let _ = i.try_fold((0, [None; N]), |(mut idx_col, mut arr), (idx, c)| {
        debug_assert!(c >= 'a' && c <= 'z');
        let off = c as usize - A;
        if let Some(prev_idx) = arr[off] {
            idx_col = std::cmp::max(idx_col, prev_idx);
        }
        arr[off] = Some(idx);

        if idx >= idx_col + depth {
            ControlFlow::Break((idx_col, arr))
        } else {
            ControlFlow::Continue((idx_col, arr))
        }
    });
    let (idx, _) = i.next().unwrap();

    Solution::U64(idx as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"bvwbjplbgvbhsrlpgdmjqwftvncz"), Solution::U64(5));
        assert_eq!(solve(1, r"nppdvjthqldpwncqszvftbrmjlhg"), Solution::U64(6));
        assert_eq!(solve(1, r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Solution::U64(10));
        assert_eq!(solve(1, r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Solution::U64(11));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(1848));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Solution::U64(19));
        assert_eq!(solve(2, r"bvwbjplbgvbhsrlpgdmjqwftvncz"), Solution::U64(23));
        assert_eq!(solve(2, r"nppdvjthqldpwncqszvftbrmjlhg"), Solution::U64(23));
        assert_eq!(solve(2, r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Solution::U64(29));
        assert_eq!(solve(2, r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Solution::U64(26));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(2308));
    }
}
