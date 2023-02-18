struct Move {
    count: usize,
    from: usize,
    to: usize,
}
impl Move {
    fn parse(s: &str) -> Self {
        let (count, rest) = s.trim_start_matches("move ").split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        Self {
            count: count.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        }
    }
}

/// Reads "_modified" input files
fn parse_stacks_and_moves(s: &str) -> (Vec<Vec<u8>>, Vec<Move>) {
    let (stacks, moves) = s.split_once("\n\n").unwrap();
    let stacks = stacks
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.as_bytes()[0]).collect())
        .collect();
    let moves = moves.lines().map(Move::parse).collect();
    (stacks, moves)
}

pub fn part_1(input: &str) -> String {
    let (mut stacks, moves) = parse_stacks_and_moves(input);
    for m in moves {
        for _ in 0..m.count {
            let pop = stacks[m.from].pop().unwrap();
            stacks[m.to].push(pop);
        }
    }
    let ans: Vec<u8> = stacks.iter().map(|s| s.last().unwrap()).cloned().collect();
    String::from_utf8(ans).unwrap()
}

use std::mem;
pub fn part_2(input: &str) -> String {
    let (mut stacks, moves) = parse_stacks_and_moves(input);
    for m in moves {
        // Yikes Rust, two mut ref is hard. v3 with mem:swap
        let mut to = Vec::new();
        let mut from = Vec::new();
        mem::swap(stacks.get_mut(m.to).unwrap(), &mut to);
        mem::swap(stacks.get_mut(m.from).unwrap(), &mut from);
        to.extend_from_slice(&from[from.len() - m.count..]);
        from.truncate(from.len() - m.count);
        mem::swap(stacks.get_mut(m.to).unwrap(), &mut to);
        mem::swap(stacks.get_mut(m.from).unwrap(), &mut from);
    }
    let ans: Vec<u8> = stacks.iter().map(|s| s.last().unwrap()).cloned().collect();
    String::from_utf8(ans).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day05::*;
    use std::fs;
    #[test]
    fn example05_day_part1() {
        let input = fs::read_to_string("input/example05_modified").unwrap();
        assert_eq!(part_1(&input), "CMZ");
    }
    #[test]
    fn day05_part1() {
        let input = fs::read_to_string("input/day05_modified").unwrap();
        assert_eq!(part_1(&input), "VWLCWGSDQ");
    }
    #[test]
    fn example05_day_part2() {
        let input = fs::read_to_string("input/example05_modified").unwrap();
        assert_eq!(part_2(&input), "MCD");
    }
    #[test]
    fn day05_part2() {
        let input = fs::read_to_string("input/day05_modified").unwrap();
        assert_eq!(part_2(&input), "TCGLQSLPW");
    }
}
