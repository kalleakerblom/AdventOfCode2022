use std::collections::HashMap;

struct Dir {
    files: u64,
    subdirs: Vec<String>,
}
fn get_dir_map(input: &str) -> HashMap<String, Dir> {
    let mut lines = input.lines().peekable();
    let mut path = Vec::new();
    let mut result = HashMap::new();
    while let Some(next) = lines.next() {
        if next == "$ cd .." {
            path.pop();
            continue;
        }
        if let Some(subdir) = next.strip_prefix("$ cd ") {
            path.push(subdir);
            continue;
        } else {
            assert!(next == "$ ls");
        }
        // read ls printout
        let mut files = 0;
        let mut subdirs = Vec::new();
        while lines.peek().filter(|l| !l.starts_with('$')).is_some() {
            let content = lines.next().unwrap();
            if let Some(subdir) = content.strip_prefix("dir ") {
                subdirs.push(subdir.to_string());
            } else {
                // only care about file size at the moment
                files += content.split_once(' ').unwrap().0.parse::<u64>().unwrap();
            }
        }
        result.insert(path.join("/"), Dir { files, subdirs });
    }
    result
}

fn get_dir_size(
    path: &str,
    dir_map: &HashMap<String, Dir>,
    size_map: &mut HashMap<String, u64>,
) -> u64 {
    let dir = &dir_map[path];
    let mut size: u64 = dir.files;
    for subdir in &dir.subdirs {
        let subpath = path.to_string() + "/" + subdir;
        size += get_dir_size(&subpath, dir_map, size_map);
    }
    size_map.insert(path.to_string(), size);
    size
}

pub fn part_1(input: &str) -> u64 {
    let dir_map = get_dir_map(input);
    let mut size_map = HashMap::new();
    get_dir_size("/", &dir_map, &mut size_map);
    size_map
        .iter()
        .filter_map(|(_k, &v)| if v < 100_000 { Some(v) } else { None })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let dir_map = get_dir_map(input);
    let mut size_map = HashMap::new();
    get_dir_size("/", &dir_map, &mut size_map);
    let to_cut = 30_000_000 - (70_000_000 - size_map["/"]);
    size_map
        .iter()
        .filter_map(|(_k, size)| if *size >= to_cut { Some(*size) } else { None })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day07::*;
    use std::fs;
    #[test]
    fn example07_day_part1() {
        let input = fs::read_to_string("input/example07").unwrap();
        assert_eq!(part_1(&input), 95437);
    }
    #[test]
    fn day07_part1() {
        let input = fs::read_to_string("input/day07").unwrap();
        assert_eq!(part_1(&input), 1443806);
    }
    #[test]
    fn example07_part2() {
        let input = fs::read_to_string("input/example07").unwrap();
        assert_eq!(part_2(&input), 24933642);
    }
    #[test]
    fn day07_part2() {
        let input = fs::read_to_string("input/day07").unwrap();
        assert_eq!(part_2(&input), 942298);
    }
}
