use crate::Solution;
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sensor {
    x : isize,
    y : isize,
    beacon_x : isize,
    beacon_y : isize,
    dist : isize,
}
impl Sensor {
    pub fn new(input: &'static str) -> Self {
        let mut tok = input.split("=");
        tok.next().expect("expect \'Sensor at...\'");
        let (x, y, beacon_x, beacon_y) = tok
            .map(|x| x.split(&[',', ':'][..]).next().unwrap().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let dist = (x - beacon_x).abs() + (y - beacon_y).abs();
        Self {
            x,
            y,
            beacon_x,
            beacon_y,
            dist,
        }
    }

    pub fn no_beacon_range(&self, line_y : isize) -> Option<(isize, isize)> {
        let offset_y = (self.y - line_y).abs();

        if offset_y > self.dist {
            None
        } else {
            let remain = self.dist - offset_y;
            let min_x = self.x - remain;
            let max_x = self.x + remain;
            Some((min_x, max_x))
        }
    }

    // return (start_x, start_y, dir_x, dir_y) to make entire round
    pub fn round(&self) -> [(isize, isize, isize, isize); 4] {
        [((self.x - self.dist - 1, self.y, 1, -1)),
         ((self.x, self.y - self.dist - 1, 1, 1)),
         ((self.x + self.dist + 1, self.y, -1, -1)),
         ((self.x, self.y + self.dist + 1, -1, 1))]
    }

    pub fn can_overap(&self, other : &Self) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() <= self.dist + other.dist
    }
    pub fn dist_to_edge(&self, x : isize, y : isize) -> isize {
        (self.x - x).abs() + (self.y - y).abs() - self.dist
    }

}

#[test]
fn test_sensor() {
    assert_eq!(Sensor::new("Sensor at x=-2, y=-1: closest beacon is at x=1, y=2"),
        Sensor {
            x : -2,
            y : -1,
            beacon_x : 1,
            beacon_y : 2,
            dist : 6,
        });
    assert_eq!(Sensor::new("Sensor at x=0, y=0: closest beacon is at x=10, y=0")
        .no_beacon_range(0), Some((-10, 10)));
    assert_eq!(Sensor::new("Sensor at x=0, y=0: closest beacon is at x=10, y=0")
        .no_beacon_range(20), None);
    assert_eq!(Sensor::new("Sensor at x=0, y=0: closest beacon is at x=10, y=0")
        .round(), [(-11, 0, 1, -1),
                   (0, -11, 1, 1),
                   (11, 0, -1, -1),
                   (0, 11, -1, 1)]);
    assert_eq!(Sensor::new("Sensor at x=0, y=0: closest beacon is at x=10, y=0")
        .dist_to_edge(1, 1), -8);
    assert_eq!(Sensor::new("Sensor at x=0, y=0: closest beacon is at x=10, y=0")
        .dist_to_edge(7, 7), 4);
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    do_solve(part, input, 2_000_000, 4_000_000)
}

pub fn do_solve(part: u8, input: &'static str, y_line : isize, search_lim : isize) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/15.input")
    };

    if part == 1 {
        // get ranges
        let ranges = input
            .lines()
            .map(|x| Sensor::new(x))
            .filter_map(|x| x.no_beacon_range(y_line))
            .collect::<Vec<_>>();
        let mut steps = ranges
            .iter()
            .fold(FxHashSet::default(), |mut acc, r| {
                acc.insert(r.0);
                acc.insert(r.1);
                acc });
        let mut steps = steps
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        steps.sort();

        // get unique covered sub-ranges
        let mut covered = Vec::default();
        for (step_l, step_r) in steps.iter().tuple_windows() {
            if ranges.iter().any(|(min_x, max_x)| min_x <= step_l && step_r <= max_x) {
                covered.push((step_l, step_r));
            }
        }

        // get covered lenght
        let mut last_pos = None;
        let mut covered_sum = 0;
        for (step_l, step_r) in &covered {
            covered_sum += *step_r - *step_l + 1;
            if let Some(last_pos) = last_pos {
                if last_pos == *step_l {
                    covered_sum -= 1; // XXX double count of border
                }
            }
            last_pos = Some(*step_r);
        }

        // remove existing *unique* beacons on the line (text is vague as usual…)
        let beacons_in_line = input
            .lines()
            .map(|x| Sensor::new(x))
            .filter_map(|x|
                if x.beacon_y == y_line
                && covered.iter().any(|(step_l, step_r)| **step_l <= x.beacon_x && x.beacon_x <= **step_r)
                {
                    Some(x.beacon_x)
                } else {
                    None
                })
            .fold(FxHashSet::default(), |mut acc, r| { acc.insert(r); acc });
        covered_sum -= beacons_in_line.len() as isize;

        Solution::ISIZE(covered_sum)
    } else {
        let sensors = input
            .lines()
            .map(|x| Sensor::new(x))
            .collect::<Vec<_>>();

        for (sidx, sensor) in sensors.iter().enumerate() {
            let arc = sensor.dist + 2;
            let possible_sensors = sensors.iter().enumerate()
                .filter(|(idx, _)| *idx != sidx)
                .filter_map(|(_, sensor)| if sensor.can_overap(sensor) {
                    Some(sensor)
                } else {
                    None
                })
                .collect::<Vec<_>>();
            for (start_x, start_y, dx, dy) in &sensor.round() {
                let (mut x, mut y) = (*start_x, *start_y);
                let mut step = sensor.dist + 2;
                while step > 0 {
                    if (x < 0 || x > search_lim || y < 0 || y > search_lim) {
                        break; // too brutal
                    }
                    let gap = possible_sensors.iter().map(|sensor| sensor.dist_to_edge(x, y)).min().unwrap();
                    if gap == 1 {
                        return Solution::ISIZE(x * 4_000_000 + y)
                    }
                    let gap = if gap < -1 { -gap / 2 } else { 1 };
                    x += dx * gap;
                    y += dy * gap;
                    step -= gap;
                }
            }
        }
        panic!("no beacon found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(do_solve(1, r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3", 10, 20), Solution::ISIZE(26));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::ISIZE(4951427));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(do_solve(2, r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3", 10, 20), Solution::ISIZE(56000011));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::ISIZE(13029714573243));
    }
}
