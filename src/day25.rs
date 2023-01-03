fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .bytes()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => panic!(),
        } * 5i64.pow(i as u32))
        .sum()
}
const SNAFU: [(i8, u8); 5] = [(-2, b'='), (-1, b'-'), (0, b'0'), (1, b'1'), (2, b'2')];
fn decimal_to_snafu(i: u64) -> String {
    let mut current = 0;
    let mut symbols = 0;
    for exp in 0u32.. {
        if current > i {
            break;
        }
        current += 2 * 5u64.pow(exp);
        symbols += 1;
    }
    let mut res = vec![b'2'; symbols];
    let len = res.len() as u32;
    for (exp, c) in res.iter_mut().enumerate() {
        let five_pow = 5u64.pow(len - 1 - exp as u32);
        for (val, symbol) in &SNAFU[..4] {
            let sub = (2 - val) as u64 * five_pow;
            if current >= sub + i {
                current -= sub;
                *c = *symbol;
                break;
            }
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}

pub fn part_1(input: &str) -> String {
    let sum: i64 = input.lines().map(snafu_to_decimal).sum();
    decimal_to_snafu(sum as u64)
}

#[cfg(test)]
mod tests {
    use crate::day25::*;
    use std::fs;
    #[test]
    fn sna_to_dec() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
    }
    #[test]
    fn dec_to_sna() {
        for _ in 0..1_000_000 {
            assert_eq!(decimal_to_snafu(1747), "1=-0-2");
        }
    }

    #[test]
    fn day25_part1() {
        let input = fs::read_to_string("input/day25").unwrap();
        assert_eq!(part_1(&input), "2-10==12-122-=1-1-22");
    }
}
