use std::collections::HashMap;

type Id = String;
#[derive(Clone)]
enum Expr {
    Val(i64),
    Op(Id, Id, fn(i64, i64) -> i64),
}

impl Expr {
    fn parse(s: &str) -> (Id, Expr) {
        let (id, s) = s.split_once(':').unwrap();
        let split: Vec<&str> = s.split_whitespace().collect();
        let id = id.to_string();
        if split.len() == 1 {
            return (id, Expr::Val(split[0].parse().unwrap()));
        }
        let left = split[0].to_string();
        let right = split[2].to_string();
        let op = match split[1] {
            "+" => |a, b| a + b,
            "-" => |a, b| a - b,
            "*" => |a, b| a * b,
            "/" => |a, b| a / b,
            _ => panic!(),
        };
        (id, Expr::Op(left, right, op))
    }
}

fn eval(id: &Id, exprs: &mut HashMap<Id, Expr>) -> i64 {
    match exprs[id].clone() {
        Expr::Val(v) => v,
        Expr::Op(l, r, op) => {
            let res = op(eval(&l, exprs), eval(&r, exprs));
            *exprs.get_mut(id).unwrap() = Expr::Val(res);
            res
        }
    }
}

fn eval_part2(id: &Id, exprs: &mut HashMap<Id, Expr>) -> Expr {
    match exprs[id].clone() {
        Expr::Val(v) => Expr::Val(v),
        Expr::Op(l, r, op) if l == "humn" => {
            eval_part2(&r, exprs);
            *exprs.get_mut(id).unwrap() = Expr::Op(l, r, op);
            exprs[id].clone()
        }
        Expr::Op(l, r, op) if r == "humn" => {
            eval_part2(&l, exprs);
            *exprs.get_mut(id).unwrap() = Expr::Op(l, r, op);
            exprs[id].clone()
        }
        Expr::Op(l, r, op) => {
            if let (Expr::Val(a), Expr::Val(b)) = (eval_part2(&l, exprs), eval_part2(&r, exprs)) {
                let res = op(a, b);
                *exprs.get_mut(id).unwrap() = Expr::Val(res);
                exprs[id].clone()
            } else {
                exprs[id].clone()
            }
        }
    }
}

fn op_char(op: &fn(i64, i64) -> i64) -> char {
    // ooooof, please forgive
    match op(3, 3) {
        6 => '+',
        0 => '-',
        9 => '*',
        1 => '/',
        _ => panic!(),
    }
}

fn print(id: &Id, exprs: &HashMap<Id, Expr>) {
    if id == "humn" {
        print!("x");
    } else {
        match exprs.get(id).unwrap() {
            Expr::Val(v) => print!("{v}"),
            Expr::Op(l, r, op) => {
                print!("(");
                print(l, exprs);
                let oc = op_char(op);
                print!("{oc}");
                print(r, exprs);
                print!(")");
            }
        }
    }
}

pub fn part_1(input: &str) -> i64 {
    let mut exprs: HashMap<_, _> = input.lines().map(Expr::parse).collect();
    eval(&"root".to_string(), &mut exprs)
}
pub fn part_2(input: &str) -> u64 {
    let mut exprs: HashMap<_, _> = input.lines().map(Expr::parse).collect();
    if let Expr::Op(l, r, _) = exprs.get("root").unwrap() {
        let (l, r) = (l.clone(), r.clone());
        eval_part2(&l, &mut exprs);
        eval_part2(&r, &mut exprs);
        print(&l, &exprs);
        println!();
        print(&r, &exprs);
        println!();
    } else {
        panic!()
    }
    todo!("Hacky solve: Print l & r and go to Wolfram Alpha to solve it.")
}

#[cfg(test)]
mod tests {
    use crate::day21::*;
    use std::fs;
    #[test]
    fn example21_part1() {
        let input = fs::read_to_string("input/example21").unwrap();
        assert_eq!(part_1(&input), 152);
    }
    #[test]
    fn day21_part1() {
        let input = fs::read_to_string("input/day21").unwrap();
        assert_eq!(part_1(&input), 110181395003396);
    }
    #[test]
    fn example21_part2() {
        let input = fs::read_to_string("input/example21").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn day21_part2() {
        let input = fs::read_to_string("input/day21").unwrap();
        assert_eq!(part_2(&input), 3721298272959);
    }
}
