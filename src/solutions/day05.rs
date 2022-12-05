use itertools::Itertools;
use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/05.input")
    };

    // find number of stacks: index line start with a space
    let n = input
        .lines()
        .skip_while(|l| l.chars().filter(|c| *c == '[').count() > 0)
        .next().unwrap()
        .split_whitespace()
        .map(|tok| tok.parse::<usize>().unwrap())
        .map(|id| { assert!(id != 0); id })
        .max()
        .unwrap();

    // fill them
    let mut stacks = Vec::new();
    stacks.resize(n, Vec::new());
    let mut lines = input.lines();
    for line in &mut lines {
        if line.chars().next().unwrap() == ' '
        && line.chars().skip(1).next().unwrap().is_digit(10)
        {
            break;
        }
        for (idx, c) in line.chars().enumerate() {
            if idx > 0 && (idx - 1) % 4 == 0 && c != ' ' {
                let idx = (idx - 1) / 4;
                debug_assert!(idx < n);
                stacks[idx].push(c);
            }
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    // parse & apply the operations
    for line in lines {
        if let Some((qty, from, to)) = line
            .split_whitespace()
            .filter(|s| !["move", "from", "to"].contains(s))
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
        {
            let from = from - 1;
            let to = to - 1;
            debug_assert!(stacks[from].len() >= qty);
            let pop = stacks[from].len() - qty;
            let mut rem = stacks[from].split_off(pop);
            if part == 1 {
                rem.reverse();
            }
            stacks[to].extend_from_slice(&rem[..]);
        }
    }

    // pops
    let mut res = String::new();
    for stack in &mut stacks {
        res.push(stack.pop().unwrap());
    }

    Solution::Str(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"), Solution::Str(String::from("CMZ")));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::Str(String::from("VGBBJCRMN")));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"), Solution::Str(String::from("MCD")));
    }

    // #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(0));
    }
}
