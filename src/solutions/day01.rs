use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/01.input")
    };

    if part == 1 {
        let mut res = 0;
        let mut line = input.lines();
        loop {
            let mut cur = 0;
            let eof = loop {
                if let Some(line) = line.next() {
                    if let Ok(value) = line.parse::<i64>() {
                        cur += value;
                        continue;
                    }
                } else {
                    break true;
                }
                break false;
            };
            res = std::cmp::max(cur, res);
            if eof {
                break;
            }
        }
        Solution::I64(res)
    } else {
        let mut res = [0; 3];
        let mut line = input.lines();
        loop {
            let mut cur = 0;
            let eof = loop {
                if let Some(line) = line.next() {
                    if let Ok(value) = line.parse::<i64>() {
                        cur += value;
                        continue;
                    }
                } else {
                    break true;
                }
                break false;
            };
            for i in 0..3 {
                if cur > res[i] {
                    for j in (i + 1..3).rev() {
                        res[j] = res[j - 1];
                    }
                    res[i] = cur;
                    break;
                }
            }
            if eof {
                break;
            }
        }
        Solution::I64(res.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_last() {
        let input = r"
1

2
3";
        assert_eq!(solve(1, input), Solution::I64(5));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::I64(72602));
    }

    #[test]
    fn part_2_basic() {
        let input = r"
1

2

3

4";
        assert_eq!(solve(2, input), Solution::I64(9));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::I64(207410));
    }
}
