use scan_fmt::scan_fmt;
use std::{cmp, collections::HashMap};
struct ObsBotCost {
    ore: u16,
    clay: u16,
}
struct GeoBotCost {
    ore: u16,
    obs: u16,
}
struct Obs(u16);
struct Bp {
    ore_bot_cost: u16,  //Ore
    clay_bot_cost: u16, //Ore
    obs_bot_cost: ObsBotCost,
    geo_bot_cost: GeoBotCost,
}
#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    ore_bot: u16,
    clay_bot: u16,
    obs_bot: u16,
    geo_bot: u16,
    clay: u16,
    ore: u16,
    obs: u16,
    geo: u16,
}

impl Bp {
    fn parse(s: &str) -> Self {
        let (_id, ob_cost, cb_cost, obs_cost_ore, obs_cost_clay, geo_cost_ore, geo_cost_obs) =
            scan_fmt!(s, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", u16,u16, u16, u16, u16, u16, u16).unwrap();
        Self {
            ore_bot_cost: ob_cost,
            clay_bot_cost: cb_cost,
            obs_bot_cost: ObsBotCost {
                ore: obs_cost_ore,
                clay: obs_cost_clay,
            },
            geo_bot_cost: GeoBotCost {
                ore: geo_cost_ore,
                obs: geo_cost_obs,
            },
        }
    }
}
// NOTE: Seems to have no gains over ~15, weird.
const CACHE_TIME: u16 = 15;
fn play(
    bp: &Bp,
    mut state: State,
    time: u16,
    best: &mut u16,
    memo: &mut HashMap<(u16, State), u16>,
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
    if state.ore >= bp.geo_bot_cost.ore + state.ore_bot
        && state.obs >= bp.geo_bot_cost.obs + state.obs_bot
    {
        let mut new_state = state;
        new_state.ore -= bp.geo_bot_cost.ore;
        new_state.obs -= bp.geo_bot_cost.obs;
        new_state.geo_bot += 1;
        play(bp, new_state, time - 1, best, memo);
        // cache best
        if time < CACHE_TIME {
            memo.insert((time, state), *best);
        }
        return;
    }

    // ObsBot
    if state.ore >= bp.obs_bot_cost.ore + state.ore_bot
        && state.clay >= bp.obs_bot_cost.clay + state.clay_bot
        && state.obs_bot < bp.geo_bot_cost.obs
    {
        let mut new_state = state;
        new_state.ore -= bp.obs_bot_cost.ore;
        new_state.clay -= bp.obs_bot_cost.clay;
        new_state.obs_bot += 1;
        play(bp, new_state, time - 1, best, memo);
    }
    //ClayBot
    if state.ore >= bp.clay_bot_cost + state.ore_bot && state.clay_bot < bp.obs_bot_cost.clay {
        let mut new_state = state;
        new_state.ore -= bp.clay_bot_cost;
        new_state.clay_bot += 1;
        play(bp, new_state, time - 1, best, memo);
    }
    // OreBot
    let max_ore_cost = bp
        .clay_bot_cost
        .max(bp.obs_bot_cost.ore.max(bp.geo_bot_cost.ore));
    if state.ore >= bp.ore_bot_cost + state.ore_bot && state.ore_bot < max_ore_cost {
        let mut new_state = state;
        new_state.ore -= bp.ore_bot_cost;
        new_state.ore_bot += 1;
        play(bp, new_state, time - 1, best, memo);
    }
    // No bot built
    play(bp, state, time - 1, best, memo);
    // cache best
    if time < CACHE_TIME {
        memo.insert((time, state), *best);
    }
}

fn part_1(input: &str) -> usize {
    let blueprints: Vec<_> = input.lines().map(Bp::parse).collect();
    blueprints
        .iter()
        .enumerate()
        .map(|(id, bp)| {
            let mut best = 0;
            let mut memo = HashMap::new();
            play(
                bp,
                State {
                    ore_bot: 1,
                    ..Default::default()
                },
                24,
                &mut best,
                &mut memo,
            );
            (id + 1) * best as usize
        })
        .sum()
}
fn part_2(input: &str) -> usize {
    let blueprints: Vec<_> = input.lines().map(Bp::parse).collect();
    blueprints
        .iter()
        .take(3)
        .map(|bp| {
            let mut best = 0;
            let mut memo = HashMap::new();
            play(
                bp,
                State {
                    ore_bot: 1,
                    ..Default::default()
                },
                32,
                &mut best,
                &mut memo,
            );
            best as usize
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
