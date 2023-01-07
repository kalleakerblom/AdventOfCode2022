use itertools::Itertools;

type Pos = (i32, i32);
struct Sensor {
    pos: Pos,
    beacon_pos: Pos,
    radius: i32,
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let (x, rest) = s
            .trim_start_matches("Sensor at x=")
            .split_once(',')
            .unwrap();
        let (y, rest) = rest.trim_start_matches(" y=").split_once(':').unwrap();
        let (bx, rest) = rest
            .trim_start_matches(" closest beacon is at x=")
            .split_once(',')
            .unwrap();
        let by = rest.trim_start_matches(" y=");
        let pos: Pos = (x.parse().unwrap(), y.parse().unwrap());
        let beacon_pos: Pos = (bx.parse().unwrap(), by.parse().unwrap());
        let radius = pos.0.abs_diff(beacon_pos.0) as i32 + pos.1.abs_diff(beacon_pos.1) as i32;
        Self {
            pos,
            radius,
            beacon_pos,
        }
    }
    fn in_range(&self, p: Pos) -> bool {
        if p == self.beacon_pos {
            return false;
        }
        self.pos.0.abs_diff(p.0) + self.pos.1.abs_diff(p.1) <= self.radius as u32
    }
    fn perimeter(&self) -> Perimeter {
        Perimeter {
            center: self.pos,
            radius: self.radius,
            dx: 1,
            dy: -1,
            next: Some((self.pos.0, self.pos.1 + self.radius + 1)),
        }
    }
    fn x_range(&self) -> (i32, i32) {
        (self.pos.0 - self.radius, self.pos.0 + self.radius)
    }
}
struct Perimeter {
    center: Pos,
    radius: i32,
    dx: i32,
    dy: i32,
    next: Option<Pos>,
}

impl Iterator for Perimeter {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.next?;
        self.next = match (self.dx, self.dy) {
            (1, -1) => {
                // 12 to 3
                if out.0 == self.center.0 + self.radius {
                    self.dx = -1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (-1, -1) => {
                // 3 to 6
                if out.1 == self.center.1 - self.radius {
                    self.dy = 1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (-1, 1) => {
                // 6 to 9
                if out.0 == self.center.0 - self.radius {
                    self.dx = 1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (1, 1) => {
                // 9 to 12
                if out.1 == self.center.1 + self.radius {
                    None
                } else {
                    Some((out.0 + self.dx, out.1 + self.dy))
                }
            }
            _ => panic!(),
        };
        Some(out)
    }
}

pub fn part_1(input: &str, row: i32) -> u32 {
    let sensors = input.lines().map(Sensor::parse).collect_vec();
    let (min_x, max_x) = sensors
        .iter()
        .map(Sensor::x_range)
        .fold((i32::MAX, i32::MIN), |acc, (min, max)| {
            (acc.0.min(min), acc.1.max(max))
        });
    (min_x..max_x)
        .into_iter()
        .map(|x| sensors.iter().any(|s| s.in_range((x, row))) as u32)
        .sum()
}
pub fn part_2(input: &str) -> i64 {
    let sensors = input.lines().map(Sensor::parse).collect_vec();
    let inside_box = |p: Pos| (0..=4_000_000).contains(&p.0) && (0..=4_000_000).contains(&p.1);
    let mut distress_pos = None;
    'outer: for i in 0..sensors.len() {
        let perimeter = sensors[i].perimeter();
        for p in perimeter {
            if !inside_box(p) {
                continue;
            }
            if sensors.iter().all(|s| !s.in_range(p)) {
                distress_pos = Some(p);
                break 'outer;
            }
        }
    }
    let (x, y) = distress_pos.unwrap();
    4000000 * x as i64 + y as i64
}

#[cfg(test)]
mod tests {
    use crate::day15::*;
    use std::fs;
    #[test]
    fn example15_day_part1() {
        let input = fs::read_to_string("input/example15").unwrap();
        assert_eq!(part_1(&input, 10), 26);
    }
    #[test]
    fn day15_part1() {
        let input = fs::read_to_string("input/day15").unwrap();
        assert_eq!(part_1(&input, 2_000_000), 5716881);
    }
    #[test]
    fn day15_part2() {
        let input = fs::read_to_string("input/day15").unwrap();
        assert_eq!(part_2(&input), 10852583132904);
    }
}
