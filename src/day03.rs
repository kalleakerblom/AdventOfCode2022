use itertools::Itertools;
use std::collections::HashSet;
fn prio(b: u8) -> u32 {
    if b.is_ascii_uppercase() {
        (b - b'A') as u32 + 27
    } else {
        (b - b'a') as u32 + 1
    }
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let (first, second) = bytes.split_at(bytes.len() / 2);
            let set = HashSet::<u8>::from_iter(first.iter().cloned());
            let dup = second.iter().find(|b| set.contains(b)).unwrap();
            prio(*dup)
        })
        .sum()
}
pub fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for mut group in input.lines().chunks(3).into_iter() {
        let set_a = HashSet::<u8>::from_iter(group.next().unwrap().bytes());
        let set_b = HashSet::<u8>::from_iter(group.next().unwrap().bytes());
        let set_c = HashSet::<u8>::from_iter(group.next().unwrap().bytes());
        let intersect_a_b: HashSet<u8> = set_a.intersection(&set_b).cloned().collect();
        let mut intersect = intersect_a_b.intersection(&set_c);
        sum += prio(*intersect.next().unwrap());
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day03::*;
    use std::fs;
    #[test]
    fn example_day03_part1() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_1(&input), 157);
    }
    #[test]
    fn day03_part1() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_1(&input), 7863);
    }
    #[test]
    fn example03_part2() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_2(&input), 70);
    }
    #[test]
    fn day03_part2() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_2(&input), 2488);
    }
}
