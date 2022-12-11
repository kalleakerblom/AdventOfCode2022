use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    inspection_count: u64,
    inspect_op: fn(u64) -> u64,
    pass_op: fn(u64) -> usize,
}

fn example_monkeys() -> Vec<Monkey> {
    [
        Monkey {
            items: [79, 98].into(),
            inspection_count: 0,
            inspect_op: |val| val * 19,
            pass_op: |worry| if worry % 23 == 0 { 2 } else { 3 },
        },
        Monkey {
            items: [54, 65, 75, 74].into(),
            inspection_count: 0,
            inspect_op: |val| val + 6,
            pass_op: |worry| if worry % 19 == 0 { 2 } else { 0 },
        },
        Monkey {
            items: [79, 60, 97].into(),
            inspection_count: 0,
            inspect_op: |val| val * val,
            pass_op: |worry| if worry % 13 == 0 { 1 } else { 3 },
        },
        Monkey {
            items: [74].into(),
            inspection_count: 0,
            inspect_op: |val| val + 3,
            pass_op: |worry| if worry % 17 == 0 { 0 } else { 1 },
        },
    ]
    .into()
}
fn real_monkeys() -> Vec<Monkey> {
    [
        Monkey {
            items: [53, 89, 62, 57, 74, 51, 83, 97].into(),
            inspection_count: 0,
            inspect_op: |val| val * 3,
            pass_op: |worry| if worry % 13 == 0 { 1 } else { 5 },
        },
        Monkey {
            items: [85, 94, 97, 92, 56].into(),
            inspection_count: 0,
            inspect_op: |val| val + 2,
            pass_op: |worry| if worry % 19 == 0 { 5 } else { 2 },
        },
        Monkey {
            items: [86, 82, 82].into(),
            inspection_count: 0,
            inspect_op: |val| val + 1,
            pass_op: |worry| if worry % 11 == 0 { 3 } else { 4 },
        },
        Monkey {
            items: [94, 68].into(),
            inspection_count: 0,
            inspect_op: |val| val + 5,
            pass_op: |worry| if worry % 17 == 0 { 7 } else { 6 },
        },
        Monkey {
            items: [83, 62, 74, 58, 96, 68, 85].into(),
            inspection_count: 0,
            inspect_op: |val| val + 4,
            pass_op: |worry| if worry % 3 == 0 { 3 } else { 6 },
        },
        Monkey {
            items: [50, 68, 95, 82].into(),
            inspection_count: 0,
            inspect_op: |val| val + 8,
            pass_op: |worry| if worry % 7 == 0 { 2 } else { 4 },
        },
        Monkey {
            items: [75].into(),
            inspection_count: 0,
            inspect_op: |val| val * 7,
            pass_op: |worry| if worry % 5 == 0 { 7 } else { 0 },
        },
        Monkey {
            items: [92, 52, 85, 89, 68, 82].into(),
            inspection_count: 0,
            inspect_op: |val| val * val,
            pass_op: |worry| if worry % 2 == 0 { 0 } else { 1 },
        },
    ]
    .into()
}

fn part_1(mut monkeys: Vec<Monkey>) -> u64 {
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let worry = (monkeys[i].inspect_op)(item) / 3;
                let target = (monkeys[i].pass_op)(worry);
                monkeys[target].items.push_back(worry);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspection_count);
    monkeys[monkeys.len() - 2].inspection_count * monkeys[monkeys.len() - 1].inspection_count
}

fn part_2(mut monkeys: Vec<Monkey>, module: u64) -> u64 {
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let worry = (monkeys[i].inspect_op)(item) % module;
                let target = (monkeys[i].pass_op)(worry);
                monkeys[target].items.push_back(worry);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspection_count);
    monkeys[monkeys.len() - 2].inspection_count * monkeys[monkeys.len() - 1].inspection_count
}

#[cfg(test)]
mod tests {
    use crate::day11::*;
    use std::fs;
    #[test]
    fn example11_day_part1() {
        assert_eq!(part_1(example_monkeys()), 10605);
    }
    #[test]
    fn day11_part1() {
        assert_eq!(part_1(real_monkeys()), 110220);
    }
    #[test]
    fn example11_part2() {
        assert_eq!(part_2(example_monkeys(), 23 * 19 * 13 * 17), 2713310158);
    }
    #[test]
    fn day11_part2() {
        assert_eq!(part_2(real_monkeys(), 13 * 19 * 11 * 17 * 3 * 7 * 5 * 2), 0);
    }
}
