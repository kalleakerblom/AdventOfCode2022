use bitvec::prelude::*;
use im::HashSet;
use std::{cell::Cell, collections::HashMap};
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
struct DfsFlowPlan {
    flow_map: HashMap<Id, u32>,
    travel_map: HashMap<(Id, Id), u32>,
}

impl DfsFlowPlan {
    fn search(&self, current: Id, mut visited: HashSet<Id>, pressure: u32, time_left: u32) -> u32 {
        visited.insert(current);
        let mut best_pressure = pressure;
        for (next, flow) in self.flow_map.iter() {
            if *flow == 0 || visited.contains(next) {
                continue;
            }
            let time_cost = self.travel_map[&(current, *next)] + 1;
            let value = time_left.saturating_sub(time_cost) * self.flow_map[next];
            if value == 0 {
                continue;
            }
            let candidate = self.search(
                *next,
                visited.clone(),
                pressure + value,
                time_left.saturating_sub(time_cost),
            );
            best_pressure = best_pressure.max(candidate);
        }
        best_pressure
    }
}

pub fn part_1(input: &str) -> u32 {
    let (flow_map, mut travel_map) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel_map, &flow_map);
    let travel_map: HashMap<(Id, Id), u32> = travel_map
        .iter()
        .flat_map(|(&id_a, inner)| inner.iter().map(move |(id_b, time)| ((id_a, *id_b), *time)))
        .collect();
    let dfs = DfsFlowPlan {
        travel_map,
        flow_map,
    };
    dfs.search(AA_ID, HashSet::new(), 0, 30)
}

///////////////// part 2
struct DfsFlowPlanPart2<'a> {
    flow_map: &'a HashMap<Id, u32>,
    travel_map: &'a HashMap<(Id, Id), u32>,
    valve_map: &'a HashMap<Id, u8>,
    best: Cell<u32>,
}

impl<'a> DfsFlowPlanPart2<'a> {
    fn search(
        &self,
        current_id: Id,
        mut visited: bitvec::array::BitArray,
        score: u32,
        other_score: Option<u32>,
        time_left: u32,
    ) {
        let valve_index = self.valve_map[&current_id];
        visited.set(valve_index as usize, true);
        if time_left == 0 {
            if let Some(other_score) = other_score {
                // end of elephant plan, add up the score
                let total_score = score + other_score;
                if total_score > self.best.get() {
                    self.best.set(total_score);
                }
                return;
            } else {
                //start elephant plan
                let elephant_dfs = DfsFlowPlanPart2 {
                    flow_map: self.flow_map,
                    travel_map: self.travel_map,
                    valve_map: self.valve_map,
                    best: Cell::new(0),
                };
                elephant_dfs.search(AA_ID, visited, 0, Some(score), 26);
                if elephant_dfs.best > self.best {
                    self.best.set(elephant_dfs.best.get());
                }
                return;
            }
        }
        for (next, flow) in self.flow_map.iter() {
            if *flow == 0 {
                continue;
            }
            let next_index = self.valve_map[next];
            if *visited.get(next_index as usize).unwrap() {
                continue;
            }
            let time_cost = self.travel_map[&(current_id, *next)] + 1;
            if time_cost >= time_left {
                continue;
            }
            let value = flow * (time_left - time_cost);
            self.search(
                *next,
                visited,
                score + value,
                other_score,
                time_left - time_cost,
            );
        }
        self.search(current_id, visited, score, other_score, 0);
    }
}

pub fn part_2(input: &str) -> u32 {
    let (mut flow_map, mut travel_map) = get_flow_and_travel_maps(input);
    expand_travel_map(&mut travel_map, &flow_map);
    let travel_map: HashMap<(Id, Id), u32> = travel_map
        .iter()
        .flat_map(|(&id_a, inner)| inner.iter().map(move |(id_b, time)| ((id_a, *id_b), *time)))
        .collect();
    let valve_map = flow_map
        .keys()
        .enumerate()
        .map(|(i, valve)| (*valve, i as u8))
        .collect();
    flow_map.retain(|_, v| *v != 0);
    let dfs = DfsFlowPlanPart2 {
        flow_map: &flow_map,
        travel_map: &travel_map,
        valve_map: &valve_map,
        best: Cell::new(0),
    };
    dfs.search(AA_ID, bitarr![0; 16], 0, None, 26);
    dfs.best.get()
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
