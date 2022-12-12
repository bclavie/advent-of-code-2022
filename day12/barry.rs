use std::collections::{HashMap, VecDeque};

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/12");

#[time_run2("12")]
fn main() {
    hill_climbing(INPUT)
}

// Scuffed Graphs
#[derive(Debug)]
struct Graph {
    nodes: HashMap<(usize, usize), Node>,
}

#[derive(Debug)]
struct Node {
    coordinate: (usize, usize),
    elevation: char,
    neighbours: Vec<(usize, usize)>,
}

fn hill_climbing(i: &str) -> (String, String) {
    let chars: Vec<Vec<char>> = i.lines().map(|l| l.chars().collect()).collect();
    let (graph, starting_node, possible_starting_nodes) = build_graph(chars);

    let part1 = search_for_shortest_path(&graph, starting_node).unwrap();

    let part2 = possible_starting_nodes
        .into_iter()
        .filter_map(|coord| search_for_shortest_path(&graph, coord))
        .min()
        .unwrap();

    (part1.to_string(), part2.to_string())
}

fn is_elevation_reachable(a: char, b: char) -> bool {
    let test = if a == 'S' { 'a' } else { a };
    let other = if b == 'E' { 'z' } else { b };
    test as u32 + 1 >= other as u32
}

// BFS
fn search_for_shortest_path(graph: &Graph, starting_node: (usize, usize)) -> Option<u64> {
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    // Length of path so far, Coordinate of next node to look at.
    let mut queue: VecDeque<(u64, (usize, usize))> = VecDeque::new();

    let start = graph.nodes.get(&starting_node).unwrap();
    visited.insert(start.coordinate, true);
    queue.push_back((0, starting_node));

    while !queue.is_empty() {
        let (path_size, coord) = queue.pop_front().unwrap();
        let node = graph.nodes.get(&coord).unwrap();

        if node.elevation == 'E' {
            return Some(path_size);
        }

        for neighbour in &node.neighbours {
            let neighbour_node = graph.nodes.get(neighbour).unwrap();

            if visited.get(&neighbour_node.coordinate).is_none() {
                visited.insert(neighbour_node.coordinate, true);
                queue.push_back((path_size + 1, *neighbour));
            };
        }
    }

    None
}

fn build_graph(map: Vec<Vec<char>>) -> (Graph, (usize, usize), Vec<(usize, usize)>) {
    let mut starting_node = (0, 0);
    let mut possible_starting_nodes = Vec::new();
    let mut nodes = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut node = Node {
                coordinate: (y, x),
                elevation: map[y][x],
                neighbours: vec![],
            };
            if node.elevation == 'S' {
                starting_node = node.coordinate;
                possible_starting_nodes.push(node.coordinate);
            }
            if node.elevation == 'a' {
                possible_starting_nodes.push(node.coordinate)
            }
            // check Left
            if x > 0 && is_elevation_reachable(map[y][x], map[y][x - 1]) {
                node.neighbours.push((y, x - 1))
            }

            // check right
            if x < map[0].len() - 1 && is_elevation_reachable(map[y][x], map[y][x + 1]) {
                node.neighbours.push((y, x + 1))
            }

            // check up
            if y > 0 && is_elevation_reachable(map[y][x], map[y - 1][x]) {
                node.neighbours.push((y - 1, x))
            }

            // check down
            if y < map.len() - 1 && is_elevation_reachable(map[y][x], map[y + 1][x]) {
                node.neighbours.push((y + 1, x))
            }

            nodes.insert((y, x), node);
        }
    }

    (Graph { nodes }, starting_node, possible_starting_nodes)
}
