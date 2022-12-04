use crate::Solution;
use itertools::Itertools;

pub fn zone_fully_contains(line: &'static str) -> bool {
    let (x1, y1, x2, y2) = line
        .split(|c| c == '-' || c == ',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (x1 <= x2 && y1 >= y2) ||
    (x2 <= x1 && y2 >= y1)
}

pub fn zone_overlap(line: &'static str) -> bool {
    let (x1, y1, x2, y2) = line
        .split(|c| c == '-' || c == ',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (x1 <= x2 && y1 >= x2) ||
    (x2 <= x1 && y2 >= x1)
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/04.input")
    };

    if part == 1 {
        Solution::U64(
            input
                .lines()
                .map(|l| zone_fully_contains(l) as u64)
                .sum())
    } else {
        Solution::U64(
            input
                .lines()
                .map(|l| zone_overlap(l) as u64)
                .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(zone_fully_contains(r"2-4,6-8"), false);
        assert_eq!(zone_fully_contains(r"2-8,3-7"), true);
        assert_eq!(zone_fully_contains(r"6-6,4-6"), true);
        assert_eq!(solve(1, r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"), Solution::U64(2));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(496));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(zone_overlap(r"2-4,6-8"), false);
        assert_eq!(zone_overlap(r"5-7,7-9"), true);
        assert_eq!(zone_overlap(r"2-8,3-7"), true);
        assert_eq!(zone_overlap(r"6-6,4-6"), true);
        assert_eq!(zone_overlap(r"2-6,4-8"), true);
        assert_eq!(solve(2, r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"), Solution::U64(4));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(847));
    }
}
