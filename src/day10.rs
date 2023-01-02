struct Cpu<'a> {
    register: i32,
    wait: usize,
    to_add: i32,
    instructions: std::str::Lines<'a>,
}

impl<'a> Cpu<'a> {
    fn new(i: &'a str) -> Self {
        Self {
            register: 1,
            instructions: i.lines(),
            wait: 0,
            to_add: 0,
        }
    }

    fn step(&mut self) {
        if self.wait > 0 {
            self.wait -= 1;
            return;
        }
        self.register += self.to_add;
        match self.instructions.next().unwrap() {
            "noop" => self.to_add = 0,
            addx => {
                self.wait = 1;
                self.to_add = addx.trim_start_matches("addx ").parse::<i32>().unwrap();
            }
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    let mut cpu = Cpu::new(input);
    let mut ans = 0;
    for _ in 0..20 {
        cpu.step();
    }
    ans += 20 * cpu.register;
    for c in [60, 100, 140, 180, 220] {
        for _ in 0..40 {
            cpu.step();
        }
        ans += c * cpu.register;
    }
    ans
}

fn render(mut cpu: Cpu) -> String {
    let mut image = String::new();
    for _row in 0..6 {
        for col in 0..40 {
            cpu.step();
            if (cpu.register - col).abs() < 2 {
                image.push('#');
            } else {
                image.push('.');
            }
        }
        image.push('\n');
    }
    image
}

pub fn part_2(input: &str) -> String {
    let cpu = Cpu::new(input);
    render(cpu)
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    use std::fs;
    #[test]
    fn example10_day_part1() {
        let input = fs::read_to_string("input/example10").unwrap();
        assert_eq!(part_1(&input), 13140);
    }
    #[test]
    fn day10_part1() {
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(part_1(&input), 11960);
    }
    #[test]
    fn example10_part2() {
        let input = fs::read_to_string("input/example10").unwrap();
        assert_eq!(
            part_2(&input),
            fs::read_to_string("input/example10_image").unwrap()
        );
    }
    #[test]
    fn day10_part2() {
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(
            part_2(&input),
            fs::read_to_string("input/day10_image").unwrap()
        );
    }
}
