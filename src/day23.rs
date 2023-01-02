use std::collections::{hash_map::Entry, HashMap, HashSet};

type V2d = cgmath::Vector2<i32>;
const DIRS: [V2d; 8] = [
    V2d::new(-1, 0),
    V2d::new(1, 0),
    V2d::new(0, -1),
    V2d::new(0, 1),
    V2d::new(1, 1),
    V2d::new(1, -1),
    V2d::new(-1, 1),
    V2d::new(-1, -1),
];

const NORTH_DIRS: [V2d; 3] = [V2d::new(0, -1), V2d::new(1, -1), V2d::new(-1, -1)];
const EAST_DIRS: [V2d; 3] = [V2d::new(1, 0), V2d::new(1, 1), V2d::new(1, -1)];
const SOUTH_DIRS: [V2d; 3] = [V2d::new(0, 1), V2d::new(1, 1), V2d::new(-1, 1)];
const WEST_DIRS: [V2d; 3] = [V2d::new(-1, 0), V2d::new(-1, -1), V2d::new(-1, 1)];

enum Proposal {
    From(V2d),
    Crowded,
}

fn parse_elves(s: &str) -> HashSet<V2d> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .map(|(x, y)| V2d::new(x as i32, y as i32))
        .collect()
}
fn calc_empty_tiles_and_rounds(input: &str, max_rounds: usize) -> (i32, usize) {
    let mut elves = parse_elves(input);
    let check = [NORTH_DIRS, SOUTH_DIRS, WEST_DIRS, EAST_DIRS];
    let mut check_start = 0;
    let mut rounds = 0;
    while rounds < max_rounds {
        let mut moved = false;
        let mut proposals: HashMap<V2d, Proposal> = HashMap::new();
        // First part; gather proposals.
        for e in &elves {
            if DIRS.iter().all(|d| !elves.contains(&(e + d))) {
                continue;
            }
            for i in 0..4 {
                let dirs = check[(check_start + i) % check.len()];
                if dirs.iter().all(|d| !elves.contains(&(e + d))) {
                    match proposals.entry(e + dirs[0]) {
                        Entry::Occupied(mut entry) => {
                            entry.insert(Proposal::Crowded);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(Proposal::From(*e));
                        }
                    }
                    break;
                }
            }
        }
        // Second part; check proposals and move elves.
        for (target, proposal) in &proposals {
            if let Proposal::From(from) = proposal {
                moved = true;
                elves.remove(from);
                elves.insert(*target);
            }
        }
        rounds += 1;
        if !moved {
            break;
        }
        check_start = (check_start + 1) % check.len();
    }
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    elves.iter().for_each(|p| {
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
    });
    let empty_tiles = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;
    (empty_tiles, rounds)
}

fn part_1(s: &str) -> i32 {
    calc_empty_tiles_and_rounds(s, 10).0
}

fn part_2(s: &str) -> usize {
    calc_empty_tiles_and_rounds(s, usize::MAX).1
}

#[cfg(test)]
mod tests {
    use crate::day23::*;
    use std::fs;
    #[test]
    fn example23_part1() {
        let input = fs::read_to_string("input/example23").unwrap();
        assert_eq!(part_1(&input), 110);
    }
    #[test]
    fn day23_part1() {
        let input = fs::read_to_string("input/day23").unwrap();
        assert_eq!(part_1(&input), 3877);
    }
    #[test]
    fn example23_part2() {
        let input = fs::read_to_string("input/example23").unwrap();
        assert_eq!(part_2(&input), 20);
    }
    #[test]
    fn day23_part2() {
        let input = fs::read_to_string("input/day23").unwrap();
        assert_eq!(part_2(&input), 982);
    }
}
