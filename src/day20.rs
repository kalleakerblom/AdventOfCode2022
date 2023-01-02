fn decrypt(mut nums: Vec<(usize, i64)>, key: i64, repeat: usize) -> i64 {
    let len = nums.len();
    nums.iter_mut().for_each(|im| im.1 *= key);
    for _ in 0..repeat {
        for i in 0..len {
            let old_pos = nums
                .iter()
                .position(|&(origin, _value)| origin == i)
                .unwrap();
            let val = nums[old_pos].1;
            let new_pos = (old_pos as i64 + val).rem_euclid(len as i64 - 1);
            nums.remove(old_pos);
            nums.insert(new_pos as usize, (i, val));
        }
    }
    let zero_pos = nums.iter().position(|(_, val)| *val == 0).unwrap();
    let a = nums[(zero_pos + 1000) % nums.len()].1;
    let b = nums[(zero_pos + 2000) % nums.len()].1;
    let c = nums[(zero_pos + 3000) % nums.len()].1;
    a + b + c
}

fn part_1(input: &str) -> i64 {
    let nums: Vec<(usize, i64)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();
    decrypt(nums, 1, 1)
}

fn part_2(input: &str) -> i64 {
    let nums: Vec<(usize, i64)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();
    decrypt(nums, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use crate::day20::*;
    use std::fs;
    #[test]
    fn example20_part1() {
        let input = fs::read_to_string("input/example20").unwrap();
        assert_eq!(part_1(&input), 3);
    }
    #[test]
    fn day20_part1() {
        let input = fs::read_to_string("input/day20").unwrap();
        assert_eq!(part_1(&input), 4151);
    }
    #[test]
    fn example20_part2() {
        let input = fs::read_to_string("input/example20").unwrap();
        assert_eq!(part_2(&input), 1623178306);
    }
    #[test]
    fn day20_part2() {
        let input = fs::read_to_string("input/day20").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
