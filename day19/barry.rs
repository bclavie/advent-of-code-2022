use aoc2022::time_run2;
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../inputs/19");

#[time_run2("19")]
fn main() {
    not_enough_minerals(INPUT)
}

fn not_enough_minerals(i: &str) -> (String, String) {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let initial_states: Vec<(u64, FactoryState)> = i
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                captures[1].parse::<u64>().unwrap(),
                FactoryState::new(
                    captures[2].parse::<u64>().unwrap(),
                    captures[3].parse::<u64>().unwrap(),
                    (
                        captures[4].parse::<u64>().unwrap(),
                        captures[5].parse::<u64>().unwrap(),
                    ),
                    (
                        captures[6].parse::<u64>().unwrap(),
                        captures[7].parse::<u64>().unwrap(),
                    ),
                ),
            )
        })
        .collect();

    let p2_input = initial_states.iter().take(3).collect_vec();

    let mut scores: Vec<u64> = vec![];

    for (blue_print, state) in &initial_states {
        let mut answer = Answer::new();
        answer.build_nodes_at(*state, 24);

        scores.push(answer.current_max * blue_print);
    }

    let part1 = scores.iter().sum::<u64>().to_string();

    let mut scores2 = vec![];

    for (_, state) in p2_input {
        let mut answer = Answer::new();
        answer.build_nodes_at(*state, 32);

        scores2.push(answer.current_max);
    }

    let part2 = scores2.iter().product::<u64>().to_string();

    (part1, part2)
}

#[derive(Debug, Copy, Clone)]
struct FactoryState {
    minutes: u64,
    ores: u64,
    clays: u64,
    obsidians: u64,
    geodes: u64,
    ore_machines: u64,
    clay_machines: u64,
    obsidian_machines: u64,
    geode_machines: u64,
    ore_machine_cost: u64,
    clay_machine_cost: u64,
    // Ore, Clay
    obsidian_machine_cost: (u64, u64),
    // Ore, Obsidian
    geode_machine_cost: (u64, u64),
}

impl FactoryState {
    fn new(
        ore_machine_cost: u64,
        clay_machine_cost: u64,
        obsidian_machine_cost: (u64, u64),
        geode_machine_cost: (u64, u64),
    ) -> Self {
        Self {
            minutes: 0,
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
            ore_machines: 1,
            clay_machines: 0,
            obsidian_machines: 0,
            geode_machines: 0,
            ore_machine_cost,
            clay_machine_cost,
            obsidian_machine_cost,
            geode_machine_cost,
        }
    }

    fn wait_x_minutes(&mut self, x: u64) {
        // Apply resource changes to get the resouces after x minutes
        self.minutes += x;
        self.ores += self.ore_machines * x;
        self.clays += self.clay_machines * x;
        self.obsidians += self.obsidian_machines * x;
        self.geodes += self.geode_machines * x;
    }

    // It's only worth plotting what the next possible decisions are given our
    // current generation rate.
    // This also assumes we only create 1 robot at each point in time.
    fn get_next_decision_points(&self) -> Vec<Decision> {
        let mut decisions = vec![];

        let minutes_for_next_ore_robot = if self.ores < self.ore_machine_cost {
            let mut m = (self.ore_machine_cost - self.ores) / self.ore_machines;
            if (self.ore_machine_cost - self.ores) % self.ore_machines != 0 {
                m += 1;
            }
            m
        } else {
            0
        };

        // We have to wait 1 minute **after** we have the resources before the robot is actually made.
        // Since resource collection happens prior to us being able to build a robot.
        decisions.push(Decision::Ore(minutes_for_next_ore_robot + 1));

        let minutes_for_next_clay_robot = if self.ores < self.clay_machine_cost {
            let mut m = (self.clay_machine_cost - self.ores) / self.ore_machines;
            if (self.clay_machine_cost - self.ores) % self.ore_machines != 0 {
                m += 1;
            }
            m
        } else {
            0
        };

        decisions.push(Decision::Clay(minutes_for_next_clay_robot + 1));

        // Only plot obsidian robot if we have any clay robots to generate stuff.
        if self.clay_machines > 0 {
            let mins_for_ore_for_obsidian_robot = if self.obsidian_machine_cost.0 > self.ores {
                let mut m = (self.obsidian_machine_cost.0 - self.ores) / self.ore_machines;
                if (self.obsidian_machine_cost.0 - self.ores) % self.ore_machines != 0 {
                    m += 1;
                }
                m
            } else {
                0
            };

            let mins_for_clay_for_obsidian_robot = if self.obsidian_machine_cost.1 > self.clays {
                let mut m = (self.obsidian_machine_cost.1 - self.clays) / self.clay_machines;
                if (self.obsidian_machine_cost.1 - self.clays) % self.clay_machines != 0 {
                    m += 1;
                }
                m
            } else {
                0
            };

            decisions.push(Decision::Obsidian(
                mins_for_clay_for_obsidian_robot.max(mins_for_ore_for_obsidian_robot) + 1,
            ))
        }

        // Only plot geode robot if we have any obsidian robots to generate stuff.
        if self.obsidian_machines > 0 {
            let mins_for_ore_for_geode_robot = if self.geode_machine_cost.0 > self.ores {
                let mut m = (self.geode_machine_cost.0 - self.ores) / self.ore_machines;
                if (self.geode_machine_cost.0 - self.ores) % self.ore_machines != 0 {
                    m += 1;
                }
                m
            } else {
                0
            };

            let mins_for_obsidian_for_geode_robot = if self.geode_machine_cost.1 > self.obsidians {
                let mut m = (self.geode_machine_cost.1 - self.obsidians) / self.obsidian_machines;
                if (self.geode_machine_cost.1 - self.obsidians) % self.obsidian_machines != 0 {
                    m += 1;
                }
                m
            } else {
                0
            };

            decisions.push(Decision::Geode(
                mins_for_obsidian_for_geode_robot.max(mins_for_ore_for_geode_robot) + 1,
            ))
        }

        decisions
    }
}

