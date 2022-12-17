use crate::Solution;
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/12.input")
    };

    let w = input.lines().next().unwrap().chars().count();
    let h = input.lines().count();

    let mut grid = Vec::new();
    grid.resize(w * h, 0i8);
    let idx_of = |x, y| x + w * y;
    let iidx_of = |x : isize, y : isize| (x + (w as isize) * y) as usize;
    let idx_of_p = |p : (isize, isize)| (p.0 as usize) + w * (p.1 as usize);

    // {{{ parsing

    let mut i = input.chars();
    let offset = if part == 1 { 1 } else { 0 };
    let mut start = (0, 0);
    let mut stop = (0, 0);
    for row in 0..h {
        for col in 0..w {
            let c = i.next().unwrap();
            grid[idx_of(col, row)] = match c {
                'a' ..= 'z' => (c as usize - 'a' as usize) as i8,
                'S' => {
                    start = (col as isize, row as isize);
                    0
                },
                'E' => {
                    stop = (col as isize, row as isize);
                    ('z' as usize - 'a' as usize) as i8
                },
                _ => panic!(""),
            };
        }
        assert_eq!(i.next(), Some('\n'));
    }
    debug_assert_eq!(i.next(), None);

    // }}}
    // {{{ reverse fill distances

    let mut distances = Vec::new();
    distances.resize(w * h, -1i16);
    distances[idx_of_p(stop)] = 0;

    let mut froms = FxHashSet::default();
    froms.insert(stop);
    let mut tos = FxHashSet::default();
    loop {
        for from in &froms {
            let (col, row) = from;
            let idx = iidx_of(*col, *row);
            let cur = grid[idx];
            let cur_dist = distances[idx];
            for (dcol, drow) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (ncol, nrow) = (col + dcol, row + drow);
                if ncol < 0 || ncol >= w as isize || nrow < 0 || nrow >= h as isize { continue; }
                let nidx = iidx_of(ncol, nrow);
                if grid[nidx] < cur - 1 { continue; }
                if distances[nidx] < 0 || distances[nidx] > cur_dist + 1 {
                    distances[nidx] = cur_dist + 1;
                    tos.insert((ncol, nrow));
                }
            }
        }
        froms.clear();
        if tos.len() == 0 {
            break;
        }
        std::mem::swap(&mut froms, &mut tos);
    }

    // }}}

    if part == 1 {
        Solution::I32(distances[idx_of_p(start)] as i32)
    } else {
        Solution::I32(grid
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(idx, _)| distances[idx])
            .filter(|x| *x >= 0)
            .min()
            .unwrap() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"), Solution::I32(31));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::I32(423));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"), Solution::I32(29));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::I32(416));
    }
}
