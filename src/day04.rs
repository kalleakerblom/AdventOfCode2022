use std::ops::RangeInclusive;
fn parse_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let parse_range = |s: &str| {
        s.split_once('-')
            .map(|(s, e)| s.parse().unwrap()..=e.parse().unwrap())
            .unwrap()
    };
    line.split_once(',')
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        .unwrap()
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_ranges)
        .map(|(range_a, range_b)| {
            let contained = (range_a.contains(range_b.start()) && range_a.contains(range_b.end()))
                || (range_b.contains(range_a.start()) && range_b.contains(range_a.end()));
            u32::from(contained)
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_ranges)
        .map(|(range_a, range_b)| {
            let overlapping = range_a.contains(range_b.start())
                || range_a.contains(range_b.end())
                || range_b.contains(range_a.start());
            u32::from(overlapping)
        })
        .sum()
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
