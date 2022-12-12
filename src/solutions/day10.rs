use crate::Solution;

#[derive(Clone, Copy)]
enum Op {
    Addx(i32),
    Noop,
}
impl Op {
    pub fn new(i: &'static str) -> Self {
        let mut tok = i.split_whitespace();
        match tok.next() {
            Some("addx") => Op::Addx(tok.next().unwrap().parse::<i32>().unwrap()),
            Some("noop") => Op::Noop,
            _ => panic!(""),
        }
    }
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/10.input")
    };

    if part == 1 {
        let mut x = 1;
        let mut cycle = 1;
        let mut res = 0;
        for line in input.lines() {
            // record op
            let op = Op::new(line);
            let cycle_incr = match op {
                Op::Addx(..) => 2,
                Op::Noop => 1,
            };
            // advance cycle
            for cycle_cur in cycle..(cycle + cycle_incr) {
                if (cycle_cur as isize - 20) % 40 == 0 {
                    let signal_strength = (cycle_cur as i32) * x;
                    res += signal_strength;
                }
            }
            cycle += cycle_incr;
            // execute current op
            match op {
                Op::Addx(incr) => x += incr,
                Op::Noop => (),
            }
        }
        Solution::I32(res)
    } else {
        let mut x = 1;
        let mut cycle = 1;
        const ROW_SZ : usize = 40;
        const COL_SZ : usize = 6;
        let mut screen = [false; ROW_SZ * COL_SZ];
        for line in input.lines() {
            // record op
            let op = Op::new(line);
            let cycle_incr = match op {
                Op::Addx(..) => 2,
                Op::Noop => 1,
            };
            // advance cycle
            for cycle_cur in cycle..(cycle + cycle_incr) {
                let row_cycle = (cycle_cur - 1) % 40;
                screen[cycle_cur - 1] = (x - row_cycle as i32).abs() <= 1;
            }
            cycle += cycle_incr;
            // execute current op
            match op {
                Op::Addx(incr) => x += incr,
                Op::Noop => (),
            }
        }
        let mut res = String::from("");
        for (idx, on) in screen.iter().enumerate() {
            if idx != 0 && idx % 40 == 0 {
                res.push('\n');
            }
            res.push(if *on { '#' } else { '.' });
        }
        Solution::Str(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"), Solution::I32(13140));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::I32(14620));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"), Solution::Str(String::from("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....")));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::Str(String::from("###....##.####.###..#..#.###..####.#..#.
#..#....#.#....#..#.#..#.#..#.#....#..#.
###.....#.###..#..#.####.#..#.###..#..#.
#..#....#.#....###..#..#.###..#....#..#.
#..#.#..#.#....#.#..#..#.#.#..#....#..#.
###...##..#....#..#.#..#.#..#.#.....##.."))); // aka BJFRHRFU
    }
}