enum Decision {
    // The stored value is the number of minutes to wait in order to enact the decision.
    Ore(u64),
    Clay(u64),
    Obsidian(u64),
    Geode(u64),
}

#[derive(Debug, Copy, Clone)]
struct Answer {
    current_max: u64
}

impl Answer {
    fn new() -> Self {
        Answer { current_max: 0 }
    }

    fn build_nodes_at(&mut self, current: FactoryState, mins: u64) {
        let decisions = current.get_next_decision_points();
        // Decide what machines to build and then recursively go down each decision.
        for d in decisions {
            match d {
                Decision::Ore(mins_to_wait) => {
                    let mut next = current;
                    // Optimisation 1:
                    // If we have enough ore machines to build 1 machine per minute, stop building them.
                    if next.ore_machines
                        >= *[
                            next.ore_machine_cost,
                            next.clay_machine_cost,
                            next.obsidian_machine_cost.0,
                            next.geode_machine_cost.0,
                        ]
                        .iter()
                        .max()
                        .unwrap()
                    {
                        continue;
                    }

                    next.wait_x_minutes(mins_to_wait);
                    // If we have to go beyond the maximum time, don't bother recursing.
                    if next.minutes > mins {
                        continue;
                    }

                    next.ores = next.ores.checked_sub(next.ore_machine_cost).unwrap();
                    next.ore_machines += 1;
                    self.build_nodes_at(next, mins)
                }
                Decision::Clay(mins_to_wait) => {
                    let mut next = current;

                    // Optimisation 1:
                    // If we have enough clay machines to build 1 machine per minute, stop building them.
                    if next.clay_machines >= next.obsidian_machine_cost.1 {
                        continue;
                    }

                    next.wait_x_minutes(mins_to_wait);
                    if next.minutes > mins {
                        continue;
                    }

                    next.ores = next.ores.checked_sub(next.clay_machine_cost).unwrap();
                    next.clay_machines += 1;
                    self.build_nodes_at(next, mins)
                }
                Decision::Obsidian(mins_to_wait) => {
                    let mut next = current;
                    // Optimisation 1:
                    // If we have enough obsidian machines to build 1 machine per minute, stop building them.
                    if next.obsidian_machines >= next.geode_machine_cost.1 {
                        continue;
                    }

                    next.wait_x_minutes(mins_to_wait);
                    if next.minutes > mins {
                        continue;
                    }

                    next.ores = next.ores.checked_sub(next.obsidian_machine_cost.0).unwrap();
                    next.clays = next
                        .clays
                        .checked_sub(next.obsidian_machine_cost.1)
                        .unwrap();
                    next.obsidian_machines += 1;
                    self.build_nodes_at(next, mins)
                }
                Decision::Geode(mins_to_wait) => {
                    let mut next = current;
                    next.wait_x_minutes(mins_to_wait);
                    if next.minutes > mins {
                        continue;
                    }

                    next.ores = next.ores.checked_sub(next.geode_machine_cost.0).unwrap();
                    next.obsidians = next
                        .obsidians
                        .checked_sub(next.geode_machine_cost.1)
                        .unwrap();
                    next.geode_machines += 1;
                    self.build_nodes_at(next, mins)
                }
            }
        }

        // For each point, record what the max geode nodes would be after x minutes - only if a geode machine exists.
        let mut next = current;
        if next.geode_machines > 0 {
            let mins_to_wait = mins - next.minutes;
            next.wait_x_minutes(mins_to_wait);

            if next.geodes > self.current_max {
                self.current_max = next.geodes
            }
        }
    }
}
