use std::collections::HashSet;
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Knot(i32, i32);
impl Knot {
    fn move_head(&mut self, dir: &str) {
        match dir {
            "U" => self.1 += 1,
            "D" => self.1 -= 1,
            "R" => self.0 += 1,
            "L" => self.0 -= 1,
            _ => panic!(),
        }
    }
    fn move_tail(&mut self, head: &Knot) {
        let dx = head.0 - self.0;
        let dy = head.1 - self.1;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.0 += dx.signum();
            self.1 += dy.signum();
        }
    }
}
pub fn part_1(input: &str) -> usize {
    let mut head = Knot(0, 0);
    let mut tail = Knot(0, 0);
    let mut visited = HashSet::<Knot>::new();
    visited.insert(tail);
    for (dir, n) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let n: usize = n.parse().unwrap();
        for _ in 0..n {
            head.move_head(dir);
            tail.move_tail(&head);
            visited.insert(tail);
        }
    }
    visited.len()
}
pub fn part_2(input: &str) -> usize {
    let mut knots = [Knot(0, 0); 10];
    let mut visited = HashSet::<Knot>::new();
    visited.insert(knots[9]);
    for (dir, n) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let n: usize = n.parse().unwrap();
        for _ in 0..n {
            knots[0].move_head(dir);
            for i in 1..10 {
                let head = knots[i - 1];
                knots[i].move_tail(&head);
            }
            visited.insert(knots[9]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use crate::day09::*;
    use std::fs;
    #[test]
    fn example09_day_part1() {
        let input = fs::read_to_string("input/example09").unwrap();
        assert_eq!(part_1(&input), 13);
    }
    #[test]
    fn day09_part1() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_1(&input), 6284);
    }
    #[test]
    fn example09_part2() {
        let input = fs::read_to_string("input/example09_part2").unwrap();
        assert_eq!(part_2(&input), 36);
    }
    #[test]
    fn day09_part2() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_2(&input), 2661);
    }
}
