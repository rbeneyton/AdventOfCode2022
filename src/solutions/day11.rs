use crate::Solution;
// use rustc_hash::FxHashMap;
// 
// pub fn is_prime(n: u64) -> bool {
//   if n < 4 {
//     n > 1
//   } else if n % 2 == 0 || n % 3 == 0 {
//     false
//   } else {
//     let max_p = (n as f64).sqrt().ceil() as u64;
//     match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
//       Some(_) => false,
//       None => true
//     }
//   }
// }
// 
// pub struct Prime {
//   curr: u64,
//   next: u64,
// }
// 
// impl Prime {
//   pub fn new() -> Prime {
//     Prime {
//       curr: 2,
//       next: 3,
//     }
//   }
// }
// 
// impl Iterator for Prime {
//   type Item = u64;
// 
//   fn next(&mut self) -> Option<Self::Item> {
//     let prime = self.curr;
//     self.curr = self.next;
//     loop {
//       self.next += match self.next%6 {
//         1 => 4,
//         _ => 2,
//       };
//       if is_prime(self.next) {
//         break;
//       }
//     }
//     Some(prime)
//   }
// }
// 
// #[derive(Debug, Clone)]
// struct Num {
//     pub modes : FxHashMap::<u64, u32>,
// }
// 
// impl Num {
//     pub fn new(k: u64) -> Self {
//         let mut modes = FxHashMap::default();
//         let mut w = k;
//         if w != 1 {
//             for prime in Prime::new() {
//                 let mut f = 0;
//                 let factor = loop {
//                     if w % prime == 0 {
//                         w = w / prime;
//                         f += 1;
//                     } else {
//                         break f;
//                     }
//                 };
//                 if factor > 0 {
//                     modes.insert(prime, factor);
//                     if w == 1 {
//                         break;
//                     }
//                 }
//                 // fast exit
//                 if is_prime(w) {
//                     modes.insert(w, 1);
//                     break;
//                 }
//             }
//         }
//         Self {
//             modes
//         }
//     }
// 
//     pub fn decompose(&self) -> u64 {
//         let mut v = 1;
//         for (p, k) in self.modes.iter() {
//             v *= p.pow(*k);
//         }
//         v
//     }
// 
//     pub fn add(self, f: u64) -> Self {
//         let mut v = self.decompose();
//         v += f;
//         Self::new(v)
//     }
// 
//     pub fn mul(mut self, f: u64) -> Self {
//         if is_prime(f) {
//             self.modes.entry(f).and_modify(|k| *k += 1).or_insert(1);
//         } else {
//             let mut w = f;
//             for prime in Prime::new() {
//                 let mut f = 0;
//                 let factor = loop {
//                     if w % prime == 0 {
//                         w = w / prime;
//                         f += 1;
//                     } else {
//                         break f;
//                     }
//                 };
//                 if factor > 0 {
//                     self.modes.entry(w).and_modify(|k| *k += factor).or_insert(factor);
//                     if w == 1 {
//                         break;
//                     }
//                 }
//                 // fast exit
//                 if is_prime(w) {
//                     self.modes.entry(w).and_modify(|k| *k += 1).or_insert(1);
//                     break;
//                 }
//             }
//         }
//         self
//     }
// 
//     pub fn mulself(mut self) -> Self {
//         for (_, k) in self.modes.iter_mut() {
//             *k *= 2;
//         }
//         self
//     }
// 
//     pub fn div(&mut self, f: u64) {
//         if self.is_div(f) {
//             let div = Num::new(f);
//             debug_assert_eq!(div.modes.len(), 1);
//             for (p, pk) in div.modes {
//                 self.modes.entry(p).and_modify(|k| *k -= pk);
//             }
//         } else {
//             let mut v = self.decompose();
//             v /= f;
//             *self = Self::new(v)
//         }
//     }
// 
//     pub fn is_div(&self, f: u64) -> bool {
//         let div = Num::new(f);
//         for (p, k) in div.modes {
//             if let Some(f) = self.modes.get(&p) {
//                 if *f >= k {
//                     continue;
//                 }
//             }
//             return false;
//         }
//         true
//     }
// }
// 
// #[test]
// fn test_num() {
//     assert_eq!(Num::new(1).modes.len(), 0);
//     assert_eq!(Num::new(2 * 3).modes[&2], 1);
//     assert_eq!(Num::new(2 * 3).modes[&3], 1);
//     assert_eq!(Num::new(3 * 3).modes[&3], 2);
//     assert_eq!(Num::new(3 * 3).decompose(), 9);
//     assert_eq!(Num::new(2).add(1).modes[&3], 1);
//     assert_eq!(Num::new(3 * 3).mul(2).modes[&2], 1);
//     assert_eq!(Num::new(3 * 3).mul(3).modes[&3], 3);
//     assert_eq!(Num::new(3 * 3).mulself().modes[&3], 4);
//     let mut n = Num::new(3 * 3);
//     n.div(3);
//     assert_eq!(n.modes[&3], 1);
//     let mut n = Num::new(3);
//     n.div(3);
//     assert_eq!(n.modes.len(), 0);
//     assert_eq!(Num::new(3).is_div(3), true);
//     assert_eq!(Num::new(3 * 3).is_div(3), true);
//     assert_eq!(Num::new(2 * 2).is_div(3), false);
// }
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Add(u64),
    Mul(u64),
    MulSelf,
}
impl Op {
    pub fn new(i: &'static str) -> Self {
        let mut tok = i.split_whitespace();
        assert_eq!(tok.next(), Some("old"));
        match tok.next() {
            Some("+") => Op::Add(tok.next().unwrap().parse::<u64>().unwrap()),
            Some("*") => match tok.next() {
                Some("old") => Op::MulSelf,
                Some(num) => Op::Mul(num.parse::<u64>().unwrap()),
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    }
}
#[test]
fn test_op() {
    assert_eq!(Op::new("old + 4"), Op::Add(4));
    assert_eq!(Op::new("old * 7"), Op::Mul(7));
    assert_eq!(Op::new("old + 7"), Op::Add(7));
    assert_eq!(Op::new("old * 3"), Op::Mul(3));
    assert_eq!(Op::new("old + 3"), Op::Add(3));
    assert_eq!(Op::new("old * old"), Op::MulSelf);
    assert_eq!(Op::new("old + 8"), Op::Add(8));
    assert_eq!(Op::new("old + 2"), Op::Add(2));
    assert_eq!(Op::new("old + 4"), Op::Add(4));
}
#[derive(Debug, Clone)]
struct Monkey {
    id : usize,
    items : Vec<u64>, // Vec<Num>
    operation : Op,
    test : u64,
    true_to : usize,
    false_to : usize,
    inspected : usize,
}
impl Monkey {
    pub fn new(i: &'static str) -> Self {
        let mut line = i.lines();
        let id = line.next().unwrap();
        assert!(id.starts_with("Monkey "));
        let (_, id) = id.split_at("Monkey ".len());
        let id = id.split(":")
            .map(|x| x.parse::<usize>().unwrap())
            .next().unwrap();
        let items = line.next().unwrap();
        assert!(items.starts_with("  Starting items: "));
        let (_, items) = items.split_at("  Starting items: ".len());
        let items = items.split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            // .map(|x| Num::new(x))
            .collect::<Vec<_>>();
        let operation = line.next().unwrap();
        assert!(operation.starts_with("  Operation: new = "));
        let (_, operation) = operation.split_at("  Operation: new = ".len());
        let operation = Op::new(operation);
        let test = line.next().unwrap();
        assert!(test.starts_with("  Test: divisible by "));
        let (_, test) = test.split_at("  Test: divisible by ".len());
        let test = test.parse::<u64>().unwrap();
        let true_to = line.next().unwrap();
        assert!(true_to.starts_with("    If true: throw to monkey "));
        let (_, true_to) = true_to.split_at("    If true: throw to monkey ".len());
        let true_to = true_to.parse::<usize>().unwrap();
        let false_to = line.next().unwrap();
        assert!(false_to.starts_with("    If false: throw to monkey "));
        let (_, false_to) = false_to.split_at("    If false: throw to monkey ".len());
        let false_to = false_to.parse::<usize>().unwrap();
        Self {
            id,
            items,
            operation,
            test,
            true_to,
            false_to,
            inspected : 0,
        }
    }
}
pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/11.input")
    };
    let mut monkeys = input
        .split("\n\n")
        .map(|x| Monkey::new(x))
        .collect::<Vec<_>>();
    for (idx, monkey) in monkeys.iter().enumerate() {
        debug_assert_eq!(idx, monkey.id);
    }
    let common_diviser : u64 = monkeys.iter().map(|m| m.test).product();
    let max_rounds = if part == 1 { 20 } else { 10_000 };
    for round in 1..=max_rounds {
        for step in 0..monkeys.len() {
            let mut items = Vec::new();
            std::mem::swap(&mut monkeys[step].items, &mut items);
            monkeys[step].inspected += items.len();
            for item in items {
                let mut worry_level = match monkeys[step].operation {
                    Op::Add(incr) => item + incr,//item.add(incr),
                    Op::Mul(v) => item * v,//item.mul(v),
                    Op::MulSelf => item * item,//item.mulself(),
                };
                if part == 1 {
                    worry_level /= 3;//worry_level.div(3);
                }
                // only important part
                worry_level %= common_diviser;
                //let to = if worry_level.is_div(monkeys[step].test) {
                let to = if worry_level % monkeys[step].test == 0 {
                    monkeys[step].true_to
                } else {
                    monkeys[step].false_to
                };
                monkeys[to].items.push(worry_level);
            }
        }
    }
    let mut inspected = monkeys.iter().map(|x| x.inspected).collect::<Vec<_>>();
    inspected.sort();
    inspected.reverse();
    Solution::USIZE(inspected[0] * inspected[1])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"), Solution::USIZE(10605));
    }
    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::USIZE(55216));
    }
    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"), Solution::USIZE(2713310158));
    }
    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::USIZE(12848882750));
    }
}
