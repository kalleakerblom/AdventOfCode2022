fn snafu_to_decimal(snafu: &[char]) -> i64 {
    snafu
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => 0,
            '1' => 5i64.pow(i as u32),
            '2' => 2 * 5i64.pow(i as u32),
            '-' => -(5i64.pow(i as u32)),
            '=' => -2 * 5i64.pow(i as u32),
            _ => panic!(),
        })
        .sum()
}
const SNAFU: [(i64, char); 5] = [(-2, '='), (-1, '-'), (0, '0'), (1, '1'), (2, '2')];
fn decimal_to_snafu(i: i64) -> Vec<char> {
    let mut current = 0;
    let mut res = vec![];

    for exp in 0u32.. {
        if current > i {
            break;
        }
        current += 2 * 5i64.pow(exp);
        res.push('2');
    }
    let len = res.len() as u32;
    for (exp, c) in res.iter_mut().enumerate() {
        let term = 5i64.pow(len - 1 - exp as u32);
        for (factor, symbol) in &SNAFU[..4] {
            if (current - (2 - factor) * term) >= i {
                current -= (2 - factor) * term;
                *c = *symbol;
                break;
            }
        }
    }
    res
}

pub fn part_1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(|l| {
            let snafu: Vec<_> = l.chars().collect();
            snafu_to_decimal(&snafu)
        })
        .sum();
    decimal_to_snafu(sum).iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::day25::*;
    use std::fs;
    #[test]
    fn sna_to_dec() {
        assert_eq!(snafu_to_decimal(&['1', '=', '-', '0', '-', '2']), 1747);
    }
    #[test]
    fn dec_to_sna() {
        assert_eq!(decimal_to_snafu(1747), vec!['1', '=', '-', '0', '-', '2']);
    }

    #[test]
    fn day25_part1() {
        let input = fs::read_to_string("input/day25").unwrap();
        assert_eq!(part_1(&input), "2-10==12-122-=1-1-22");
    }
}
