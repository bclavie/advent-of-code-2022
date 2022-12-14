use std::fmt::{Display, Write};

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/14");

#[time_run2("14")]
fn main() {
    regolith_reservoir(INPUT)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Material {
    Air,
    Sand,
    Rock,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => f.write_char('.'),
            Self::Sand => f.write_char('+'),
            Self::Rock => f.write_char('#'),
        }
    }
}

#[derive(Clone)]
struct Grid {
    inner: Vec<Vec<Material>>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Grid {
    fn from_input(i: &str) -> Self {
        let line_boundaries: Vec<Vec<(usize, usize)>> = i
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|coord| {
                        let (x, y) = coord.split_once(',').unwrap();
                        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                    })
                    .collect()
            })
            .collect();

        let (x_min, _) = *line_boundaries
            .iter()
            .flatten()
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap();

        let (x_max, _) = *line_boundaries
            .iter()
            .flatten()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap();

        let (_, y_max) = *line_boundaries
            .iter()
            .flatten()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        // Don't faff with trying to adjust the X index, just allocate a massive grid.
        // For part 2, the grid along the X axis might need to be massive.
        let mut grid: Vec<Vec<Material>> = Vec::with_capacity(2 * x_max + 2);
        for _ in 0..2 * x_max + 2 {
            grid.push(vec![Material::Air; y_max + 3])
        }

        // Draw the rocks
        for line_description in line_boundaries {
            for edge in line_description.windows(2) {
                let start = edge[0];
                let end = edge[1];

                // Vertical line
                if start.0 == end.0 {
                    let start_y = start.1.min(end.1);
                    let end_y = start.1.max(end.1);
                    for y in start_y..=end_y {
                        grid[start.0][y] = Material::Rock
                    }
                }

                // Horizontal line
                if start.1 == end.1 {
                    let start_x = start.0.min(end.0);
                    let end_x = start.0.max(end.0);
                    for x in grid.iter_mut().take(end_x + 1).skip(start_x) {
                        x[start.1] = Material::Rock
                    }
                }
            }
        }

        // Part2 Floor
        for x in &mut grid {
            x[y_max + 2] = Material::Rock
        }

        Self {
            inner: grid,
            x_min,
            x_max,
            y_max,
        }
    }

    fn steps_until_sand_falls_into_abyss(&mut self) -> u64 {
        let sand_spawn_position = (500, 0);
        let (mut sand_x_position, mut sand_y_position) = sand_spawn_position;
        let mut sand_spawned = 0;

        while sand_x_position >= self.x_min
            && sand_x_position <= self.x_max
            && sand_y_position <= self.y_max
        {
            debug_assert!(self.inner[sand_x_position][sand_y_position] == Material::Air);
            // Check down
            let down = self.inner[sand_x_position][sand_y_position + 1];
            if down == Material::Rock || down == Material::Sand {
                // Check diagonal left
                let diagonal_left = self.inner[sand_x_position - 1][sand_y_position + 1];
                if diagonal_left == Material::Rock || diagonal_left == Material::Sand {
                    // Check diagonal right
                    let diagonal_right = self.inner[sand_x_position + 1][sand_y_position + 1];
                    if diagonal_right == Material::Rock || diagonal_right == Material::Sand {
                        // Sand stays here
                        self.inner[sand_x_position][sand_y_position] = Material::Sand;

                        sand_spawned += 1;
                        (sand_x_position, sand_y_position) = sand_spawn_position;
                    } else {
                        sand_x_position += 1;
                        sand_y_position += 1;
                    }
                } else {
                    sand_x_position -= 1;
                    sand_y_position += 1;
                }
            } else {
                sand_y_position += 1;
            }
        }

        sand_spawned
    }

    // What is code reuse?
    fn steps_until_sand_stops_spawning(&mut self) -> u64 {
        let sand_spawn_position = (500, 0);
        let (mut sand_x_position, mut sand_y_position) = sand_spawn_position;
        let mut sand_spawned = 0;

        while self.inner[sand_spawn_position.0][sand_spawn_position.1] != Material::Sand {
            // Check down
            let down = self.inner[sand_x_position][sand_y_position + 1];
            if down == Material::Rock || down == Material::Sand {
                // Check diagonal left
                let diagonal_left = self.inner[sand_x_position - 1][sand_y_position + 1];
                if diagonal_left == Material::Rock || diagonal_left == Material::Sand {
                    // Check diagonal right
                    let diagonal_right = self.inner[sand_x_position + 1][sand_y_position + 1];
                    if diagonal_right == Material::Rock || diagonal_right == Material::Sand {
                        // Sand stays here
                        self.inner[sand_x_position][sand_y_position] = Material::Sand;

                        sand_spawned += 1;
                        (sand_x_position, sand_y_position) = sand_spawn_position;
                    } else {
                        sand_x_position += 1;
                        sand_y_position += 1;
                    }
                } else {
                    sand_x_position -= 1;
                    sand_y_position += 1;
                }
            } else {
                sand_y_position += 1;
            }
        }

        sand_spawned
    }
}

fn regolith_reservoir(i: &str) -> (String, String) {
    let mut grid = Grid::from_input(i);
    let mut grid2 = grid.clone();

    let part1 = grid.steps_until_sand_falls_into_abyss().to_string();
    let part2 = grid2.steps_until_sand_stops_spawning().to_string();

    (part1, part2)
}
