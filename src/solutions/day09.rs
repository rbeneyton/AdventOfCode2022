use crate::Solution;
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/09.input")
    };

    if part == 1 {
        let (mut head_x, mut head_y) = (0i32, 0i32);
        let (mut tail_x, mut tail_y) = (0i32, 0i32);
        let mut tail_pos = FxHashSet::default();

        for line in input.lines() {
            let dir = line.chars().next().unwrap();
            let n = line.split_whitespace().skip(1).next().unwrap();
            let n = n.parse::<usize>().unwrap();

            for _ in 0..n {
                match dir {
                    'R' => head_x += 1,
                    'L' => head_x -= 1,
                    'U' => head_y -= 1,
                    'D' => head_y += 1,
                    _ => panic!(""),
                }
                if (tail_x - head_x).abs() > 1 {
                    tail_x += if tail_x < head_x { 1 } else { -1 };
                    if tail_y != head_y {
                        debug_assert!((tail_y - head_y).abs() <= 1);
                        tail_y += if tail_y < head_y { 1 } else { -1 };
                    }
                } else
                if (tail_y - head_y).abs() > 1 {
                    tail_y += if tail_y < head_y { 1 } else { -1 };
                    if tail_x != head_x {
                        debug_assert!((tail_x - head_x).abs() <= 1);
                        tail_x += if tail_x < head_x { 1 } else { -1 };
                    }
                }

                tail_pos.insert((tail_x, tail_y));
                assert!((tail_x - head_x).abs() + (tail_y - head_y).abs() <= 2);
            }
        }
        Solution::USIZE(tail_pos.len())
    } else {
        const N : usize = 10;
        let (mut node_x, mut node_y) = ([0i32; N], [0i32; N]);
        let mut tail_pos = FxHashSet::default();

        for line in input.lines() {
            let dir = line.chars().next().unwrap();
            let n = line.split_whitespace().skip(1).next().unwrap();
            let n = n.parse::<usize>().unwrap();

            for _ in 0..n {
                match dir {
                    'R' => node_x[0] += 1,
                    'L' => node_x[0] -= 1,
                    'U' => node_y[0] -= 1,
                    'D' => node_y[0] += 1,
                    _ => panic!(""),
                }
                for idx in 1..N {
                    let (head_x, head_y) = (node_x[idx - 1], node_y[idx - 1]);
                    let (tail_x, tail_y) = (&mut node_x[idx], &mut node_y[idx]);

                    if (*tail_x - head_x).abs() > 1 {
                        *tail_x += if *tail_x < head_x { 1 } else { -1 };
                        if *tail_y != head_y {
                            *tail_y += if *tail_y < head_y { 1 } else { -1 };
                        }
                    }
                    if (*tail_y - head_y).abs() > 1 {
                        *tail_y += if *tail_y < head_y { 1 } else { -1 };
                        if *tail_x != head_x {
                            *tail_x += if *tail_x < head_x { 1 } else { -1 };
                        }
                    }

                    assert!((*tail_x - head_x).abs() + (*tail_y - head_y).abs() <= 2);
                }

                tail_pos.insert((node_x[N - 1], node_y[N - 1]));
            }
        }
        Solution::USIZE(tail_pos.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"), Solution::USIZE(13));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::USIZE(5683));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"), Solution::USIZE(1));
        assert_eq!(solve(2, r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"), Solution::USIZE(36));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::USIZE(2372));
    }
}
