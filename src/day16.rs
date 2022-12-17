use std::{cmp, collections::HashMap};

fn get_flow_and_travel_maps(
    s: &str,
) -> (HashMap<String, u32>, HashMap<String, HashMap<String, u32>>) {
    let mut flow_map = HashMap::new();
    let mut travel_map: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for l in s.lines() {
        let (valve, rest) = l.trim_start_matches("Valve ").split_once(' ').unwrap();
        let (flow_rate, rest) = rest
            .trim_start_matches("has flow rate=")
            .split_once(';')
            .unwrap();
        let other_valves = rest
            .trim_start_matches(" tunnels lead to valves ")
            .trim_start_matches(" tunnel leads to valve ");
        let other_valves = other_valves.split(", ");

        flow_map.insert(valve.into(), flow_rate.parse().unwrap());
        for ov in other_valves {
            travel_map
                .entry(valve.into())
                .or_default()
                .insert(ov.into(), 1);
            travel_map
                .entry(ov.into())
                .or_default()
                .insert(valve.into(), 1);
        }
    }
    (flow_map, travel_map)
}

fn expand_travel_map(map: &mut HashMap<String, HashMap<String, u32>>, flow: &HashMap<String, u32>) {
    loop {
        let mut improved = false;
        for valve in flow.keys() {
            for other in flow.keys() {
                if valve == other {
                    continue;
                }
                let mut shortest: u32 = map[valve].get(other).cloned().unwrap_or(u32::MAX);
                let mut beat_current_best = false;
                for mid in map[valve].iter() {
                    let candidate = map[mid.0]
                        .get(other)
                        .map(|cost| mid.1 + cost)
                        .unwrap_or(u32::MAX);
                    if candidate < shortest {
                        shortest = candidate;
                        beat_current_best = true;
                    }
                }
                if beat_current_best {
                    map.get_mut(valve).unwrap().insert(other.into(), shortest);
                    map.get_mut(other).unwrap().insert(valve.into(), shortest);
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }
}

fn recursive_best_plan(
    plan: &mut Vec<String>,
    pressure: u32,
    time_left: u32,
    flow: &HashMap<String, u32>,
    travel: &HashMap<String, HashMap<String, u32>>,
) -> u32 {
    if time_left == 0 {
        return pressure;
    }
    let mut best = pressure;
    for next in flow.keys() {
        if plan.contains(next) {
            continue;
        }
        let time_cost = travel[plan.last().unwrap()][next] + 1;
        let value = time_left.saturating_sub(time_cost) * flow[next];
        if value == 0 {
            continue;
        }
        plan.push(next.to_string());
        let candidate = recursive_best_plan(
            plan,
            pressure + value,
            time_left.saturating_sub(time_cost),
            flow,
            travel,
        );
        plan.pop();
        best = cmp::max(best, candidate);
    }

    best
}

fn part_1(input: &str) -> u32 {
    let (flow, mut travel) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel, &flow);
    recursive_best_plan(&mut vec!["AA".into()], 0, 30, &flow, &travel)
}

///////////////// part 2

fn part_2(input: &str) -> u32 {
    let (flow, mut travel) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel, &flow);
    dbg!(&travel["AA"]);
    0
    //recursive_best_plan_part2(&mut vec!["AA".into()], 0, 26, &flow, &travel)
}

#[cfg(test)]
mod tests {
    use crate::day16::*;
    use std::fs;
    #[test]
    fn example16_part1() {
        let input = fs::read_to_string("input/example16").unwrap();
        assert_eq!(part_1(&input), 1651);
    }
    #[test]
    fn day16_part1() {
        let input = fs::read_to_string("input/day16").unwrap();
        //1355 too low
        assert_eq!(part_1(&input), 1641);
    }
    #[test]
    fn example16_part2() {
        let input = fs::read_to_string("input/example16").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn day16_part2() {
        let input = fs::read_to_string("input/day16").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
