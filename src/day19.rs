use derive_more::{Add, AddAssign, From, SubAssign};
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
#[derive(
    Clone, Copy, Default, Hash, PartialEq, Eq, From, AddAssign, Add, SubAssign, PartialOrd, Ord,
)]
struct Ore(u16);
#[derive(
    Clone, Copy, Default, Hash, PartialEq, Eq, From, AddAssign, Add, SubAssign, PartialOrd, Ord,
)]
struct Clay(u16);
#[derive(
    Clone, Copy, Default, Hash, PartialEq, Eq, From, AddAssign, Add, SubAssign, PartialOrd, Ord,
)]
struct Obs(u16);
#[derive(
    Clone, Copy, Default, Hash, PartialEq, Eq, From, AddAssign, Add, SubAssign, PartialOrd, Ord,
)]
struct Geo(u16);

struct Blueprint {
    ore_bot_cost: Ore,
    clay_bot_cost: Ore,
    obs_bot_cost: (Ore, Clay),
    geo_bot_cost: (Ore, Obs),
}
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore_bot: Ore,
    clay_bot: Clay,
    obs_bot: Obs,
    geo_bot: Geo,
    ore: Ore,
    clay: Clay,
    obs: Obs,
    geo: Geo,
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let (_id, ob_cost, cb_cost, obs_cost_ore, obs_cost_clay, geo_cost_ore, geo_cost_obs) =
            scan_fmt!(s, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", u16,u16, u16, u16, u16, u16, u16).unwrap();
        Self {
            ore_bot_cost: Ore(ob_cost),
            clay_bot_cost: Ore(cb_cost),
            obs_bot_cost: (Ore(obs_cost_ore), Clay(obs_cost_clay)),
            geo_bot_cost: (Ore(geo_cost_ore), Obs(geo_cost_obs)),
        }
    }
}
// NOTE: Seems to have no gains over ~15, weird.
const CACHE_TIME: u16 = 15;
struct DepthSearcher {
    bp: Blueprint,
    visited: HashSet<(u16, State)>,
    best: Geo,
    max_costs: (Ore, Clay, Obs),
}
impl DepthSearcher {
    fn new(bp: Blueprint) -> Self {
        let max_ore_cost = bp
            .clay_bot_cost
            .max(bp.obs_bot_cost.0.max(bp.geo_bot_cost.0));
        let max_costs = (max_ore_cost, bp.obs_bot_cost.1, bp.geo_bot_cost.1);
        Self {
            bp,
            visited: HashSet::new(),
            best: Geo(0),
            max_costs,
        }
    }
    fn search(&mut self, mut state: State, time: u16) {
        if time == 0 {
            self.best = self.best.max(state.geo);
            return;
        }
        // Collect
        state.ore += state.ore_bot;
        state.clay += state.clay_bot;
        state.obs += state.obs_bot;
        state.geo += state.geo_bot;
        if time < CACHE_TIME && self.visited.contains(&(time, state)) {
            return;
        }
        // Try building GeoBot
        if state.ore >= self.bp.geo_bot_cost.0 + state.ore_bot
            && state.obs >= self.bp.geo_bot_cost.1 + state.obs_bot
        {
            let mut new_state = state;
            new_state.ore -= self.bp.geo_bot_cost.0;
            new_state.obs -= self.bp.geo_bot_cost.1;
            new_state.geo_bot += 1.into();
            self.search(new_state, time - 1);
            // cache best
            if time < CACHE_TIME {
                self.visited.insert((time, state));
            }
            // Early return; building GeoBot is always right if possible.
            return;
        }
        // Try building ObsBot
        if state.ore >= self.bp.obs_bot_cost.0 + state.ore_bot
            && state.clay >= self.bp.obs_bot_cost.1 + state.clay_bot
            && state.obs_bot < self.max_costs.2
        {
            let mut new_state = state;
            new_state.ore -= self.bp.obs_bot_cost.0;
            new_state.clay -= self.bp.obs_bot_cost.1;
            new_state.obs_bot += 1.into();
            self.search(new_state, time - 1);
        }
        // Try building ClayBot
        if state.ore >= self.bp.clay_bot_cost + state.ore_bot && state.clay_bot < self.max_costs.1 {
            let mut new_state = state;
            new_state.ore -= self.bp.clay_bot_cost;
            new_state.clay_bot += 1.into();
            self.search(new_state, time - 1);
        }
        // Try building OreBot
        if state.ore >= self.bp.ore_bot_cost + state.ore_bot && state.ore_bot < self.max_costs.0 {
            let mut new_state = state;
            new_state.ore -= self.bp.ore_bot_cost;
            new_state.ore_bot += 1.into();
            self.search(new_state, time - 1);
        }
        // Try building no bot
        self.search(state, time - 1);
        // cache best
        if time < CACHE_TIME {
            self.visited.insert((time, state));
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let blueprints = input.lines().map(Blueprint::parse);
    blueprints
        .into_iter()
        .enumerate()
        .map(|(id, bp)| {
            let mut dfs = DepthSearcher::new(bp);
            dfs.search(
                State {
                    ore_bot: 1.into(),
                    ..Default::default()
                },
                24,
            );
            (id + 1) * dfs.best.0 as usize
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let blueprints = input.lines().map(Blueprint::parse);
    blueprints
        .into_iter()
        .take(3)
        .map(|bp| {
            let mut dfs = DepthSearcher::new(bp);
            dfs.search(
                State {
                    ore_bot: 1.into(),
                    ..Default::default()
                },
                32,
            );
            dfs.best.0 as usize
        })
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day19::*;
    use std::fs;
    #[test]
    fn example19_part1() {
        let input = fs::read_to_string("input/example19").unwrap();
        assert_eq!(part_1(&input), 33);
    }
    #[test]
    fn day19_part1() {
        let input = fs::read_to_string("input/day19").unwrap();
        assert_eq!(part_1(&input), 1958);
    }
    #[test]
    fn example19_part2() {
        let input = fs::read_to_string("input/example19").unwrap();
        assert_eq!(part_2(&input), 56 * 62);
    }
    #[test]
    fn day19_part2() {
        let input = fs::read_to_string("input/day19").unwrap();
        assert_eq!(part_2(&input), 4257);
    }
}
