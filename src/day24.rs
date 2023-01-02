use std::collections::{HashMap, HashSet};

use itertools::Itertools;
type V2d = cgmath::Vector2<i32>;
enum Dir {
    N,
    S,
    E,
    W,
}
const DIRS: [V2d; 5] = [
    V2d::new(0, 1),  //N
    V2d::new(0, -1), //S
    V2d::new(1, 0),  //E
    V2d::new(-1, 0), //W
    V2d::new(0, 0),  //Wait
];
struct Wind {
    start: V2d,
    dir: Dir,
}

impl Wind {
    fn new(start: V2d, dir_char: char) -> Self {
        let dir = match dir_char {
            '^' => Dir::N,
            'v' => Dir::S,
            '>' => Dir::E,
            '<' => Dir::W,
            _ => panic!(),
        };
        Self { start, dir }
    }
    fn pos_at_t(&self, t: i32, bounds: &WindBounds) -> V2d {
        let x_bounds = bounds.x_bounds;
        let y_bounds = bounds.y_bounds;
        match self.dir {
            Dir::N => {
                let mut p = self.start + t * DIRS[0];
                p.y = (p.y - y_bounds.0).rem_euclid(1 + y_bounds.1 - y_bounds.0) + y_bounds.0;
                p
            }
            Dir::S => {
                let mut p = self.start + t * DIRS[1];
                p.y = (p.y - y_bounds.0).rem_euclid(1 + y_bounds.1 - y_bounds.0) + y_bounds.0;
                p
            }
            Dir::E => {
                let mut p = self.start + t * DIRS[2];
                p.x = (p.x - x_bounds.0).rem_euclid(1 + x_bounds.1 - x_bounds.0) + x_bounds.0;
                p
            }
            Dir::W => {
                let mut p = self.start + t * DIRS[3];
                p.x = (p.x - x_bounds.0).rem_euclid(1 + x_bounds.1 - x_bounds.0) + x_bounds.0;
                p
            }
        }
    }
}

enum Tile {
    Floor,
    Wall,
}
#[derive(Debug)]
struct WindBounds {
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
}
fn parse_tiles_and_winds_and_bounds(s: &str) -> (HashMap<V2d, Tile>, Vec<Wind>, WindBounds) {
    let mut tiles = HashMap::new();
    let mut winds = Vec::new();
    let height = s.lines().count();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = height as i32 - y as i32 - 1;
            let pos = V2d::new(x, y);
            match c {
                '.' => {
                    tiles.insert(pos, Tile::Floor);
                }
                '#' => {
                    tiles.insert(pos, Tile::Wall);
                }
                w @ '^' | w @ 'v' | w @ '>' | w @ '<' => {
                    tiles.insert(pos, Tile::Floor);
                    winds.push(Wind::new(pos, w));
                }
                _ => panic!(),
            }
        }
    }
    let width = s.lines().next().unwrap().chars().count();
    let wind_bounds = WindBounds {
        x_bounds: (1, width as i32 - 2),
        y_bounds: (1, height as i32 - 2),
    };
    (tiles, winds, wind_bounds)
}

#[derive(PartialEq, Eq)]
struct State {
    priority: i32,
    pos: V2d,
    time: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
            .then_with(|| self.time.cmp(&other.time))
    }
}
const LIMIT: u32 = 400;
fn recursive_search(
    pos: V2d,
    goal: &V2d,
    time: i32,
    tiles: &HashMap<V2d, Tile>,
    winds: &[Wind],
    bounds: &WindBounds,
    visited: &mut HashSet<(V2d, i32)>,
    best: &mut i32,
    recursions: u32,
) {
    if recursions > LIMIT || !visited.insert((pos, time)) {
        return;
    }
    if pos == *goal {
        if time < *best {
            *best = time;
        }
        return;
    }

    let mut neighbors = DIRS
        .iter()
        .map(|d| pos + d)
        .filter(|n| matches!(tiles.get(n), Some(Tile::Floor)))
        .filter(move |n| winds.iter().all(|w| w.pos_at_t(time + 1, bounds) != *n))
        .collect_vec();
    neighbors.sort_by_key(|n| n.x.abs_diff(goal.x) + n.y.abs_diff(goal.y));
    for n in neighbors {
        recursive_search(
            n,
            goal,
            time + 1,
            tiles,
            winds,
            bounds,
            visited,
            best,
            recursions + 1,
        );
    }
}

pub fn part_1(input: &str) -> i32 {
    let (tiles, winds, bounds) = parse_tiles_and_winds_and_bounds(input);
    let goal = V2d::new(bounds.x_bounds.1, 0);
    let start = V2d::new(bounds.x_bounds.0, bounds.y_bounds.1 + 1);
    let mut best = i32::MAX;
    let mut visited = HashSet::new();
    recursive_search(
        start,
        &goal,
        0,
        &tiles,
        &winds,
        &bounds,
        &mut visited,
        &mut best,
        0,
    );
    best
}
pub fn part_2(input: &str) -> i32 {
    let (tiles, winds, bounds) = parse_tiles_and_winds_and_bounds(input);
    let goal = V2d::new(bounds.x_bounds.1, 0);
    let start = V2d::new(bounds.x_bounds.0, bounds.y_bounds.1 + 1);
    let mut best = i32::MAX;
    let mut visited = HashSet::new();
    recursive_search(
        start,
        &goal,
        0,
        &tiles,
        &winds,
        &bounds,
        &mut visited,
        &mut best,
        0,
    );
    let time2 = best;
    best = i32::MAX;
    visited.clear();
    recursive_search(
        goal,
        &start,
        time2,
        &tiles,
        &winds,
        &bounds,
        &mut visited,
        &mut best,
        0,
    );
    let time3 = best;
    best = i32::MAX;
    visited.clear();
    recursive_search(
        start,
        &goal,
        time3,
        &tiles,
        &winds,
        &bounds,
        &mut visited,
        &mut best,
        0,
    );
    best
}

#[cfg(test)]
mod tests {
    use crate::day24::*;
    use std::fs;
    #[test]
    fn winds() {
        let w = Wind::new(V2d::new(1, 1), '^');
        dbg!(w.pos_at_t(
            9,
            &WindBounds {
                x_bounds: (1, 4),
                y_bounds: (1, 4)
            }
        ));
    }
    #[test]
    fn example24_part1() {
        let input = fs::read_to_string("input/example24").unwrap();
        assert_eq!(part_1(&input), 18);
    }
    #[test]
    fn day24_part1() {
        let input = fs::read_to_string("input/day24").unwrap();
        assert_eq!(part_1(&input), 264);
    }
    #[test]
    fn example24_part2() {
        let input = fs::read_to_string("input/example24").unwrap();
        assert_eq!(part_2(&input), 54);
    }
    #[test]
    fn day24_part2() {
        let input = fs::read_to_string("input/day24").unwrap();
        assert_eq!(part_2(&input), 789);
    }
}
