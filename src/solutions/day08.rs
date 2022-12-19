use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/08.input")
    };

    let w = input.lines().next().unwrap().chars().count();
    let h = input.lines().count();
    debug_assert_eq!(w, h);
    let n = w;

    if part == 1 {
        let mut grid = Vec::new();
        grid.resize(n * n, 0u8);
        let idx_of = |x, y| x + n * y;

        let mut i = input.chars();
        let offset = if part == 1 { 1 } else { 0 };
        for row in 0..n {
            for col in 0..n {
                let c = i.next().unwrap();
                grid[idx_of(col, row)] = offset + c.to_digit(10).unwrap() as u8;
            }
            assert_eq!(i.next(), Some('\n'));
        }
        debug_assert_eq!(i.next(), None);

        const VALUE : u8 = 0x0F;
        const MASKS : [u8; 4] = [0x10, 0x20, 0x40, 0x80];
        const VISIBLE : u8 = 0xF0;

        let mut scan = |max: &mut u8, row, col, mask| {
            let v = grid[idx_of(col, row)] & VALUE;
            if v > *max {
                *max = v;
                grid[idx_of(col, row)] |= mask;
            }
        };

        let mut max;
        for row in 0..n {
            max = 0;
            for col in 0..n {
                scan(&mut max, row, col, MASKS[0])
            }
        }
        for row in 0..n {
            max = 0;
            for col in (0..n).rev() {
                scan(&mut max, row, col, MASKS[1])
            }
        }
        for col in 0..n {
            max = 0;
            for row in 0..n {
                scan(&mut max, row, col, MASKS[2])
            }
        }
        for col in 0..n {
            max = 0;
            for row in (0..n).rev() {
                scan(&mut max, row, col, MASKS[3])
            }
        }

        Solution::U64(grid
            .iter()
            .filter(|v| (*v & VISIBLE) != 0)
            .count() as u64)
    } else {
        let n = n + 2;
        let n_ = n - 1;
        let mut grid = Vec::new();
        grid.resize(n * n, 0u8);
        let idx_of = |x, y| x + n * y;

        let mut i = input.chars();
        let offset = if part == 1 { 1 } else { 0 };
        for row in 1..n_ {
            for col in 1..n_ {
                let c = i.next().unwrap();
                grid[idx_of(col, row)] = offset + c.to_digit(10).unwrap() as u8;
            }
            assert_eq!(i.next(), Some('\n'));
        }
        debug_assert_eq!(i.next(), None);

        let mut score = Vec::new();
        score.resize(n * n, 1u32);

        for row in 1..n_ {
            // {{{ RIGHT
            // digit scanner grid
            let mut scans = [0; 10];
            for col in 1..n_ {
                // invariant: scans always contains next position of 'digit'
                for scol in (col + 1)..n_ {
                    let digit = (grid[idx_of(scol, row)]) as usize;
                    if scans[digit] <= col {
                        scans[digit] = scol;
                        if (0..10).filter(|d| scans[*d] > col).count() == 10 {
                            break;
                        }
                    }
                }
                // avoid continuous miss scan
                for d in 0..10 {
                    if scans[d] <= col {
                        scans[d] = n_;
                    }
                }

                let digit = (grid[idx_of(col, row)]) as usize;
                // greater tree
                let (upper_idx, upper_digit) = (digit..10)
                    .map(|d| (scans[d], d))
                    .min_by(|a, b| a.0.cmp(&b.0))
                    .expect("should not happen");
                debug_assert!(upper_idx >= col);
                // same size tree case
                let dist = if upper_digit == digit {
                    upper_idx - col
                } else {
                    debug_assert!(upper_digit > digit);
                    upper_idx - col - 1
                };
                score[idx_of(col, row)] *= dist as u32;
            }
            // }}}
            // {{{ LEFT
            // digit scanner grid
            let mut scans = [n; 10];
            for col in (1..n_).rev() {
                // invariant: scans always contains next position of 'digit'
                for scol in (1..(col - 1)).rev() {
                    let digit = (grid[idx_of(scol, row)]) as usize;
                    if scans[digit] >= col {
                        scans[digit] = scol;
                        if (0..10).filter(|d| scans[*d] < col).count() == 10 {
                            break;
                        }
                    }
                }
                // avoid continuous miss scan
                for d in 0..10 {
                    if scans[d] >= col {
                        scans[d] = 0;
                    }
                }
                let digit = (grid[idx_of(col, row)]) as usize;
                // greater tree
                let (upper_idx, upper_digit) = (digit..10)
                    .map(|d| (scans[d], d))
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .expect("should not happen");
                debug_assert!(upper_idx <= col);
                // same size tree case
                let dist = if upper_digit == digit {
                    col - upper_idx
                } else {
                    col - 1 - upper_idx
                };
                score[idx_of(col, row)] *= dist as u32;
            }
            // }}}
        }

        // println!("A");
        // for row in 1..n_ {
        //     for col in 1..n_ {
        //         print!("{} ", score[idx_of(row, col)]);
        //     }
        //     println!("");
        // }

        let mut grid_rot = Vec::new();
        grid_rot.resize(n * n, 0u8);
        for row in 1..n_ {
            for col in 1..n_ {
                grid_rot[idx_of(row, col)] = grid[idx_of(col, row)];
            }
        }
        let grid = grid_rot;

        for row in 1..n_ {
            // {{{ RIGHT
            // digit scanner grid
            let mut scans = [0; 10];
            for col in 1..n_ {
                // invariant: scans always contains next position of 'digit'
                for scol in (col + 1)..n_ {
                    let digit = (grid[idx_of(scol, row)]) as usize;
                    if scans[digit] <= col {
                        scans[digit] = scol;
                        if (0..10).filter(|d| scans[*d] > col).count() == 10 {
                            break;
                        }
                    }
                }
                // avoid continuous miss scan
                for d in 0..10 {
                    if scans[d] <= col {
                        scans[d] = n_;
                    }
                }

                let digit = (grid[idx_of(col, row)]) as usize;
                // greater tree
                let (upper_idx, upper_digit) = (digit..10)
                    .map(|d| (scans[d], d))
                    .min_by(|a, b| a.0.cmp(&b.0))
                    .expect("should not happen");
                debug_assert!(upper_idx >= col);
                // same size tree case
                let dist = if upper_digit == digit {
                    upper_idx - col
                } else {
                    debug_assert!(upper_digit > digit);
                    upper_idx - col - 1
                };
                score[idx_of(row, col)] *= dist as u32;
            }
            // }}}
            // {{{ LEFT
            // digit scanner grid
            let mut scans = [n_; 10];
            for col in (1..n_).rev() {
                // invariant: scans always contains next position of 'digit'
                for scol in (1..(col - 1)).rev() {
                    let digit = (grid[idx_of(scol, row)]) as usize;
                    if scans[digit] >= col {
                        scans[digit] = scol;
                        if (0..10).filter(|d| scans[*d] < col).count() == 10 {
                            break;
                        }
                    }
                }
                // avoid continuous miss scan
                for d in 0..10 {
                    if scans[d] >= col {
                        scans[d] = 0;
                    }
                }
                let digit = (grid[idx_of(col, row)]) as usize;
                // greater tree
                let (upper_idx, upper_digit) = (digit..10)
                    .map(|d| (scans[d], d))
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .expect("should not happen");
                debug_assert!(upper_idx <= col);
                // same size tree case
                let dist = if upper_digit == digit {
                    col - upper_idx
                } else {
                    col - 1 - upper_idx
                };
                score[idx_of(row, col)] *= dist as u32;
            }
            // }}}
        }

        // println!("B");
        // for row in 1..n_ {
        //     for col in 1..n_ {
        //         print!("{} ", score[idx_of(row, col)]);
        //     }
        //     println!("");
        // }

        // for col in 1..n_ {
        //     // {{{ DOWN
        //     // digit scanner grid
        //     let mut scans = [None; 10];
        //     for row in 1..n_ {
        //         // invariant: scans always contains next position of 'digit'
        //         for srow in (row + 1)..n_ {
        //             let digit = (grid[idx_of(col, srow)]) as usize;
        //             if scans[digit].is_none() || scans[digit].unwrap() <= row {
        //                 scans[digit] = Some(srow);
        //                 if (0..10).filter(|d| scans[*d].is_some()).count() == 10 {
        //                     break;
        //                 }
        //             }
        //         }
        //         let digit = (grid[idx_of(col, row)]) as usize;
        //         // greater tree
        //         let (upper_idx, upper_digit) = (digit..10)
        //                 .filter_map(|d| if let Some(v) = scans[d] {
        //                     if v > row {
        //                         Some((v, d))
        //                     } else {
        //                         None
        //                     }
        //                 } else {
        //                     None
        //                 })
        //             .min_by(|a, b| a.0.cmp(&b.0))
        //             .unwrap_or((n_, 10));
        //             debug_assert!(upper_idx >= row);
        //         // same size tree case
        //         let dist = if upper_digit == digit {
        //             upper_idx - row
        //         } else {
        //             debug_assert!(upper_digit > digit);
        //             upper_idx - row - 1
        //         };
        //         score[idx_of(col, row)] *= dist as u32;
        //     }
        //     // }}}
        //     // {{{ UP
        //     // digit scanner grid
        //     let mut scans = [None; 10];
        //     for row in (1..n_).rev() {
        //         // invariant: scans always contains next position of 'digit'
        //         for srow in (1..(row - 1)).rev() {
        //             let digit = (grid[idx_of(col, srow)]) as usize;
        //             if scans[digit].is_none() || scans[digit].unwrap() >= row {
        //                 scans[digit] = Some(srow);
        //                 if (0..10).filter(|d| scans[*d].is_some()).count() == 10 {
        //                     break;
        //                 }
        //             }
        //         }
        //         let digit = (grid[idx_of(col, row)]) as usize;
        //         // greater tree
        //         let (upper_idx, upper_digit) = (digit..10)
        //                 .filter_map(|d| if let Some(v) = scans[d] {
        //                     if v < row {
        //                         Some((v, d))
        //                     } else {
        //                         None
        //                     }
        //                 } else {
        //                     None
        //                 })
        //             .max_by(|a, b| a.0.cmp(&b.0))
        //             .unwrap_or((0, 10));
        //         debug_assert!(upper_idx <= row);
        //         // same size tree case
        //         let dist = if upper_digit == digit {
        //             row - upper_idx
        //         } else {
        //             row - 1 - upper_idx
        //         };
        //         score[idx_of(col, row)] *= dist as u32;
        //     }
        //     // }}}
        // }

        Solution::U32(*score
            .iter()
            .max()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"30373
25512
65332
33549
35390
"), Solution::U64(21));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(1823));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"30373
25512
65332
33549
35390
"), Solution::U32(8));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U32(211680));
    }
}
