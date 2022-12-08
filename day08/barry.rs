use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/08");

#[time_run2("08")]
fn main() {
    visible_trees(INPUT)
}

#[derive(Clone)]
struct Tree {
    visible: bool,
    left_score: u64,
    right_score: u64,
    top_score: u64,
    bottom_score: u64,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            visible: Default::default(),
            left_score: Default::default(),
            right_score: Default::default(),
            top_score: Default::default(),
            bottom_score: Default::default(),
        }
    }
}

fn visible_trees(i: &str) -> (String, String) {
    let grid: Vec<Vec<i8>> = i
        .lines()
        .map(|line| {
            let l: Vec<i8> = line
                .chars()
                .map(|c| c.to_string().parse::<i8>().unwrap())
                .collect();
            l
        })
        .collect();

    let grid_y = grid.len();
    let grid_x = grid[0].len();
    let mut coloured_grid: Vec<Vec<Tree>> = Vec::with_capacity(grid_y);
    for _ in 0..grid_y {
        coloured_grid.push(vec![Tree::default(); grid_x]);
    }

    // Looking from the left
    for (j, trees) in grid.iter().enumerate() {
        let mut max_size = -1;
        let mut seen_tree_sizes: Vec<i8> = vec![];
        for (i, tree_size) in trees.iter().enumerate() {
            if tree_size > &max_size {
                max_size = *tree_size;
                coloured_grid[j][i].visible = true;
            }

            let mut left_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                left_score += 1;
                if tree_size <= size {
                    break;
                }
            }
            coloured_grid[j][i].left_score = left_score;

            seen_tree_sizes.push(*tree_size);
        }
    }

    // Looking from the right
    for (j, trees) in grid.iter().enumerate() {
        let mut max_size = -1;
        let mut seen_tree_sizes: Vec<i8> = vec![];

        for (i, tree_size) in trees.iter().enumerate().rev() {
            if tree_size > &max_size {
                max_size = *tree_size;
                coloured_grid[j][i].visible = true;
            }
            let mut right_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                right_score += 1;
                if tree_size <= size {
                    break;
                }
            }
            coloured_grid[j][i].right_score = right_score;
            seen_tree_sizes.push(*tree_size);
        }
    }

    // Top to bottom
    for i in 0..grid_x {
        let mut max_size = -1;
        let mut seen_tree_sizes: Vec<i8> = vec![];

        for j in 0..grid_y {
            let tree_size = grid[j][i];
            if tree_size > max_size {
                max_size = tree_size;
                coloured_grid[j][i].visible = true;
            }

            let mut top_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                top_score += 1;
                if tree_size <= *size {
                    break;
                }
            }
            coloured_grid[j][i].top_score = top_score;

            seen_tree_sizes.push(grid[j][i]);
        }
    }

    // Finally bottom to top
    for i in 0..grid_x {
        let mut max_size = -1;
        let mut seen_tree_sizes: Vec<i8> = vec![];
        for j in (0..grid_y).rev() {
            let tree_size = grid[j][i];
            if tree_size > max_size {
                max_size = tree_size;
                coloured_grid[j][i].visible = true;
            }
            let mut bottom_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                bottom_score += 1;
                if tree_size <= *size {
                    break;
                }
            }
            coloured_grid[j][i].bottom_score = bottom_score;

            seen_tree_sizes.push(grid[j][i]);
        }
    }

    // Part 1
    let mut total1 = 0;
    for line in coloured_grid.iter() {
        for tree in line {
            if tree.visible {
                total1 += 1;
            }
        }
    }

    let max_scenic_score = coloured_grid
        .iter()
        .flatten()
        .map(|tree| tree.left_score * tree.right_score * tree.top_score * tree.bottom_score)
        .max().unwrap();

    (total1.to_string(), max_scenic_score.to_string())
}
