use std::ops::RangeInclusive;
fn range(s: &str) -> RangeInclusive<u32> {
    let (start, end) = s.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn part_1(input: &str) -> u32 {
    let mut count = 0;
    for l in input.lines() {
        let (a, b) = l.split_once(',').unwrap();
        let range_a = range(a);
        let range_b = range(b);
        if (range_a.contains(range_b.start()) && range_a.contains(range_b.end()))
            || (range_b.contains(range_a.start()) && range_b.contains(range_a.end()))
        {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> u32 {
    let mut count = 0;
    for l in input.lines() {
        let (a, b) = l.split_once(',').unwrap();
        let range_a = range(a);
        let range_b = range(b);
        if range_a.contains(range_b.start())
            || range_a.contains(range_b.end())
            || range_b.contains(range_a.start())
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day04::*;
    use std::fs;
    #[test]
    fn example04_day_part1() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_1(&input), 2);
    }
    #[test]
    fn day04_part1() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_1(&input), 534);
    }
    #[test]
    fn example04_part2() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_2(&input), 4);
    }
    #[test]
    fn day04_part2() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_2(&input), 841);
    }
}
