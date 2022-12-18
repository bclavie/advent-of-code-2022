use std::collections::{HashMap, HashSet, VecDeque};

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/18");

#[time_run2("18")]
fn main() {
    boiling_boulders(INPUT)
}

fn boiling_boulders(i: &str) -> (String, String) {
    let mut grid: HashMap<(i64, i64, i64), Cube> = i
        .lines()
        .map(|line| {
            let mut vals = line.split(',').map(|s| s.parse::<i64>().unwrap());
            let (x, y, z) = (
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            );
            let cube = Cube::new(x, y, z);
            ((x, y, z), cube)
        })
        .collect();
    let mut grid2 = grid.clone();

    // Take 1 exposed side away from each neighbour.
    for cube in grid.clone().values() {
        for x in [cube.x - 1, cube.x + 1] {
            if let Some(touching_cube) = grid.get_mut(&(x, cube.y, cube.z)) {
                touching_cube.exposed_sides -= 1;
            }
        }

        for y in [cube.y - 1, cube.y + 1] {
            if let Some(touching_cube) = grid.get_mut(&(cube.x, y, cube.z)) {
                touching_cube.exposed_sides -= 1;
            }
        }

        for z in [cube.z - 1, cube.z + 1] {
            if let Some(touching_cube) = grid.get_mut(&(cube.x, cube.y, z)) {
                touching_cube.exposed_sides -= 1;
            }
        }
    }

    let part1 = grid.values().fold(0, |acc, c| acc + c.exposed_sides);

    p2(&mut grid2);
    let part2 = grid2.values().fold(0, |acc, c| acc + c.exposed_sides);

    (part1.to_string(), part2.to_string())
}

// For part 2 - reverse the problem.
// Do BFS based colouring from outside the grid using
// empty blocks.
fn p2(grid: &mut HashMap<(i64, i64, i64), Cube>) {
    for v in grid.values_mut() {
        // Set all the exposed sides to be 0 initially.
        v.exposed_sides = 0;
    }
    let min_x = grid.values().map(|v| v.x).min().unwrap() - 1;
    let min_y = grid.values().map(|v| v.y).min().unwrap() - 1;
    let min_z = grid.values().map(|v| v.z).min().unwrap() - 1;
    let max_x = grid.values().map(|v| v.x).max().unwrap() + 1;
    let max_y = grid.values().map(|v| v.y).max().unwrap() + 1;
    let max_z = grid.values().map(|v| v.z).max().unwrap() + 1;

    let start = (min_x, min_y, min_z);
    let mut queue = VecDeque::<(i64, i64, i64)>::new();
    let mut visited = HashSet::<(i64, i64, i64)>::new();

    queue.push_back(start);
    visited.insert(start);
    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();

        for x in [coord.0 - 1, coord.0 + 1] {
            if x > max_x || x < min_x {
                continue;
            }
            match grid.get_mut(&(x, coord.1, coord.2)) {
                Some(touching_cube) => {
                    touching_cube.exposed_sides += 1;
                }
                None => {
                    if !visited.contains(&(x, coord.1, coord.2)) {
                        queue.push_back((x, coord.1, coord.2));
                        visited.insert((x, coord.1, coord.2));
                    }
                }
            }
        }

        for y in [coord.1 - 1, coord.1 + 1] {
            if y > max_y || y < min_y {
                continue;
            }
            match grid.get_mut(&(coord.0, y, coord.2)) {
                Some(touching_cube) => {
                    touching_cube.exposed_sides += 1;
                }
                None => {
                    if !visited.contains(&(coord.0, y, coord.2)) {
                        queue.push_back((coord.0, y, coord.2));
                        visited.insert((coord.0, y, coord.2));
                    }
                }
            }
        }

        for z in [coord.2 - 1, coord.2 + 1] {
            if z > max_z || z < min_z {
                continue;
            }
            match grid.get_mut(&(coord.0, coord.1, z)) {
                Some(touching_cube) => {
                    touching_cube.exposed_sides += 1;
                }
                None => {
                    if !visited.contains(&(coord.0, coord.1, z)) {
                        queue.push_back((coord.0, coord.1, z));
                        visited.insert((coord.0, coord.1, z));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
    exposed_sides: u64,
}

impl Cube {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
            exposed_sides: 6,
        }
    }
}
