fn block_sums(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.split("\n\n").map(|block| {
        block
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .sum::<i32>()
    })
}

fn part_1(input: &str) -> i32 {
    block_sums(input).max().unwrap()
}

fn part_2(input: &str) -> i32 {
    let mut calorie_sums: Vec<_> = block_sums(input).collect();
    let len = calorie_sums.len();
    calorie_sums.select_nth_unstable(len - 3);
    calorie_sums[(len - 3)..].iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::day01::*;
    use std::fs;

    #[test]
    fn day01_part1() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_1(&input), 69177);
    }
    #[test]
    fn example01_part2() {
        let input = fs::read_to_string("input/example01").unwrap();
        assert_eq!(part_2(&input), 45000);
    }
    #[test]
    fn day01_part2() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_2(&input), 207456);
    }
}
