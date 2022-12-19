use crate::Solution;
use itertools::Itertools;
use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Point {
    Air,
    Rock,
    Sand,
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/14.input")
    };

    // {{{ get geometry

    let (sand_x, sand_y) = (500, 0);
    let mut max_x = sand_x;
    let mut min_x = sand_x;
    let mut max_depth = sand_y;

    for line in input.lines() {
        for (x, y) in line
            .split(" -> ")
            .map(|x| x.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap())
        {
            max_x = cmp::max(max_x, x);
            min_x = cmp::min(min_x, x);
            max_depth = cmp::max(max_depth, y);
        }
    }

    let origin_y = sand_y;
    let h = max_depth + 3 - sand_y;

    let (origin_x, w) = if part == 1 {
        (min_x - 1, max_x + 2 - min_x + 1)
    } else {
        // extra width for part 2 to allow a full triangle in worst case
        (min_x - 1 - h, max_x + 2 - min_x + 1 + 2 * h)
    };

    // }}}
    // {{{ fill grid

    let mut grid = Vec::new();

    grid.resize(w * h, Point::Air);
    // XXX we rotate grid to get faster scan when going down
    let idx_of = |x, y| y + h * (x - origin_x);

    for line in input.lines() {
        for ((from_x, from_y), (to_x, to_y)) in line
            .split(" -> ")
            .map(|x| x.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap())
            .tuple_windows()
        {
            let (x1, x2) = (cmp::min(from_x, to_x), cmp::max(from_x, to_x));
            let (y1, y2) = (cmp::min(from_y, to_y), cmp::max(from_y, to_y));
            for x in x1..=x2 {
                for y in y1..=y2 {
                    grid[idx_of(x, y)] = Point::Rock;
                }
            }
        }
    }

    // waste a line, but make logic identic to part 1
    if part == 2 {
        for x in origin_x..(origin_x + w) {
            grid[idx_of(x, h - 1)] = Point::Rock;
        }
    }

    // }}}
    // {{{ fill sand

    let mut resting_sand = 0;
    'fill: loop {
        let (mut x, mut y) = (sand_x, sand_y);
        loop {
            if part == 1 && y + 1 == h {
                break 'fill;
            }
            debug_assert_eq!(grid[idx_of(x, y)], Point::Air);
            // down
            if grid[idx_of(x, y + 1)] == Point::Air {
                (x, y) = (x, y + 1); continue;
            }
            // down-left
            if grid[idx_of(x - 1, y + 1)] == Point::Air {
                (x, y) = (x - 1, y + 1); continue;
            }
            // down-right
            if grid[idx_of(x + 1, y + 1)] == Point::Air {
                (x, y) = (x + 1, y + 1); continue;
            }
            grid[idx_of(x, y)] = Point::Sand;
            resting_sand += 1;
            if part == 2 && x == sand_x && y == sand_y {
                break 'fill;
            }
            (x, y) = (sand_x, sand_y);
        }
    }

    // println!("just to look!");
    // for y in 0..h {
    //     for x in origin_x..(origin_x + w) {
    //         print!("{}", match grid[idx_of(x, y)] {
    //             Point::Air => '.',
    //             Point::Rock => '#',
    //             Point::Sand => 'o',
    //         });
    //     }
    //     println!("");
    // }
    // }}}

    Solution::USIZE(resting_sand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"), Solution::USIZE(24));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::USIZE(696));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"), Solution::USIZE(93));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::USIZE(23610));
    }
}
