use itertools::{
    chain,
    join,
};
use crate::Solution;

pub type Dir<K, V> = rustc_hash::FxHashMap<K, V>;
// pub type Dir<K, V> = std::collections::BTreeMap<K, V>; // for debug

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/07.input")
    };

    // fill tree
    let root = vec!["",];
    let mut pwd = Vec::new();
    let mut sizes = Dir::default();
    let mut node = sizes.entry(String::from("")).or_insert(0);
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            match line.chars().skip(5).next().unwrap() {
                '/' => { pwd.clear(); },
                '.' => { pwd.pop(); },
                _ => { pwd.push(&line[5..]); },
            }
            let path = join(chain(&root, &pwd), "/");
            node = sizes.entry(path).or_insert(0);
        } else {
            if line.chars().next().unwrap().is_digit(10) {
                let sz = line.split_whitespace().next().unwrap().parse::<u64>();
                if let Ok(sz) = sz {
                    *node += sz;
                }
            }
        }
    }

    // fill accumulative tree
    let mut acc_sizes = Dir::default();
    for (path, sz) in sizes {
        acc_sizes.entry(path.clone()).or_insert(sz);
        let mut dir = path.clone();
        while let Some(trailing) = dir.rfind('/') {
            dir.split_off(trailing);
            let node = acc_sizes.entry(dir.clone()).or_insert(0);
            *node += sz;
        }
    }

    if part == 1 {
        Solution::U64(acc_sizes
            .iter()
            .filter_map(|(path, sz)| if path.len() > 0 && *sz <= 100000 {
                Some(*sz)
            } else {
                None
            })
            .sum())
    } else {
        const SPACE : u64 = 70000000;
        const REQUIRED : u64 = 30000000;
        let used = *acc_sizes.get(&String::from("")).unwrap();
        debug_assert!(used < SPACE);
        let free = SPACE - used;
        debug_assert!(free < REQUIRED);
        let to_free = REQUIRED - free;

        Solution::U64(acc_sizes
            .iter()
            .filter_map(|(_, sz)| if *sz >= to_free {
                Some(*sz)
            } else {
                None
            })
            .min()
            .unwrap())
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
