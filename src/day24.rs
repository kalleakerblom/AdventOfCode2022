use std::collections::{HashMap, HashSet};
type V2d = cgmath::Vector2<i32>;

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

const N: V2d = V2d::new(0, 1);
const S: V2d = V2d::new(0, -1);
const E: V2d = V2d::new(1, 0);
const W: V2d = V2d::new(-1, 0);
const WAIT: V2d = V2d::new(0, 0);

#[derive(Clone)]
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
    fn pos_at_t(&self, t: i32, bounds: &Bounds) -> V2d {
        let x_bounds = bounds.x_bounds;
        let y_bounds = bounds.y_bounds;
        match self.dir {
            Dir::N => {
                let mut p = self.start + t * N;
                p.y = (p.y - y_bounds.0).rem_euclid(1 + y_bounds.1 - y_bounds.0) + y_bounds.0;
                p
            }
            Dir::S => {
                let mut p = self.start + t * S;
                p.y = (p.y - y_bounds.0).rem_euclid(1 + y_bounds.1 - y_bounds.0) + y_bounds.0;
                p
            }
            Dir::E => {
                let mut p = self.start + t * E;
                p.x = (p.x - x_bounds.0).rem_euclid(1 + x_bounds.1 - x_bounds.0) + x_bounds.0;
                p
            }
            Dir::W => {
                let mut p = self.start + t * W;
                p.x = (p.x - x_bounds.0).rem_euclid(1 + x_bounds.1 - x_bounds.0) + x_bounds.0;
                p
            }
        }
    }
}

#[derive(Debug)]
struct Bounds {
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
}

type WindsByCoordinate = HashMap<i32, Vec<Wind>>;
fn parse_winds_and_bounds(s: &str) -> (WindsByCoordinate, WindsByCoordinate, Bounds) {
    let mut winds_by_x = WindsByCoordinate::new();
    let mut winds_by_y = WindsByCoordinate::new();
    let height = s.lines().count();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = height as i32 - y as i32 - 1;
            let pos = V2d::new(x, y);
            match c {
                w @ '^' | w @ 'v' | w @ '>' | w @ '<' => {
                    let new_wind = Wind::new(pos, w);
                    winds_by_x.entry(x).or_default().push(new_wind.clone());
                    winds_by_y.entry(y).or_default().push(new_wind);
                }
                _ => (),
            }
        }
    }
    let width = s.lines().next().unwrap().chars().count();
    let bounds = Bounds {
        x_bounds: (1, width as i32 - 2),
        y_bounds: (1, height as i32 - 2),
    };
    (winds_by_x, winds_by_y, bounds)
}

struct Searcher {
    start: V2d,
    goal: V2d,
    winds_by_x: WindsByCoordinate,
    winds_by_y: WindsByCoordinate,
    bounds: Bounds,
}

impl Searcher {
    fn recursive_search(
        &self,
        pos: V2d,
        time: i32,
        visited: &mut HashSet<(V2d, i32)>,
        best: &mut i32,
        recursions: u32,
    ) {
        if recursions > 400 || !visited.insert((pos, time)) {
            return;
        }
        if pos == self.goal {
            if time < *best {
                *best = time;
            }
            return;
        }

        let x_range = self.bounds.x_bounds.0..=self.bounds.x_bounds.1;
        let y_range = self.bounds.y_bounds.0..=self.bounds.y_bounds.1;
        let in_bounds = |v: &V2d| x_range.contains(&v.x) && y_range.contains(&v.y);

        let next_moves = [N, S, W, E, WAIT]
            .iter()
            .map(|dir| pos + dir)
            .filter(move |next| {
                if !(in_bounds(next) || *next == self.start || *next == self.goal) {
                    return false;
                }
                let x_winds = self.winds_by_x.get(&next.x);
                let y_winds = self.winds_by_y.get(&next.y);
                let no_winds_here = |w: &[Wind]| {
                    w.iter()
                        .all(|w| w.pos_at_t(time + 1, &self.bounds) != *next)
                };
                match (x_winds, y_winds) {
                    (None, None) => true,
                    (None, Some(w)) => no_winds_here(w),
                    (Some(w), None) => no_winds_here(w),
                    (Some(wx), Some(wy)) => no_winds_here(wx) && no_winds_here(wy),
                }
            });
        for n in next_moves {
            self.recursive_search(n, time + 1, visited, best, recursions + 1);
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    let (winds_by_x, winds_by_y, bounds) = parse_winds_and_bounds(input);
    let goal = V2d::new(bounds.x_bounds.1, 0);
    let start = V2d::new(bounds.x_bounds.0, bounds.y_bounds.1 + 1);
    let mut best = i32::MAX;
    let mut visited = HashSet::new();
    let searcher = Searcher {
        start,
        goal,
        winds_by_x,
        winds_by_y,
        bounds,
    };
    searcher.recursive_search(start, 0, &mut visited, &mut best, 0);
    best
}

pub fn part_2(input: &str) -> i32 {
    let (winds_by_x, winds_by_y, bounds) = parse_winds_and_bounds(input);
    let goal = V2d::new(bounds.x_bounds.1, 0);
    let start = V2d::new(bounds.x_bounds.0, bounds.y_bounds.1 + 1);
    let mut best = i32::MAX;
    let mut visited = HashSet::new();
    let mut searcher = Searcher {
        start,
        goal,
        winds_by_x,
        winds_by_y,
        bounds,
    };
    searcher.recursive_search(start, 0, &mut visited, &mut best, 0);
    let time2 = best;
    best = i32::MAX;
    visited.clear();
    searcher.start = goal;
    searcher.goal = start;
    searcher.recursive_search(goal, time2, &mut visited, &mut best, 0);
    let time3 = best;
    best = i32::MAX;
    visited.clear();
    searcher.start = start;
    searcher.goal = goal;
    searcher.recursive_search(start, time3, &mut visited, &mut best, 0);
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
            &Bounds {
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
