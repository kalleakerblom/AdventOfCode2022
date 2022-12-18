use cgmath::Vector3;
use std::collections::{HashSet, VecDeque};
type V3d = Vector3<i32>;
type CellSet = HashSet<V3d>;
type Bounds = (V3d, V3d);
fn read_cells(s: &str) -> (CellSet, Bounds) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    let cell_set = s
        .lines()
        .map(|l: &str| {
            let mut coords = l.split(',').map(|n| n.parse::<i32>().unwrap());
            let (x, y, z) = (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            );
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
            V3d::new(x, y, z)
        })
        .collect();
    // NOTE: Did not work with padding +1/-1 for bounds, why?
    let min = V3d::new(min_x - 2, min_y - 2, min_z - 2);
    let max = V3d::new(max_x + 2, max_y + 2, max_z + 2);
    (cell_set, (min, max))
}

const DIRS: [V3d; 6] = [
    V3d::new(1, 0, 0),
    V3d::new(-1, 0, 0),
    V3d::new(0, 1, 0),
    V3d::new(0, -1, 0),
    V3d::new(0, 0, 1),
    V3d::new(0, 0, -1),
];

fn surface_area(cells: &CellSet) -> u32 {
    let cell_surface = |c| {
        DIRS.iter()
            .map(|d| !cells.contains(&(c + d)) as u32)
            .sum::<u32>()
    };
    cells.iter().map(cell_surface).sum()
}

fn get_outside_cells(start: V3d, (min, max): (V3d, V3d), lava_cells: &CellSet) -> CellSet {
    let in_bounds = |p: V3d| {
        (min.x..max.x).contains(&p.x)
            && (min.y..max.y).contains(&p.y)
            && (min.z..max.z).contains(&p.z)
    };
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    visited.insert(start);
    while let Some(next) = to_visit.pop_front() {
        for d in DIRS {
            let neighbor = next + d;
            if lava_cells.contains(&neighbor) || !in_bounds(neighbor) {
                continue;
            }
            if visited.insert(neighbor) {
                to_visit.push_back(neighbor);
            }
        }
    }
    visited
}

fn surface_area_part2(lava_cells: &CellSet, outside_cells: &CellSet) -> u32 {
    let cell_surface = |c| {
        DIRS.iter()
            .map(|d| (!lava_cells.contains(&(c + d)) && outside_cells.contains(&(c + d))) as u32)
            .sum::<u32>()
    };
    lava_cells.iter().map(cell_surface).sum()
}

fn part_1(input: &str) -> u32 {
    let (cell_set, _) = read_cells(input);
    surface_area(&cell_set)
}
fn part_2(input: &str) -> u32 {
    let (lava_cells, bounds) = read_cells(input);
    let outside_cells = get_outside_cells(bounds.0, bounds, &lava_cells);
    surface_area_part2(&lava_cells, &outside_cells)
}

#[cfg(test)]
mod tests {
    use crate::day18::*;
    use std::fs;
    #[test]
    fn example18_part1() {
        let input = fs::read_to_string("input/example18").unwrap();
        assert_eq!(part_1(&input), 64);
    }
    #[test]
    fn day18_part1() {
        let input = fs::read_to_string("input/day18").unwrap();
        assert_eq!(part_1(&input), 4628);
    }
    #[test]
    fn example18_part2() {
        let input = fs::read_to_string("input/example18").unwrap();
        assert_eq!(part_2(&input), 58);
    }
    #[test]
    fn day18_part2() {
        let input = fs::read_to_string("input/day18").unwrap();
        assert_eq!(part_2(&input), 2582);
    }
}
