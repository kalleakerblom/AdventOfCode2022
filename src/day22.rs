use std::{collections::HashMap, iter};

type V2d = cgmath::Vector2<i32>;
enum Turn {
    Left,
    Right,
}
fn turn(dir: V2d, turn: &Turn) -> V2d {
    match turn {
        Turn::Left => V2d::new(-dir.y, dir.x),
        Turn::Right => V2d::new(dir.y, -dir.x),
    }
}
enum Tile {
    Floor,
    Wall,
}
type TileMap = HashMap<V2d, Tile>;
enum Cmd {
    Move(u32),
    Turn(Turn),
}
fn parse_commands(s: &str) -> Vec<Cmd> {
    let mut res = Vec::new();
    let mut chars = s.trim().chars().peekable();
    while let Some(next) = chars.next() {
        match next {
            'L' => res.push(Cmd::Turn(Turn::Left)),
            'R' => res.push(Cmd::Turn(Turn::Right)),
            _ => {
                let mut digits = String::new();
                digits.push(next);
                while let Some(d) = chars.peek() {
                    if !d.is_ascii_digit() {
                        break;
                    }
                    digits.push(*d);
                    chars.next();
                }
                res.push(Cmd::Move(digits.parse().unwrap()))
            }
        }
    }
    res
}

fn parse_map_and_start(s: &str) -> (TileMap, V2d) {
    let mut res = TileMap::new();
    let mut start = None;
    s.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let pos = V2d::new(x as i32, -(row as i32));
            match ch {
                '.' => {
                    start.get_or_insert(pos);
                    res.insert(pos, Tile::Floor)
                }
                '#' => res.insert(pos, Tile::Wall),
                _ => None,
            };
        })
    });
    (res, start.unwrap())
}

fn wrap_map_pos(p: V2d, dir: V2d, map: &TileMap) -> V2d {
    let rev = -dir;
    iter::repeat(())
        .scan(p, |pos, _| {
            *pos += rev;
            if map.contains_key(pos) {
                Some(*pos)
            } else {
                None
            }
        })
        .last()
        .unwrap()
}

