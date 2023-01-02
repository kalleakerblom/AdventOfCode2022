enum Outcome {
    Win,
    Loss,
    Tie,
}
impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}
impl Rps {
    fn score(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
    fn from_outcome(other: &Self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Win => match other {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissors,
                Rps::Scissors => Rps::Rock,
            },
            Outcome::Loss => match other {
                Rps::Rock => Rps::Scissors,
                Rps::Paper => Rps::Rock,
                Rps::Scissors => Rps::Paper,
            },
            Outcome::Tie => other.clone(),
        }
    }
    fn play(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Tie;
        }
        match (&self, &other) {
            (Rps::Rock, Rps::Scissors) | (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) => {
                Outcome::Win
            }
            _ => Outcome::Loss,
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut score = 0;
    for l in input.lines() {
        let (other, me) = l.split_once(' ').unwrap();
        let (other, me) = (Rps::from_str(other), Rps::from_str(me));
        score += me.score() + me.play(&other).score();
    }
    score
}
pub fn part_2(input: &str) -> u32 {
    let mut score = 0;
    for l in input.lines() {
        let (other, outcome) = l.split_once(' ').unwrap();
        let (other, outcome) = (Rps::from_str(other), Outcome::from_str(outcome));
        let me = Rps::from_outcome(&other, &outcome);
        score += me.score() + outcome.score();
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::day02::*;
    use std::fs;
    #[test]
    fn day02_part1() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_1(&input), 12276);
    }
    #[test]
    fn day02_part2() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_2(&input), 9975);
    }
}
