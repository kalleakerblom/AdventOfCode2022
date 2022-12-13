use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Node {
    Val(i32),
    List(Vec<Node>),
}

impl Node {
    fn parse(s: &str) -> Self {
        parse_recursive(&mut s.chars().peekable()).unwrap()
    }
}

fn parse_recursive<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Option<Node> {
    if matches!(chars.peek(), Some('[')) {
        chars.next().unwrap();
        let mut list = Vec::new();
        while let Some(node) = parse_recursive(chars) {
            list.push(node);
            if matches!(chars.peek(), Some(',')) {
                chars.next();
            }
        }
        Node::List(list).into()
    } else if matches!(chars.peek(), Some(']')) {
        chars.next();
        None
    } else {
        let mut num = String::new();
        loop {
            if matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                num.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        Node::Val(num.parse().unwrap()).into()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Val(a), Node::Val(b)) => a.cmp(b),
            (Node::Val(a), b @ Node::List(_)) => {
                let list_a = Node::List([Node::Val(*a)].into());
                list_a.cmp(b)
            }
            (a @ Node::List(_), Node::Val(b)) => {
                let list_b = Node::List([Node::Val(*b)].into());
                a.cmp(&list_b)
            }
            (Node::List(a), Node::List(b)) => a
                .iter()
                .zip(b.iter())
                .find_map(|(na, nb)| match na.cmp(nb) {
                    Ordering::Equal => None,
                    cmp => Some(cmp),
                })
                .unwrap_or_else(|| a.len().cmp(&b.len())),
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut count = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let mut lines = pair.lines();
        let a = Node::parse(lines.next().unwrap());
        let b = Node::parse(lines.next().unwrap());
        match a.partial_cmp(&b).unwrap() {
            Ordering::Less => count += i + 1,
            Ordering::Equal => panic!(),
            Ordering::Greater => (),
        }
    }
    count
}
fn part_2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .split("\n\n")
        .flat_map(str::lines)
        .map(Node::parse)
        .collect();
    let n2 = Node::parse("[[2]]");
    let n6 = Node::parse("[[6]]");
    packets.push(n2.clone());
    packets.push(n6.clone());
    packets.sort_unstable();
    let pos2 = packets.binary_search(&n2);
    let pos6 = packets.binary_search(&n6);
    (pos2.unwrap() + 1) * (pos6.unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use crate::day13::*;
    use std::fs;
    #[test]
    fn example13_day_part1() {
        let input = fs::read_to_string("input/example13").unwrap();
        assert_eq!(part_1(&input), 13);
    }
    #[test]
    fn day13_part1() {
        let input = fs::read_to_string("input/day13").unwrap();
        assert_eq!(part_1(&input), 6395);
    }
    #[test]
    fn example13_part2() {
        let input = fs::read_to_string("input/example13").unwrap();
        assert_eq!(part_2(&input), 140);
    }
    #[test]
    fn day13_part2() {
        let input = fs::read_to_string("input/day13").unwrap();
        assert_eq!(part_2(&input), 24921);
    }
}
