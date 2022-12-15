use std::{
    cmp,
    collections::{HashMap, HashSet},
};

struct SandMap {
    source: (i32, i32),
    map: HashSet<(i32, i32)>,
    bottom: i32,
}

impl SandMap {
    fn from_str(s: &str) -> Self {
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        let mut bottom = -1;
        for l in s.lines() {
            let mut points = l.split(" -> ");
            let coords = |p: &str| {
                p.split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap()
            };
            let mut prev: (i32, i32) = coords(points.next().unwrap());
            bottom = cmp::max(bottom, prev.1);
            for next in points {
                let next = coords(next);
                bottom = cmp::max(bottom, next.1);
                let sig_x = (next.0 - prev.0).signum();
                if sig_x != 0 {
                    let mut x = prev.0;
                    while x != next.0 {
                        map.insert((x, next.1));
                        x += sig_x;
                    }
                    map.insert(next);
                } else {
                    let sig_y = (next.1 - prev.1).signum();
                    let mut y = prev.1;
                    while y != next.1 {
                        map.insert((next.0, y));
                        y += sig_y;
                    }
                }
                map.insert(next);
                prev = next;
            }
        }
        Self {
            source: (500, 0),
            map,
            bottom,
        }
    }
    fn step(&mut self) -> bool {
        let mut p = self.source;
        loop {
            if p.1 > self.bottom {
                break false;
            }
            if !self.map.contains(&(p.0, p.1 + 1)) {
                p = (p.0, p.1 + 1);
                continue;
            }
            if !self.map.contains(&(p.0 - 1, p.1 + 1)) {
                p = (p.0 - 1, p.1 + 1);
                continue;
            }
            if !self.map.contains(&(p.0 + 1, p.1 + 1)) {
                p = (p.0 + 1, p.1 + 1);
                continue;
            }
            // rest
            self.map.insert(p);
            break true;
        }
    }
    fn step_part2(&mut self) -> bool {
        let mut p = self.source;
        loop {
            if p.1 + 1 == self.bottom + 2 {
                self.map.insert(p);
                break true;
            }
            if !self.map.contains(&(p.0, p.1 + 1)) {
                p = (p.0, p.1 + 1);
                continue;
            }
            if !self.map.contains(&(p.0 - 1, p.1 + 1)) {
                p = (p.0 - 1, p.1 + 1);
                continue;
            }
            if !self.map.contains(&(p.0 + 1, p.1 + 1)) {
                p = (p.0 + 1, p.1 + 1);
                continue;
            }
            // rest
            self.map.insert(p);
            break p != self.source;
        }
    }
}

fn part_1(input: &str) -> u32 {
    let mut sand_map = SandMap::from_str(input);
    let mut sand = 0;
    while sand_map.step() {
        sand += 1;
    }
    sand
}
fn part_2(input: &str) -> u32 {
    let mut sand_map = SandMap::from_str(input);
    let mut sand = 0;
    while sand_map.step_part2() {
        sand += 1;
    }
    sand + 1
}

#[cfg(test)]
mod tests {
    use crate::day14::*;
    use std::fs;
    #[test]
    fn example14_day_part1() {
        let input = fs::read_to_string("input/example14").unwrap();
        assert_eq!(part_1(&input), 24);
    }
    #[test]
    fn day14_part1() {
        let input = fs::read_to_string("input/day14").unwrap();
        assert_eq!(part_1(&input), 757);
    }
    #[test]
    fn example14_part2() {
        let input = fs::read_to_string("input/example14").unwrap();
        assert_eq!(part_2(&input), 93);
    }
    #[test]
    fn day14_part2() {
        let input = fs::read_to_string("input/day14").unwrap();
        assert_eq!(part_2(&input), 24943);
    }
}
