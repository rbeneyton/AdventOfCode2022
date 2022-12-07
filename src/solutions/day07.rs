use crate::Solution;
use rustc_hash::FxHashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct Dir {
    content : FxHashMap<&'static str, Rc<RefCell<Self>>>,
    size: u64,
}
impl Dir {
    pub fn sum_small_node(&self) -> u64 {
        let mut res = if self.size <= 100000 { self.size } else { 0 };
        for (_, child) in &self.content {
            res += child.borrow().sum_small_node();
        }
        res
    }
    pub fn min_greater_than(&self, to_free : u64) -> u64 {
        let mut res = u64::MAX;
        if self.size > to_free {
            res = std::cmp::min(self.size, res);
        }
        for (_, child) in &self.content {
            res = std::cmp::min(res, child.borrow().min_greater_than(to_free));
        }
        res
    }
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/07.input")
    };

    // fill tree
    let root = Rc::new(RefCell::new(Dir::default()));
    let mut pwd = vec![root.clone()];
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            match line.chars().skip(5).next().unwrap() {
                '/' => { pwd.truncate(1); },
                '.' => { pwd.pop(); },
                _ => {
                    let new_dir = Rc::new(RefCell::new(Dir::default()));
                    // XXX check if already defined/visited
                    debug_assert!(!pwd.last_mut().unwrap().borrow_mut().content.contains_key(&line[5..]));
                    pwd.last_mut().unwrap().borrow_mut().content.insert(&line[5..], new_dir.clone());
                    pwd.push(new_dir);
                },
            }
        } else {
            if line.chars().next().unwrap().is_digit(10) {
                let sz = line.split_whitespace().next().unwrap().parse::<u64>();
                if let Ok(sz) = sz {
                    for dir in &pwd {
                        dir.borrow_mut().size += sz;
                    }
                }
            }
        }
    }

    if part == 1 {
        Solution::U64(root.borrow().sum_small_node())
    } else {
        const SPACE : u64 = 70000000;
        const REQUIRED : u64 = 30000000;
        let used = root.borrow().size;
        debug_assert!(used < SPACE);
        let free = SPACE - used;
        debug_assert!(free < REQUIRED);
        let to_free = REQUIRED - free;

        Solution::U64(root.borrow().min_greater_than(to_free))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"), Solution::U64(95437));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(1390824));
    }

    #[test]
    #[allow(unused)]
    fn part_2_sample() {
        assert_eq!(solve(2, r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"), Solution::U64(24933642));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(7490863));
    }
}