fn execute(start: V2d, dir: V2d, map: &TileMap, cmds: &[Cmd]) -> (V2d, V2d) {
    let mut pos = start;
    let mut dir = dir;
    for c in cmds {
        match c {
            Cmd::Turn(t) => dir = turn(dir, t),
            Cmd::Move(steps) => {
                for _ in 0..*steps {
                    match map.get(&(pos + dir)) {
                        Some(Tile::Wall) => break,
                        Some(Tile::Floor) => pos += dir,
                        None => {
                            let wrapped = wrap_map_pos(pos, dir, map);
                            if let Tile::Floor = map[&wrapped] {
                                pos = wrapped
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    (pos, dir)
}

pub fn part_1(input: &str) -> i32 {
    let (map, cmds) = input.split_once("\n\n").unwrap();
    let (map, start) = parse_map_and_start(map);
    let cmds = parse_commands(cmds);
    let (pos, dir) = execute(start, V2d::new(1, 0), &map, &cmds);
    dbg!(pos, dir);
    let row = -pos.y + 1;
    let column = pos.x + 1;
    let facing = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => panic!(),
    };
    1000 * row + 4 * column + facing
}
/////////////PART 2//////////////

fn execute2(start: V2d, dir: V2d, map: &TileMap, cmds: &[Cmd]) -> (V2d, V2d) {
    let wrap_map = wrap_map();
    let mut pos = start;
    let mut dir = dir;
    for c in cmds {
        match c {
            Cmd::Turn(t) => dir = turn(dir, t),
            Cmd::Move(steps) => {
                for _ in 0..*steps {
                    match map.get(&(pos + dir)) {
                        Some(Tile::Wall) => break,
                        Some(Tile::Floor) => pos += dir,
                        None => {
                            dbg!(pos + dir);
                            let (wrap_pos, wrap_dir) = wrap_map[&(pos + dir)];
                            if let Tile::Floor = map[&wrap_pos] {
                                pos = wrap_pos;
                                dir = wrap_dir;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    (pos, dir)
}

fn wrap_map() -> HashMap<V2d, (V2d, V2d)> {
    use cgmath::vec2;
    let mut res = HashMap::new();
    //A
    for y in 0..50 {
        res.insert(vec2(150, -y), (vec2(99, -149 + y), vec2(-1, 0)));
    }
    for y in 100..150 {
        res.insert(vec2(100, -y), (vec2(149, -149 + y), vec2(-1, 0)));
    }
    //B
    for x in 100..150 {
        res.insert(vec2(x, 1), (vec2(x - 100, -199), vec2(0, 1)));
    }
    for x in 0..50 {
        res.insert(vec2(x, -200), (vec2(x + 100, 0), vec2(0, -1)));
    }
    //C
    for x in 100..150 {
        res.insert(vec2(x, -50), (vec2(99, 50 - x), vec2(-1, 0)));
    }
    for y in 50..100 {
        res.insert(vec2(100, -y), (vec2(50 + y, -49), vec2(0, 1)));
    }
    //D
    for y in 50..100 {
        res.insert(vec2(49, -y), (vec2(y - 50, -100), vec2(0, -1)));
    }
    for x in 0..50 {
        res.insert(vec2(x, -99), (vec2(50, -x - 50), vec2(1, 0)));
    }
    //E
    for x in 50..100 {
        res.insert(vec2(x, -150), (vec2(49, -x - 100), vec2(-1, 0)));
    }
    for y in 150..200 {
        res.insert(vec2(50, -y), (vec2(y - 100, -149), vec2(0, 1)));
    }
    //F
    for y in 0..50 {
        res.insert(vec2(49, -y), (vec2(0, -149 + y), vec2(1, 0)));
    }
    for y in 100..150 {
        res.insert(vec2(-1, -y), (vec2(50, -149 + y), vec2(1, 0)));
    }
    //G
    for x in 50..100 {
        res.insert(vec2(x, 1), (vec2(0, -100 - x), vec2(1, 0)));
    }
    for y in 150..200 {
        res.insert(vec2(-1, -y), (vec2(y - 100, 0), vec2(0, -1)));
    }
    res
}

pub fn part_2(input: &str) -> i32 {
    let (map, cmds) = input.split_once("\n\n").unwrap();
    let (map, start) = parse_map_and_start(map);
    let cmds = parse_commands(cmds);
    // NOTE: execute2 uses hardcoded wrap_map for my actual input
    let (pos, dir) = execute2(start, V2d::new(1, 0), &map, &cmds);
    let row = -pos.y + 1;
    let column = pos.x + 1;
    let facing = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => panic!(),
    };
    1000 * row + 4 * column + facing
}

#[cfg(test)]
mod tests {
    use crate::day22::*;
    use std::fs;
    #[test]
    fn example22_part1() {
        let input = fs::read_to_string("input/example22").unwrap();
        assert_eq!(part_1(&input), 6032);
    }
    #[test]
    fn day22_part1() {
        let input = fs::read_to_string("input/day22").unwrap();
        assert_eq!(part_1(&input), 0);
    }
    #[test]
    fn example22_part2() {
        let input = fs::read_to_string("input/example22").unwrap();
        assert_eq!(part_2(&input), 5031);
    }
    #[test]
    fn wrap_map_test() {
        let wrap_map = wrap_map();
        for (key, wrap) in wrap_map.iter() {
            let (p, d) = wrap_map[&(wrap.0 - wrap.1)];
            assert_eq!(*key, p - d);
        }
    }
    #[test]
    fn day22_part2() {
        let input = fs::read_to_string("input/day22").unwrap();
        assert_eq!(part_2(&input), 110400);
    }
}
