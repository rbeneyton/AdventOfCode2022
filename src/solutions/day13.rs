use crate::Solution;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u16),
}
impl Packet {
    pub fn new(i: &'static str) -> Self {
        let mut ints = vec![vec![]];
        for tok in i.split_inclusive(&['[', ',', ']'][..]) {
            let (tok, sep) = tok.split_at(tok.len() - 1);
            if tok.len() > 0 {
                let int = tok.parse::<u16>().unwrap();
                ints.last_mut().unwrap().push(Packet::Int(int))
            }
            match sep {
                "[" => {
                    ints.push(vec![]);
                },
                "]" => {
                    let pop = ints.pop().unwrap();
                    ints.last_mut().unwrap().push(Packet::List(pop));
                },
                "," => (),
                _ => panic!(),
            }
        }
        debug_assert_eq!(ints.len(), 1);
        Packet::List(ints.pop().unwrap())
    }

    pub fn is_right(left : &Packet, right : &Packet) -> Ordering {
        match (left, right) {
            (Packet::Int(left), Packet::Int(right)) => left.cmp(&right),
            (Packet::List(left), Packet::List(right)) => {
                let (mut left, mut right) = (left.iter(), right.iter());
                loop {
                    match (left.next(), right.next()) {
                        (None, None) => break Ordering::Equal,
                        (None, _) => break Ordering::Less,
                        (_, None) => break Ordering::Greater,
                        (Some(ref left), Some(ref right)) => {
                            match Packet::is_right(left, right) {
                                Ordering::Equal => continue,
                                x => break x,
                            }
                        }
                    }
                }
            },
            (left, Packet::Int(right)) =>
                Packet::is_right(left, &Packet::List(vec![Packet::Int(*right)])),
            (Packet::Int(left), right) =>
                Packet::is_right(&Packet::List(vec![Packet::Int(*left)]), right),
        }
    }
}
#[test]
fn test_packet() {
    assert_eq!(Packet::new(""), Packet::List(vec![]));
    assert_eq!(Packet::new("[]"), Packet::List(vec![Packet::List(vec![
    ])]));
    assert_eq!(Packet::new("[0,0]"), Packet::List(vec![Packet::List(vec![
        Packet::Int(0),
        Packet::Int(0),
    ])]));
    assert_eq!(Packet::new("[0,[1]]"), Packet::List(vec![Packet::List(vec![
        Packet::Int(0),
        Packet::List(vec![
            Packet::Int(1),
        ]),
    ])]));
    assert_eq!(Packet::new("[0,[1,2]]"), Packet::List(vec![Packet::List(vec![
        Packet::Int(0),
        Packet::List(vec![
            Packet::Int(1),
            Packet::Int(2),
        ]),
    ])]));
    assert_eq!(Packet::new("[0,[2,]]"), Packet::List(vec![Packet::List(vec![
        Packet::Int(0),
        Packet::List(vec![
            Packet::Int(2),
        ]),
    ])]));
    assert_eq!(Packet::new("[[0],1]"), Packet::List(vec![Packet::List(vec![
        Packet::List(vec![
            Packet::Int(0),
        ]),
        Packet::Int(1),
    ])]));
    assert_eq!(Packet::new("[[[0]],1]"), Packet::List(vec![Packet::List(vec![
        Packet::List(vec![
            Packet::List(vec![
                Packet::Int(0),
            ]),
        ]),
        Packet::Int(1),
    ])]));
}

#[derive(Debug, Clone)]
struct Pair {
    left : Packet,
    right : Packet,
}
impl Pair {
    pub fn new(i: &'static str) -> Self {
        let mut i = i.lines();
        Self {
            left : Packet::new(i.next().unwrap()),
            right : Packet::new(i.next().unwrap()),
        }
    }

    pub fn is_right(self) -> bool {
        Packet::is_right(&self.left, &self.right) == Ordering::Less
    }
}
#[test]
fn test_pair() {
    assert_eq!(Pair::new("[1,1,3,1,1]\n[1,1,5,1,1]").is_right(), true);
    assert_eq!(Pair::new("[[1],[2,3,4]]\n[[1],4]").is_right(), true);
    assert_eq!(Pair::new("[9]\n[[8,7,6]]").is_right(), false);
    assert_eq!(Pair::new("[[4,4],4,4]\n[[4,4],4,4,4]").is_right(), true);
    assert_eq!(Pair::new("[7,7,7,7]\n[7,7,7]").is_right(), false);
    assert_eq!(Pair::new("[]\n[3]").is_right(), true);
    assert_eq!(Pair::new("[[[]]]\n[[]]").is_right(), false);
    assert_eq!(Pair::new("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]").is_right(), false);
}


pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/13.input")
    };

    if part == 1 {
        Solution::USIZE(input
            .split("\n\n")
            .map(|x| Pair::new(x).is_right())
            .enumerate()
            .filter_map(|(idx, x)| if x { Some(idx + 1) } else { None })
            .sum())
    } else {
        let mut packets = input
            .lines()
            .filter(|x| x.len() != 0)
            .map(|x| Packet::new(x))
            .map(|x| (x, false))
            .collect::<Vec<_>>();
        packets.push((Packet::new("[[2]]"), true));
        packets.push((Packet::new("[[6]]"), true));
        packets.sort_by(|a, b| Packet::is_right(&a.0, &b.0));
        Solution::USIZE(packets
            .iter()
            .enumerate()
            .filter_map(|(idx, (_, flag))| if *flag { Some(idx + 1) } else { None })
            .product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"), Solution::USIZE(13));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::USIZE(6272));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"), Solution::USIZE(140));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::USIZE(22288));
    }
}
