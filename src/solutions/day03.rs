use crate::Solution;
use itertools::Itertools;

const MAX_PRIORITY : usize = 52;

pub fn priority(c: char) -> usize {
    if c >= 'a' {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

pub fn prio_common_letter_two_bag(line: &'static str) -> usize {
    debug_assert_eq!(line.len() % 2, 0);
    let n = line.len() / 2;
    let mut priorities = [false; MAX_PRIORITY + 1];
    for (idx, c) in line.chars().enumerate() {
        let prio = priority(c);

        if idx < n {
            priorities[prio] = true;
        } else {
            if priorities[prio] {
                return prio;
            }
        }
    }
    panic!("no common letter")
}

pub fn prio_common_letter_three_elves(
    line_a: &'static str,
    line_b: &'static str,
    line_c: &'static str) -> usize
{
    let mut priorities = [0u8; MAX_PRIORITY + 1];
    for c in line_a.chars() {
        priorities[priority(c)] = 1;
    }
    for c in line_b.chars() {
        let prio = priority(c);
        if priorities[prio] == 1 {
            priorities[prio] = 2;
        }
    }
    for c in line_c.chars() {
        let prio = priority(c);
        if priorities[prio] == 2 {
            return prio;
        }
    }
    panic!("no common letter")
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/03.input")
    };

    if part == 1 {
        Solution::U64(
            input
                .lines()
                .map(|l| prio_common_letter_two_bag(l) as u64)
                .sum())
    } else {
        Solution::U64(
            input
                .lines()
                //.chunks(3)
                .batching(|i|
                    match i.next() {
                        Some(line) => Some(prio_common_letter_three_elves(
                            line,
                            i.next().unwrap(),
                            i.next().unwrap()) as u64),
                        None => None,
                    })
                .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(prio_common_letter_two_bag(r"vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(solve(1, r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"), Solution::U64(157));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(7763));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"), Solution::U64(18));
        assert_eq!(solve(2, r"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"), Solution::U64(52));
        assert_eq!(solve(2, r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"), Solution::U64(70));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(2569));
    }
}
