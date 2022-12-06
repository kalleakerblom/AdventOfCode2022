use std::collections::HashSet;

fn part_1(input: &str) -> usize {
    let unique = |a, b, c, d| a != b && a != c && a != d && b != c && b != d && c != d;
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, win)| unique(win[0], win[1], win[2], win[3]))
        .unwrap()
        .0
        + 4
}

fn part_2(input: &str) -> usize {
    let unique = |win: &[u8]| {
        let mut set = HashSet::with_capacity(14);
        win.iter().all(|byte| set.insert(*byte))
    };
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, win)| unique(win))
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod tests {
    use crate::day06::*;
    use std::fs;
    #[test]
    fn example06_day_part1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_1(input), 7);
    }
    #[test]
    fn day06_part1() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_1(&input), 1876);
    }
    #[test]
    fn example06_part2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_2(input), 19);
    }
    #[test]
    fn day06_part2() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_2(&input), 2202);
    }
}
