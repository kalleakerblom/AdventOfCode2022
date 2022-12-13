use std::collections::{HashSet, VecDeque};

type Pos = (i32, i32);
struct Map {
    tiles: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
}
impl Map {
    fn from_str(s: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let tiles: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'S' => {
                            start = Some((x as i32, y as i32));
                            b'a'
                        }
                        b'E' => {
                            end = Some((x as i32, y as i32));
                            b'z'
                        }
                        b => b,
                    })
                    .collect()
            })
            .collect();
        Map {
            tiles,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
    fn height(&self, pos: Pos) -> Option<u8> {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.tiles[0].len() as i32
            || pos.1 >= self.tiles.len() as i32
        {
            return None;
        }
        Some(self.tiles[pos.1 as usize][pos.0 as usize])
    }
}
const DIRS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn get_to_visit(center: Pos, map: &Map, visited: &mut HashSet<Pos>, backwards: bool) -> Vec<Pos> {
    let mut to_visit = Vec::new();
    let h = map.height(center).unwrap();
    for dir in DIRS {
        let neighbor = (center.0 + dir.0, center.1 + dir.1);
        if visited.contains(&neighbor) {
            continue;
        }
        if let Some(next_height) = map.height(neighbor) {
            if !backwards {
                if h > next_height || next_height - h <= 1 {
                    to_visit.push(neighbor);
                    visited.insert(neighbor);
                }
            } else if next_height > h || h - next_height <= 1 {
                to_visit.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    to_visit
}

fn part_1(input: &str) -> u32 {
    let map = Map::from_str(input);
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((map.start, 0));
    while let Some((pos, steps)) = to_visit.pop_front() {
        if pos == map.end {
            return steps;
        }
        visited.insert(pos);
        let next = get_to_visit(pos, &map, &mut visited, false);
        to_visit.extend(next.iter().map(|n| (*n, steps + 1)));
    }
    panic!()
}
fn part_2(input: &str) -> u32 {
    let map = Map::from_str(input);
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((map.end, 0));
    while let Some((pos, steps)) = to_visit.pop_front() {
        if map.height(pos).unwrap() == b'a' {
            return steps;
        }
        visited.insert(pos);
        let next = get_to_visit(pos, &map, &mut visited, true);
        to_visit.extend(next.iter().map(|n| (*n, steps + 1)));
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;
    use std::fs;
    #[test]
    fn example12_day_part1() {
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        assert_eq!(part_1(input), 31);
    }
    #[test]
    fn day12_part1() {
        let input = fs::read_to_string("input/day12").unwrap();
        assert_eq!(part_1(&input), 370);
    }
    #[test]
    fn example12_part2() {
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        assert_eq!(part_2(input), 29);
    }
    #[test]
    fn day12_part2() {
        let input = fs::read_to_string("input/day12").unwrap();
        assert_eq!(part_2(&input), 363);
    }
}
