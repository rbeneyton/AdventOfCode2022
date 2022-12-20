use crate::Solution;
use rustc_hash::FxHashMap;
use std::cmp;

// {{{ Valve

const MAX_TUNNEL : usize = 5;
const MAX_VALVES : usize = 54;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    idx : usize,
    name : &'static str,
    rate : usize,
    dest_str : &'static str,
    dest_sz : usize,
    dest : [usize; MAX_TUNNEL], // max indirect
}
type Valves = Vec<Valve>;

impl Valve {
    pub fn new(input: &'static str) -> Self {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let mut tok = input.split(' ');
        assert_eq!(tok.next(), Some("Valve"));
        let name = tok.next().unwrap();

        let rate = tok.skip(2).next().unwrap();
        assert!(rate.starts_with("rate="));
        let (_, rate) = rate.split_at("rate=".len());
        let rate = rate.split(";").next().unwrap();
        let rate = rate.parse::<usize>().unwrap();

        // seriously, muliple sentence here:
        // ; tunnel leads to valve ...
        // ; tunnels lead to valves ...
        // this is not a parsing contest \o/
        let cut = if let Some(cut) = input.find(" valves ") {
            cut + " valves ".len()
        } else {
            if let Some(cut) = input.find(" valve ") {
                cut + " valve ".len()
            } else {
                panic!("parsing valves");
            }
        };
        let (_, dest_str) = input.split_at(cut);
        Self {
            idx : 0,
            name,
            rate,
            dest_str,
            dest_sz : 0,
            dest : [0; MAX_TUNNEL],
        }
    }

