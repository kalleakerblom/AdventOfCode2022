use im::HashSet;
use std::{cmp, collections::HashMap};
type Id = u16;
const AA_ID: Id = b'A' as u16 * b'A' as u16;

fn get_flow_and_travel_maps(s: &str) -> (HashMap<Id, u32>, HashMap<Id, HashMap<Id, u32>>) {
    let mut flow_map = HashMap::new();
    let mut travel_map: HashMap<Id, HashMap<Id, u32>> = HashMap::new();
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
        let string_to_id = |s: &str| s.bytes().map(|b| b as u16).product();
        let valve_id = string_to_id(valve);
        let flow_rate = flow_rate.parse().unwrap();
        flow_map.insert(valve_id, flow_rate);

        for ov in other_valves {
            let other_id = string_to_id(ov);
            travel_map.entry(valve_id).or_default().insert(other_id, 1);
            travel_map.entry(other_id).or_default().insert(valve_id, 1);
        }
    }
    (flow_map, travel_map)
}

fn expand_travel_map(map: &mut HashMap<Id, HashMap<Id, u32>>, flow: &HashMap<Id, u32>) {
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
                    map.get_mut(valve).unwrap().insert(*other, shortest);
                    map.get_mut(other).unwrap().insert(*valve, shortest);
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
    current: Id,
    mut visited: HashSet<Id>,
    pressure: u32,
    time_left: u32,
    flow_map: &HashMap<Id, u32>,
    travel: &HashMap<Id, HashMap<Id, u32>>,
) -> u32 {
    visited.insert(current);
    let mut best = pressure;
    for (next, flow) in flow_map.iter() {
        if *flow == 0 {
            continue;
        }
        if visited.contains(next) {
            continue;
        }
        let time_cost = travel[&current][next] + 1;
        let value = time_left.saturating_sub(time_cost) * flow_map[next];
        if value == 0 {
            continue;
        }
        let candidate = recursive_best_plan(
            *next,
            visited.clone(),
            pressure + value,
            time_left.saturating_sub(time_cost),
            flow_map,
            travel,
        );
        best = cmp::max(best, candidate);
    }

    best
}

fn part_1(input: &str) -> u32 {
    let (flow, mut travel) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel, &flow);
    recursive_best_plan(AA_ID, HashSet::new(), 0, 30, &flow, &travel)
}

///////////////// part 2
fn recursive_make_plan_part2(
    current: Id,
    mut visited: HashSet<Id>,
    score: u32,
    other_score: Option<u32>,
    time_left: u32,
    flow_map: &HashMap<Id, u32>,
    travel: &HashMap<Id, HashMap<Id, u32>>,
    best: &mut u32,
) {
    visited.insert(current);
    if time_left == 0 {
        if let Some(other_score) = other_score {
            // end of elephant plan, add up the score
            let total_score = score + other_score;
            if total_score > *best {
                *best = total_score;
            }
            return;
        } else {
            //start elephant plan
            recursive_make_plan_part2(
                AA_ID,
                visited.clone(),
                0,
                Some(score),
                26,
                flow_map,
                travel,
                best,
            );
            return;
        }
    }
    for (next, flow) in flow_map.iter() {
        if *flow == 0 {
            continue;
        }
        if visited.contains(next) {
            continue;
        }
        let time_cost = travel[&current][next] + 1;
        if time_cost >= time_left {
            continue;
        }
        let value = flow_map[next] * (time_left - time_cost);
        recursive_make_plan_part2(
            *next,
            visited.clone(),
            score + value,
            other_score,
            time_left - time_cost,
            flow_map,
            travel,
            best,
        );
    }
    recursive_make_plan_part2(
        current,
        visited,
        score,
        other_score,
        0,
        flow_map,
        travel,
        best,
    );
}

fn part_2(input: &str) -> u32 {
    let (flow_map, mut travel) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel, &flow_map);
    let mut best = 0;
    recursive_make_plan_part2(
        AA_ID,
        HashSet::new(),
        0,
        None,
        26,
        &flow_map,
        &travel,
        &mut best,
    );
    best
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
        assert_eq!(part_1(&input), 1641);
    }
    #[test]
    fn example16_part2() {
        let input = fs::read_to_string("input/example16").unwrap();
        assert_eq!(part_2(&input), 1707);
    }
    #[test]
    fn day16_part2() {
        let input = fs::read_to_string("input/day16").unwrap();
        assert_eq!(part_2(&input), 2261);
    }
}
