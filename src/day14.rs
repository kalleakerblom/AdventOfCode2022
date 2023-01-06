use std::{cmp, collections::HashSet, iter};
enum Part {
    One,
    Two,
}
struct SandMap {
    source: (i32, i32),
    map: HashSet<(i32, i32)>,
    bottom: i32,
    part: Part,
}
impl SandMap {
    fn from_str(s: &str, part: Part) -> Self {
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        let mut bottom = -1;
        for l in s.lines() {
            let mut points = l.split(" -> ").map(|p: &str| {
                p.split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap()
            });
            let mut prev: (i32, i32) = points.next().unwrap();
            bottom = cmp::max(bottom, prev.1);
            for next in points {
                bottom = cmp::max(bottom, next.1);
                let dx = (next.0 - prev.0).signum();
                let dy = (next.1 - prev.1).signum();
                let mut p = prev;
                loop {
                    map.insert(p);
                    if p == next {
                        break;
                    }
                    p = (p.0 + dx, p.1 + dy);
                }
                prev = next;
            }
        }
        Self {
            source: (500, 0),
            map,
            bottom,
            part,
        }
    }

    fn step(&mut self) -> bool {
        let mut p = self.source;
        loop {
            if matches!(self.part, Part::One) && p.1 > self.bottom {
                // fell off the map
                break false;
            } else if p.1 + 1 == self.bottom + 2 {
                // fell to inf floor
                self.map.insert(p);
                break true;
            }
            if let Some(fall) = [0, -1, 1]
                .iter()
                .map(|dx| (p.0 + dx, p.1 + 1))
                .find(|fall| !self.map.contains(fall))
            {
                p = fall;
            } else {
                // at rest
                self.map.insert(p);
                break p != self.source;
            }
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut sand_map = SandMap::from_str(input, Part::One);
    iter::from_fn(|| sand_map.step().then_some(())).count()
}

pub fn part_2(input: &str) -> usize {
    let mut sand_map = SandMap::from_str(input, Part::Two);
    let sand = iter::from_fn(|| sand_map.step().then_some(())).count();
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
