use std::{cmp, collections::HashSet};

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

fn part_1(input: &str) -> u64 {
    let mut block_count = 0;
    let mut highest = 0;
    let mut type_iter = BLOCK_ORDER.iter().cycle();
    let mut winds_iter = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            bad => panic!("bad:{bad}"),
        })
        .cycle();
    let mut stationary = HashSet::<Pos>::new();
    while block_count < 2022 {
        let mut new_block = Block::new(*type_iter.next().unwrap(), highest);
        block_count += 1;
        loop {
            match winds_iter.next().unwrap() {
                Dir::Left => new_block.move_left(&stationary),
                Dir::Right => new_block.move_right(&stationary),
            }
            if !new_block.drop(&stationary) {
                break;
            }
        }
        highest = cmp::max(highest, new_block.highest());
        new_block.fill_stationary(&mut stationary);
    }
    //draw(stationary, highest);
    highest as u64
}
fn part_2(input: &str) -> u64 {
    todo!()
}

fn draw(stationary: HashSet<Pos>, highest: i64) {
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
        println!("{line}");
    }
    println!("+-------+");
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
        let input = fs::read_to_string("input/example17").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn day17_part2() {
        let input = fs::read_to_string("input/day17").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
