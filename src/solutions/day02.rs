use crate::Solution;
#[allow(unused_imports)]
use inpt::{Inpt, inpt};

#[derive(Inpt)]
#[inpt(regex = r"(.) (.)")]
struct Line {
    c1: char,
    c2: char,
}

#[derive(Clone, Copy)]
enum Tile {
    Rock,
    Paper,
    Scissor,
}

impl Line {
    // manual parsing as inpt is too slow here
    pub fn new(i: &'static str) -> Self {
        let mut i = i.chars();
        Line {
            c1: i.next().unwrap(),
            c2: i.skip(1).next().unwrap(),
        }
    }
    pub fn score_part1(self) -> u64 {
        use Tile::{Rock, Paper, Scissor};

        let t1 = match self.c1 {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            _ => panic!(),
        };
        let t2 = match self.c2 {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissor,
            _ => panic!(),
        };
        let round_outcome = match (t1, t2) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 6,
            (Rock, Scissor) => 0,
            (Paper, Rock) => 0,
            (Paper, Paper) => 3,
            (Paper, Scissor) => 6,
            (Scissor, Rock) => 6,
            (Scissor, Paper) => 0,
            (Scissor, Scissor) => 3,
        };
        let bonus_selected = match t2 {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        round_outcome + bonus_selected
    }

    pub fn score_part2(self) -> u64 {
        use Tile::{Rock, Paper, Scissor};

        let t1 = match self.c1 {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            _ => panic!(),
        };
        let round_outcome = match self.c2 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!(),
        };
        let tile_selected = match (t1, round_outcome) {
            (Rock, 0) => Scissor,
            (Rock, 3) => Rock,
            (Rock, 6) => Paper,
            (Paper, 0) => Rock,
            (Paper, 3) => Paper,
            (Paper, 6) => Scissor,
            (Scissor, 0) => Paper,
            (Scissor, 3) => Scissor,
            (Scissor, 6) => Rock,
            _ => panic!(),
        };
        let bonus_selected = match tile_selected {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        round_outcome + bonus_selected
    }
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/02.input")
    };

    if part == 1 {
        Solution::U64(
            input
                .lines()
                // .map(|l| inpt::<Line>(l).unwrap())
                .map(|l| Line::new(l))
                .map(|p| p.score_part1())
                .sum())
    } else {
        Solution::U64(
            input
                .lines()
                // .map(|l| inpt::<Line>(l).unwrap())
                .map(|l| Line::new(l))
                .map(|p| p.score_part2())
                .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = r"A Y
B X
C Z";
        assert_eq!(solve(1, input), Solution::U64(15));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(13268));
    }

    #[test]
    fn part_2_sample() {
        let input = r"A Y
B X
C Z";
        assert_eq!(solve(2, input), Solution::U64(12));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(15508));
    }
}
