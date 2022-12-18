use std::{cmp, collections::HashSet, env, fs};

use itertools::Itertools;

type Pos = (i64, i64);
const LEFT_BOUND: i64 = 0;
const RIGHT_BOUND: i64 = 8;

#[derive(Clone, Copy, Debug)]
enum Type {
    HBar,
    Cross,
    RevL,
    VBar,
    Box,
}

#[derive(Debug)]
struct Block {
    bottom_left: Pos,
    block_type: Type,
}

impl Block {
    fn new(block_type: Type, highest: i64) -> Self {
        let bottom_left = (LEFT_BOUND + 3, highest + 4);
        Self {
            bottom_left,
            block_type,
        }
    }
    fn drop(&mut self, stationary: &HashSet<Pos>) -> bool {
        if self.bottom_left.1 == 1 {
            return false;
        }
        let pos_blocked = |pos: Pos| stationary.contains(&pos);
        let (x, y) = self.bottom_left;
        let blocked: bool = match self.block_type {
            Type::HBar => (x..).take(4).map(|x_| (x_, y - 1)).any(pos_blocked),
            Type::Cross => [(x, y), (x + 1, y - 1), (x + 2, y)]
                .into_iter()
                .any(pos_blocked),
            Type::RevL => [(x, y - 1), (x + 1, y - 1), (x + 2, y - 1)]
                .into_iter()
                .any(pos_blocked),
            Type::VBar => pos_blocked((x, y - 1)),
            Type::Box => [(x, y - 1), (x + 1, y - 1)].into_iter().any(pos_blocked),
        };
        if !blocked {
            self.bottom_left = (x, y - 1);
            return true;
        }
        false
    }
    fn move_left(&mut self, stationary: &HashSet<Pos>) {
        let pos_blocked = |pos: Pos| pos.0 <= LEFT_BOUND || stationary.contains(&pos);
        let (x, y) = self.bottom_left;
        let blocked = match self.block_type {
            Type::HBar => pos_blocked((x - 1, y)),
            Type::Cross => [(x, y), (x - 1, y + 1), (x, y + 2)]
                .into_iter()
                .any(pos_blocked),
            Type::RevL => [(x - 1, y), (x + 1, y + 1), (x + 1, y + 2)]
                .into_iter()
                .any(pos_blocked),
            Type::VBar => [(x - 1, y), (x - 1, y + 1), (x - 1, y + 2), (x - 1, y + 3)]
                .into_iter()
                .any(pos_blocked),
            Type::Box => [(x - 1, y), (x - 1, y + 1)].into_iter().any(pos_blocked),
        };
        if !blocked {
            self.bottom_left = (x - 1, y);
        }
    }
    fn move_right(&mut self, stationary: &HashSet<Pos>) {
        let pos_blocked = |pos: Pos| pos.0 >= RIGHT_BOUND || stationary.contains(&pos);
        let (x, y) = self.bottom_left;
        let blocked = match self.block_type {
            Type::HBar => pos_blocked((x + 4, y)),
            Type::Cross => [(x + 2, y), (x + 3, y + 1), (x + 2, y + 2)]
                .into_iter()
                .any(pos_blocked),
            Type::RevL => [(x + 3, y), (x + 3, y + 1), (x + 3, y + 2)]
                .into_iter()
                .any(pos_blocked),
            Type::VBar => [(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 1, y + 3)]
                .into_iter()
                .any(pos_blocked),
            Type::Box => [(x + 2, y), (x + 2, y + 1)].into_iter().any(pos_blocked),
        };
        if !blocked {
            self.bottom_left = (x + 1, y);
        }
    }
    fn fill_stationary(&self, stationary: &mut HashSet<Pos>) {
        let fill = |pos| {
            stationary.insert(pos);
        };
        let (x, y) = self.bottom_left;
        match self.block_type {
            Type::HBar => (x..).take(4).map(|x_| (x_, y)).for_each(fill),
            Type::Cross => [
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ]
            .into_iter()
            .for_each(fill),
            Type::RevL => [
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ]
            .into_iter()
            .for_each(fill),
            Type::VBar => (y..).take(4).map(|y_| (x, y_)).for_each(fill),
            Type::Box => [(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)]
                .into_iter()
                .for_each(fill),
        }
    }
    fn highest(&self) -> i64 {
        match self.block_type {
            Type::HBar => self.bottom_left.1,
            Type::Cross => self.bottom_left.1 + 2,
            Type::RevL => self.bottom_left.1 + 2,
            Type::VBar => self.bottom_left.1 + 3,
            Type::Box => self.bottom_left.1 + 1,
        }
    }
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

const BLOCK_ORDER: [Type; 5] = [Type::HBar, Type::Cross, Type::RevL, Type::VBar, Type::Box];

fn calc_tower_height(winds: &str, blocks: u64) -> u64 {
    let mut block_count = 0;
    let mut highest = 0;
    let mut type_index = 0;
    let winds = winds
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            bad => panic!("bad:{bad}"),
        })
        .collect_vec();
    let mut wind_index = 0;
    let mut stationary = HashSet::<Pos>::new();
    while block_count < blocks {
        let mut new_block = Block::new(BLOCK_ORDER[type_index], highest);
        type_index = (type_index + 1) % BLOCK_ORDER.len();
        block_count += 1;
        loop {
            match winds[wind_index] {
                Dir::Left => new_block.move_left(&stationary),
                Dir::Right => new_block.move_right(&stationary),
            }
            wind_index = (wind_index + 1) % winds.len();
            if !new_block.drop(&stationary) {
                break;
            }
        }
        highest = cmp::max(highest, new_block.highest());
        new_block.fill_stationary(&mut stationary);
    }
    draw(stationary, highest);
    highest as u64
}

fn part_1(input: &str) -> u64 {
    calc_tower_height(input, 2022)
}
fn part_2(input: &str) -> u64 {
    calc_tower_height(input, 6000) // 1_000_000_000_000
}

fn draw(stationary: HashSet<Pos>, highest: i64) {
    let mut output = String::new();
    for y in (1..=highest).rev() {
        let mut line = String::new();
        line.push('|');
        for x in 1..8 {
            if stationary.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        line.push('|');
        output.push_str(&line);
        output.push('\n');
    }
    output.push_str("+-------+");
    fs::write("output/drawing_day17.txt", output);
}

#[cfg(test)]
mod tests {
    use crate::day17::*;
    use std::fs;
    #[test]
    fn example17_part1() {
        assert_eq!(part_1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"), 3068);
    }
    #[test]
    fn day17_part1() {
        let input = fs::read_to_string("input/day17").unwrap();
        assert_eq!(part_1(&input), 3114);
    }
    #[test]
    fn example17_part2() {
        assert_eq!(
            part_2(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"),
            1_514_285_714_288
        );
    }
    #[test]
    fn day17_part2() {
        let input = fs::read_to_string("input/day17").unwrap();
        let blocks = 1_000_000_000_000u64 - 240;
        // after 240 blocks (height 365), same pattern repeats every 1740 blocks, with height 2681
        let div = blocks / 1740;
        let rem = dbg!(blocks % 1740);
        // height of remainder blocks of repeating section = 1434
        assert_eq!(365 + div * 2681 + 1434, 1540804597682u64);
        assert_eq!(part_2(&input), 1540804597682u64);
    }
}
