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
            north: (self.pos.0, self.pos.1 + self.radius + 1),
            east: (self.pos.0 + self.radius + 1, self.pos.1),
            south: (self.pos.0, self.pos.1 - (self.radius + 1)),
            west: (self.pos.0 - (self.radius + 1), self.pos.1),
            dx: 1,
            dy: -1,
            next: Some((self.pos.0, self.pos.1 + self.radius + 1)),
        }
    }
}
struct Perimeter {
    north: Pos,
    east: Pos,
    south: Pos,
    west: Pos,
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
                if out == self.east {
                    self.dx = -1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (-1, -1) => {
                if out == self.south {
                    self.dy = 1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (-1, 1) => {
                if out == self.west {
                    self.dx = 1;
                }
                Some((out.0 + self.dx, out.1 + self.dy))
            }
            (1, 1) => {
                if out == self.north {
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

fn part_1(input: &str, row: i32) -> u32 {
    let sensors = input.lines().map(Sensor::parse).collect_vec();
    (-10_000_000..10_000_000)
        .into_iter()
        .map(|x| sensors.iter().any(|s| s.in_range((x, row))) as u32)
        .sum()
}
fn part_2(input: &str) -> i64 {
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