    pub fn new_vec(input: &'static str)
        -> (Vec::<Self>, FxHashMap::<&'static str, usize>)
    {
        let mut valves = Vec::new();
        let mut ids = FxHashMap::default();
        for (idx, line) in input.lines().enumerate() {
            let mut valve = Valve::new(line);
            valve.idx = idx;
            ids.insert(valve.name, idx);
            valves.push(valve);
        }
        let sz = valves.len();
        for valve in valves.iter_mut() {
            for dest in valve.dest_str.split(", ") {
                let dest_idx = *ids.get(dest).expect("invalid ref");
                debug_assert!(dest_idx < sz);
                valve.dest[valve.dest_sz] = dest_idx;
                valve.dest_sz += 1;
                debug_assert!(valve.dest_sz <= MAX_TUNNEL);
            }
        }
        (valves, ids)
    }
}

#[test]
fn test_valve() {
    assert_eq!(Valve::new("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
        Valve {
            idx : 0,
            name : "AA",
            rate : 0,
            dest_str : "DD, II, BB",
            dest_sz : 0,
            dest : [0; MAX_TUNNEL],
        });
}

// }}}
// {{{ State1

const MAX_CHOICE1 : usize = 15 + 20; // extra choice for rate==0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State1 {
    cur_valve : usize,
    rate : usize,
    acc_rate : usize,
    seq_sz : usize,
    seq : [usize; MAX_CHOICE1],
    open : [bool; MAX_VALVES],
    have_waited : bool,
}

impl Default for State1 {
    fn default() -> Self {
        Self {
            cur_valve : 0,
            rate : 0,
            acc_rate : 0,
            seq_sz : 0,
            seq : [0; MAX_CHOICE1],
            open : [false; MAX_VALVES],
            have_waited : false,
        }
    }
}

impl State1 {
    pub fn can_wait(&self) -> bool {
        !self.have_waited
    }

    pub fn wait(mut self) -> Self {
        self.acc_rate += self.rate;
        self.have_waited = true;
        self
    }

    pub fn move_to(mut self, id : usize) -> Self {
        self.cur_valve = id;
        self.seq[self.seq_sz] = id;
        self.seq_sz += 1;
        self.acc_rate += self.rate;
        self.have_waited = false;
        self
    }

    pub fn open(mut self, valves : &Valves) -> Self {
        let id = self.seq[self.seq_sz - 1];
        debug_assert_eq!(self.open[id], false);
        self.cur_valve = id;
        self.open[id] = true;
        self.acc_rate += self.rate;
        self.rate += valves[id].rate;
        self.have_waited = false;
        self
    }
}

impl Ord for State1 {
    fn cmp(&self, other: &State1) -> cmp::Ordering {
        other.acc_rate.cmp(&self.acc_rate)
            //.then_with(|| other.dist.cmp(&self.dist)) // min distance
    }
}

impl PartialOrd for State1 {
    fn partial_cmp(&self, other: &State1) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// }}}
// {{{ State2

const MAX_CHOICE2 : usize = 15 + 20; // extra choice for rate==0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateInt {
    rate : usize,
    cur_valve : usize,
    seq_sz : usize,
    seq : [usize; MAX_CHOICE2],
    have_waited : bool,
}
impl Default for StateInt {
    fn default() -> Self {
        Self {
            rate : 0,
            cur_valve : 0,
            seq_sz : 0,
            seq : [0; MAX_CHOICE2],
            have_waited : false,
        }
    }
}
impl StateInt {
    pub fn can_wait(&self) -> bool {
        !self.have_waited
    }

    pub fn wait(&mut self) {
        // self.acc_rate += self.rate;
        self.have_waited = true;
    }

    pub fn move_to(&mut self, id : usize) {
        self.cur_valve = id;
        self.seq[self.seq_sz] = id;
        self.seq_sz += 1;
        // self.acc_rate += self.rate;
        self.have_waited = false;
    }

    pub fn open(&mut self, valves : &Valves) {
        let id = self.seq[self.seq_sz - 1];
        // debug_assert_eq!(self.open[id], false);
        self.cur_valve = id;
        // self.open[id] = true;
        // self.acc_rate += self.rate;
        self.rate += valves[id].rate;
        self.have_waited = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State2 {
    acc_rate : usize,
    open : [bool; MAX_VALVES],
    actors : [StateInt; 2],
}

impl Default for State2 {
    fn default() -> Self {
        Self {
            acc_rate : 0,
            open : [false; MAX_VALVES],
            actors : [StateInt::default(); 2],
        }
    }
}

impl State2 {
    pub fn wait(mut self, who : usize) -> Self {
        self.acc_rate += self.actors[who].rate;
        self.actors[who].wait();
        self
    }

    pub fn move_to(mut self, who : usize, id : usize) -> Self {
        self.actors[who].move_to(id);
        self.acc_rate += self.actors[who].rate;
        self
    }

    pub fn open(mut self, who : usize, valves : &Valves) -> Self {
        let id = self.actors[who].seq[self.actors[who].seq_sz - 1];
        debug_assert_eq!(self.open[id], false);
        self.open[id] = true;
        self.acc_rate += self.actors[who].rate;
        self.actors[who].open(valves);
        self
    }

    pub fn utility(&self, utility : &Vec<usize>) -> usize {
        2 * self.acc_rate
        + utility[self.actors[0].cur_valve]
        + utility[self.actors[1].cur_valve]
    }
}

impl Ord for State2 {
    fn cmp(&self, other: &State2) -> cmp::Ordering {
        other.acc_rate.cmp(&self.acc_rate)
            //.then_with(|| other.dist.cmp(&self.dist)) // min distance
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &State2) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// }}}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/16.input")
    };

    let (valves, valves_by_name) = Valve::new_vec(input);

    let valve_of = |name| {
        let idx = valves_by_name.get(name).expect("invalid valve name");
        debug_assert!(*idx < valves.len());
        &valves[*idx]
    };

    // {{{ compute distance matrix

    let mut distances = FxHashMap::default();
    {
        for valve in &valves {
            let mut hit = FxHashMap::default();
            let mut froms = Vec::new();
            let mut tos = Vec::new();
            froms.push(valve.idx);
            let mut dist = 1;
            loop {
                for pos in &froms {
                    let cur_valve = &valves[*pos];
                    for choice in (0..(cur_valve.dest_sz))
                        .map(|x| cur_valve.dest[x])
                    {
                        if !hit.contains_key(&choice) {
                            hit.insert(choice, dist);
                            tos.push(choice);
                        }
                    }
                }
                froms.clear();
                if tos.len() == 0 {
                    break;
                }
                std::mem::swap(&mut froms, &mut tos);
                dist += 1;
            }
            for (dest, dist) in hit {
                distances.insert((valve.idx, dest), dist);
            }
        }
    }

    // println!("");
    // for from in 0..valves.len() {
    //     for to in 0..valves.len() {
    //         print!("{:02} ", *distances.get(&(from, to)).unwrap());
    //     }
    //     println!("");
    // }

    // }}}
    // {{{ compute utility vector

    let mut utility = Vec::default();
    utility.resize(valves.len(), 0);
    for from in &valves {
        utility[from.idx] += from.rate;
        for to in &valves {
            let dist : usize = *distances.get(&(from.idx, to.idx)).unwrap();
            utility[from.idx] += to.rate / (1 + dist).pow(2);
        }
    }

    // for from in 0..valves.len() {
    //     print!("{:02} ", valves[from].rate);
    // }
    // println!("");
    // for from in 0..valves.len() {
    //     print!("{:02} ", utility[from]);
    // }
    // println!("");

    // }}}

    if part == 1 {
        let mut state = State1::default();
        state.cur_valve = valve_of("AA").idx;
        state.seq[state.seq_sz] = state.cur_valve;
        state.seq_sz += 1;

        let mut froms = Vec::new();
        let mut tos = Vec::new();
        froms.push(state);

        for time in 0..30 {

            for state in &froms {

                if state.can_wait() {
                    tos.push(state.wait());
                }
                if !state.open[state.cur_valve] {
                    tos.push(state.open(&valves));
                }
                let cur_valve = &valves[state.cur_valve];
                for choice in (0..(cur_valve.dest_sz))
                    .map(|x| cur_valve.dest[x])
                {
                    tos.push(state.move_to(choice));
                }
            }
            froms.clear();
            if tos.len() == 0 {
                panic!("");
                break;
            }
            tos.sort();
            // println!("time:{} len:{}", time, tos.len());
            tos.truncate(10_000);

            std::mem::swap(&mut froms, &mut tos);
        }

        Solution::USIZE(froms.iter()
            .map(|x| x.acc_rate)
            .max()
            .unwrap())
    } else {
        let mut state = State2::default();
        for actor in state.actors.iter_mut() {
            actor.cur_valve = valve_of("AA").idx;
            actor.seq[actor.seq_sz] = actor.cur_valve;
            actor.seq_sz += 1;
        }

        let mut froms = Vec::new();
        let mut tos = Vec::new();
        froms.push(state);
        let mut tmps = Vec::new();

        for time in 0..26 {

            for state in &froms {
                tmps.truncate(0);

                // {{{ actor 0
                if state.actors[0].can_wait() {
                    tmps.push(state.wait(0));
                }
                if !state.open[state.actors[0].cur_valve] {
                    tmps.push(state.open(0, &valves));
                }
                let cur_valve = &valves[state.actors[0].cur_valve];
                for choice in (0..(cur_valve.dest_sz))
                    .map(|x| cur_valve.dest[x])
                    // XXX heuristics
                    .filter(|x| *x != state.actors[1].cur_valve)
                {
                    tmps.push(state.move_to(0, choice));
                }
                // }}}

                for tmp in &tmps {

                    if tmp.actors[1].can_wait() {
                        tos.push(tmp.wait(1));
                    }
                    if !tmp.open[tmp.actors[1].cur_valve] {
                        tos.push(tmp.open(1, &valves));
                    }
                    let cur_valve = &valves[tmp.actors[1].cur_valve];
                    for choice in (0..(cur_valve.dest_sz))
                        .map(|x| cur_valve.dest[x])
                        // XXX heuristics
                        //.filter(|x| !tmp.open[*x])
                        .filter(|x| *x != tmp.actors[0].cur_valve)
                    {
                        tos.push(tmp.move_to(1, choice));
                    }
                }
            }
            froms.clear();
            if tos.len() == 0 {
                panic!("");
                break;
            }
            tos.sort_by(|a, b| b.utility(&utility).cmp(&a.utility(&utility)));
            println!("time:{} len:{}", time, tos.len());
            tos.truncate(200_000);

            std::mem::swap(&mut froms, &mut tos);
        }

        Solution::USIZE(froms.iter()
            .map(|x| x.acc_rate)
            .max()
            .unwrap()) // 2579 too low
        // 2611 too low
        // 2653 too low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"),
            Solution::USIZE(1651));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(1991));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"),
            Solution::USIZE(1707));
    }

    // #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(0));
    }
}
