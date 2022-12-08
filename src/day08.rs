use std::{cmp, collections::HashSet};

fn read_map(s: &str) -> Vec<Vec<i8>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect()
}
fn part_1(input: &str) -> usize {
    let map = read_map(input);
    // find hidden from west & east
    let mut hidden_from_east = HashSet::new();
    let mut hidden_from_west = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        let mut tallest_from_west = -1;
        row.iter().enumerate().for_each(|(x, next)| {
            if next > &tallest_from_west {
                tallest_from_west = *next;
            } else {
                hidden_from_west.insert((x, y));
            }
        });
        let mut tallest_from_east = -1;
        row.iter().enumerate().rev().for_each(|(x, next)| {
            if next > &tallest_from_east {
                tallest_from_east = *next;
            } else {
                hidden_from_east.insert((x, y));
            }
        });
    }
    // find hidden from north & south
    let width = map[0].len();
    let height = map.len();
    let mut hidden_from_north = HashSet::new();
    let mut hidden_from_south = HashSet::new();
    for x in 0..width {
        let mut tallest_from_north = -1;
        (0..height).for_each(|y| {
            if map[y][x] > tallest_from_north {
                tallest_from_north = map[y][x];
            } else {
                hidden_from_north.insert((x, y));
            }
        });
        let mut tallest_from_south = -1;
        (0..height).rev().for_each(|y| {
            if map[y][x] > tallest_from_south {
                tallest_from_south = map[y][x];
            } else {
                hidden_from_south.insert((x, y));
            }
        });
    }
    let hidden_ew: HashSet<_> = hidden_from_east.intersection(&hidden_from_west).collect();
    let hidden_ns: HashSet<_> = hidden_from_north.intersection(&hidden_from_south).collect();
    let hidden = hidden_ew.intersection(&hidden_ns).count();
    height * width - hidden
}
fn part_2(input: &str) -> u32 {
    let map = read_map(input);
    let mut best_score = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let house = map[y][x];
            let mut score_n = 0;
            for n in (0..y).rev() {
                if house <= map[n][x] {
                    score_n += 1;
                    break;
                }
                score_n += 1;
            }
            let mut score_s = 0;
            for s in y + 1..map.len() {
                if house <= map[s][x] {
                    score_s += 1;
                    break;
                }
                score_s += 1;
            }
            let mut score_w = 0;
            for w in (0..x).rev() {
                if house <= map[y][w] {
                    score_w += 1;
                    break;
                }
                score_w += 1;
            }
            let mut score_e = 0;
            for e in (x + 1)..map[0].len() {
                if house <= map[y][e] {
                    score_e += 1;
                    break;
                }
                score_e += 1;
            }
            best_score = cmp::max(score_e * score_n * score_s * score_w, best_score);
        }
    }
    best_score as u32
}

#[cfg(test)]
mod tests {
    use crate::day08::*;
    use std::fs;
    #[test]
    fn example08_day_part1() {
        let input = fs::read_to_string("input/example08").unwrap();
        assert_eq!(part_1(&input), 21);
    }
    #[test]
    fn day08_part1() {
        let input = fs::read_to_string("input/day08").unwrap();
        assert_eq!(part_1(&input), 1820);
    }
    #[test]
    fn example08_part2() {
        let input = fs::read_to_string("input/example08").unwrap();
        assert_eq!(part_2(&input), 8);
    }
    #[test]
    fn day08_part2() {
        let input = fs::read_to_string("input/day08").unwrap();
        assert_eq!(part_2(&input), 385112);
    }
}
