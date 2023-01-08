use derive_more::{Add, AddAssign, From, SubAssign};
use scan_fmt::scan_fmt;
use std::{cmp, collections::HashMap};
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

struct Bp {
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

impl Bp {
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
fn play(
    bp: &Bp,
    mut state: State,
    time: u16,
    best: &mut Geo,
    memo: &mut HashMap<(u16, State), Geo>,
) {
    if time == 0 {
        *best = cmp::max(*best, state.geo);
        return;
    }
    //collect
    state.ore += state.ore_bot;
    state.clay += state.clay_bot;
    state.obs += state.obs_bot;
    state.geo += state.geo_bot;
    if time < CACHE_TIME && memo.contains_key(&(time, state)) {
        return;
    }
    // try all spends
    //GeoBot
    if state.ore >= bp.geo_bot_cost.0 + state.ore_bot
        && state.obs >= bp.geo_bot_cost.1 + state.obs_bot
    {
        let mut new_state = state;
        new_state.ore -= bp.geo_bot_cost.0;
        new_state.obs -= bp.geo_bot_cost.1;
        new_state.geo_bot += 1.into();
        play(bp, new_state, time - 1, best, memo);
        // cache best
        if time < CACHE_TIME {
            memo.insert((time, state), *best);
        }
        return;
    }

    // ObsBot
    if state.ore >= bp.obs_bot_cost.0 + state.ore_bot
        && state.clay >= bp.obs_bot_cost.1 + state.clay_bot
        && state.obs_bot < bp.geo_bot_cost.1
    {
        let mut new_state = state;
        new_state.ore -= bp.obs_bot_cost.0;
        new_state.clay -= bp.obs_bot_cost.1;
        new_state.obs_bot += 1.into();
        play(bp, new_state, time - 1, best, memo);
    }
    //ClayBot
    if state.ore >= bp.clay_bot_cost + state.ore_bot && state.clay_bot < bp.obs_bot_cost.1 {
        let mut new_state = state;
        new_state.ore -= bp.clay_bot_cost;
        new_state.clay_bot += 1.into();
        play(bp, new_state, time - 1, best, memo);
    }
    // OreBot
    let max_ore_cost = bp
        .clay_bot_cost
        .max(bp.obs_bot_cost.0.max(bp.geo_bot_cost.0));
    if state.ore >= bp.ore_bot_cost + state.ore_bot && state.ore_bot < max_ore_cost {
        let mut new_state = state;
        new_state.ore -= bp.ore_bot_cost;
        new_state.ore_bot += 1.into();
        play(bp, new_state, time - 1, best, memo);
    }
    // No bot built
    play(bp, state, time - 1, best, memo);
    // cache best
    if time < CACHE_TIME {
        memo.insert((time, state), *best);
    }
}

pub fn part_1(input: &str) -> usize {
    let blueprints: Vec<_> = input.lines().map(Bp::parse).collect();
    blueprints
        .iter()
        .enumerate()
        .map(|(id, bp)| {
            let mut best = 0.into();
            let mut memo = HashMap::new();
            play(
                bp,
                State {
                    ore_bot: 1.into(),
                    ..Default::default()
                },
                24,
                &mut best,
                &mut memo,
            );
            (id + 1) * best.0 as usize
        })
        .sum()
}
pub fn part_2(input: &str) -> usize {
    let blueprints: Vec<_> = input.lines().map(Bp::parse).collect();
    blueprints
        .iter()
        .take(3)
        .map(|bp| {
            let mut best = 0.into();
            let mut memo = HashMap::new();
            play(
                bp,
                State {
                    ore_bot: 1.into(),
                    ..Default::default()
                },
                32,
                &mut best,
                &mut memo,
            );
            best.0 as usize
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
