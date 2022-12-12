use crate::Solution;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Add(u128),
    Mul(u128),
    MulSelf,
}
impl Op {
    pub fn new(i: &'static str) -> Self {
        let mut tok = i.split_whitespace();
        assert_eq!(tok.next(), Some("old"));
        match tok.next() {
            Some("+") => Op::Add(tok.next().unwrap().parse::<u128>().unwrap()),
            Some("*") => match tok.next() {
                Some("old") => Op::MulSelf,
                Some(num) => Op::Mul(num.parse::<u128>().unwrap()),
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
    items : Vec<u128>,
    operation : Op,
    test : u128,
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
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        let operation = line.next().unwrap();
        assert!(operation.starts_with("  Operation: new = "));
        let (_, operation) = operation.split_at("  Operation: new = ".len());
        let operation = Op::new(operation);

        let test = line.next().unwrap();
        assert!(test.starts_with("  Test: divisible by "));
        let (_, test) = test.split_at("  Test: divisible by ".len());
        let test = test.parse::<u128>().unwrap();

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

    let max_rounds = if part == 1 { 20 } else { 10000 };
    for round in 1..=max_rounds {
        for step in 0..monkeys.len() {
            let mut items = Vec::new();
            std::mem::swap(&mut monkeys[step].items, &mut items);
            monkeys[step].inspected += items.len();
            for item in items {
                let mut worry_level = match monkeys[step].operation {
                    Op::Add(incr) => item + incr,
                    Op::Mul(v) => item * v,
                    Op::MulSelf => item * item,
                };
                if part == 1 {
                    worry_level /= 3;
                }
                if worry_level % monkeys[step].test == 0 {
                    let to = monkeys[step].true_to;
                    monkeys[to].items.push(worry_level);
                } else {
                    let to = monkeys[step].false_to;
                    monkeys[to].items.push(worry_level);
                }
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

    // #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(0));
    }
}
