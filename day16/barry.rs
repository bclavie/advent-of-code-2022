use std::{
    collections::{HashMap, HashSet},
    thread::JoinHandle,
};

use aoc2022::time_run2;
use itertools::{Combinations, Itertools};
use regex::Regex;
use std::thread;

const INPUT: &str = include_str!("../inputs/16");

#[time_run2("16")]
fn main() {
    proboscidea_volcanium(INPUT)
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
struct Node {
    label: String,
    flow_rate: u64,
    // Label - Path length
    neighbours: Vec<(String, u64)>,
}

// Creates a new graph with the shortest paths between each node.
fn dijkstraed_graph(g: Graph) -> Graph {
    let mut new_nodes = HashMap::new();

    for node in g.nodes.values() {
        let distances = dijkstra(&g, node.clone());
        let mut new_node = Node {
            label: node.label.clone(),
            flow_rate: node.flow_rate,
            neighbours: vec![],
        };

        for (neighbour, dist) in distances {
            new_node.neighbours.push((neighbour, dist))
        }

        new_nodes.insert(new_node.label.clone(), new_node);
    }

    Graph { nodes: new_nodes }
}

fn dijkstra(g: &Graph, source: Node) -> HashMap<String, u64> {
    let mut distances = HashMap::new();
    let mut q = vec![];

    for (label, node) in &g.nodes {
        distances.insert(label.clone(), u64::MAX);
        q.push(node.clone());
    }
    distances.insert(source.label, 0);

    while !q.is_empty() {
        let mut min_node = (q[0].label.clone(), u64::MAX);
        // Find the vertex with the minimum distance from the start.
        for n in &q {
            let dist = distances.get(&n.label).unwrap();
            if dist < &min_node.1 {
                min_node = (n.label.clone(), *dist);
            }
        }

        // Remove from q
        let (index, _) = q
            .iter()
            .enumerate()
            .find(|(_, node)| node.label == min_node.0)
            .unwrap();
        let node = q.swap_remove(index);

        for neighbour in node.neighbours {
            match q.iter().find(|node| node.label == neighbour.0) {
                Some(node) => {
                    let existing_distance_to_neighbour = distances.get(&node.label).unwrap();
                    let new_distance_to_neighbour = min_node.1 + neighbour.1;
                    if new_distance_to_neighbour < *existing_distance_to_neighbour {
                        distances.insert(neighbour.0, new_distance_to_neighbour);
                    }
                }
                None => continue,
            }
        }
    }

    distances
}

// Takes a graph and simplifies it, leaving only nodes which have a positive flow rate.
fn zero_out_graph(g: Graph) -> Graph {
    let nodes: Vec<Node> = g.nodes.into_iter().map(|(_, v)| v).collect();
    let mut new_nodes = vec![];
    let mut neighbours_to_remove = vec![];
    for node in nodes {
        if node.flow_rate == 0 && node.label != "AA" {
            neighbours_to_remove.push(node.label.clone());
        } else {
            new_nodes.push(node)
        }
    }

    for node in new_nodes.iter_mut() {
        let mut new_neighbours = vec![];
        for neighbour in &node.neighbours {
            if !neighbours_to_remove.contains(&neighbour.0) {
                new_neighbours.push(neighbour.clone())
            }
        }
        node.neighbours = new_neighbours
    }

    Graph {
        nodes: new_nodes
            .into_iter()
            .map(|n| (n.label.clone(), n))
            .collect(),
    }
}

// Assumes g is a fully dijkstra'd graph.
fn calculate_max_pressures(g: Graph) -> u64 {
    let start = g.nodes.get("AA").unwrap();
    let mut max_pressure = 0;
    let minutes_remaining = 30;
    let mut visited_nodes: HashSet<String> = HashSet::new();
    visited_nodes.insert(start.label.clone());

    // You spend 1 minute turning on each valve. For each movement to a node, you need
    // to check if it's worth visiting that node or another first. We can do this recursively.
    for neighbour in &start.neighbours {
        let pressure_for_node = calculate_max_pressure_if_node_visited(
            &g,
            neighbour.0.clone(),
            minutes_remaining - neighbour.1 as i32,
            visited_nodes.clone(),
        );
        if pressure_for_node > max_pressure {
            max_pressure = pressure_for_node
        }
    }

    max_pressure
}

fn calculate_max_pressure_if_node_visited(
    g: &Graph,
    node_label: String,
    minutes_remaining: i32,
    mut visited_nodes: HashSet<String>,
) -> u64 {
    visited_nodes.insert(node_label.clone());

    let node = g.nodes.get(&node_label).unwrap();
    // calculate the pressure for turning on this node then add it to the max potential of other nodes.
    if minutes_remaining <= 1 {
        return 0;
    }
    let pressure_for_this_node = (minutes_remaining - 1) as u64 * node.flow_rate;

    let mut max_pressure_for_other_nodes = 0;
    for neighbour in &node.neighbours {
        // Don't bother going to other nodes if already visited.
        if visited_nodes.contains(&neighbour.0) {
            continue;
        }
        let pressure_for_node = calculate_max_pressure_if_node_visited(
            g,
            neighbour.0.clone(),
            minutes_remaining - 1 - neighbour.1 as i32,
            visited_nodes.clone(),
        );
        if pressure_for_node > max_pressure_for_other_nodes {
            max_pressure_for_other_nodes = pressure_for_node
        }
    }

    pressure_for_this_node + max_pressure_for_other_nodes
}

// Assumes g is a fully dijkstra'd graph.
fn calculate_max_pressures_with_elephant(g: Graph) -> u64 {
    let start = g.nodes.get("AA").unwrap();
    let mut max_pressure = 0;
    let mut visited_nodes: HashSet<String> = HashSet::new();
    visited_nodes.insert(start.label.clone());

    // For each node in the graph, either assign it to yourself or the elephant
    // And do the same algorithm in p1 for each and then sum the two.
    // Effectively we split the graph into two subgraphs and run the same algorithm
    let nodes: Vec<String> = g
        .nodes
        .iter()
        .filter_map(|(k, _)| if k != "AA" { Some(k.clone()) } else { None })
        .collect();

    // We can cheat a bit and multithread this.
    let mut handles: Vec<(usize, usize, JoinHandle<u64>)> = vec![];
    // We only need to do combinations up to len/2, otherwise we end up repeating combinations.
    let len = nodes.len() / 2;
    for i in 1..=len {
        let combinations = nodes.clone().into_iter().combinations(i);
        let handle = get_handle_for_combinations(
            combinations,
            start.clone(),
            g.clone(),
            nodes.clone(),
            visited_nodes.clone(),
        );
        handles.push((i, nodes.len() - i, handle));
    }

    for handle in handles {
        let result = handle.2.join().unwrap();
        println!(
            "finished combinations for {} {}: {}",
            handle.0, handle.1, result
        );
        if result > max_pressure {
            max_pressure = result
        }
    }

    max_pressure
}

// Need this func due to Rust's ownership rules & their interaction with threads.
fn get_handle_for_combinations(
    combinations: Combinations<std::vec::IntoIter<String>>,
    start: Node,
    graph: Graph,
    nodes: Vec<String>,
    visited_nodes: HashSet<String>,
) -> JoinHandle<u64> {
    thread::spawn(move || {
        let mut max_for_combinations = 0;
        for my_nodes in combinations {
            let mut my_max_pressure = 0;
            let mut elephant_max_presure = 0;
            let my_node_set: HashSet<String> = my_nodes.into_iter().collect();
            let elephant_node_set: HashSet<String> = nodes
                .iter()
                .filter_map(|n| {
                    if !my_node_set.contains(n) {
                        Some(n.clone())
                    } else {
                        None
                    }
                })
                .collect();

            // My subgraph
            for neighbour in &start.neighbours {
                // Don't go to the node if it is not in our subgraph.
                if !my_node_set.contains(&neighbour.0) {
                    continue;
                }
                let pressure_for_node = calculate_max_pressure_if_node_visited_with_elephant(
                    &graph,
                    neighbour.0.clone(),
                    26 - neighbour.1 as i32,
                    visited_nodes.clone(),
                    &my_node_set,
                );
                if pressure_for_node > my_max_pressure {
                    my_max_pressure = pressure_for_node
                }
            }

            // Elephant subgraph
            for neighbour in &start.neighbours {
                // Don't go to the node if it is not in the elephant's subgraph.
                if !elephant_node_set.contains(&neighbour.0) {
                    continue;
                }
                let pressure_for_node = calculate_max_pressure_if_node_visited_with_elephant(
                    &graph,
                    neighbour.0.clone(),
                    26 - neighbour.1 as i32,
                    visited_nodes.clone(),
                    &elephant_node_set,
                );
                if pressure_for_node > elephant_max_presure {
                    elephant_max_presure = pressure_for_node
                }
            }

            if my_max_pressure + elephant_max_presure > max_for_combinations {
                max_for_combinations = my_max_pressure + elephant_max_presure
            }
        }
        max_for_combinations
    })
}

// Same as part1 - except we only get given a subgraph to traverse.
fn calculate_max_pressure_if_node_visited_with_elephant(
    g: &Graph,
    node_label: String,
    minutes_remaining: i32,
    mut visited_nodes: HashSet<String>,
    subgraph: &HashSet<String>,
) -> u64 {
    visited_nodes.insert(node_label.clone());

    let node = g.nodes.get(&node_label).unwrap();
    // calculate the pressure for turning on this node then add it to the max potential of other nodes.
    if minutes_remaining <= 1 {
        return 0;
    }
    let pressure_for_this_node = (minutes_remaining - 1) as u64 * node.flow_rate;

    let mut max_pressure_for_other_nodes = 0;
    for neighbour in &node.neighbours {
        // Don't bother going to other nodes if already visited.
        if visited_nodes.contains(&neighbour.0) {
            continue;
        }
        // Don't go to the node if it is not in our subgraph.
        if !subgraph.contains(&neighbour.0) {
            continue;
        }
        let pressure_for_node = calculate_max_pressure_if_node_visited_with_elephant(
            g,
            neighbour.0.clone(),
            minutes_remaining - 1 - neighbour.1 as i32,
            visited_nodes.clone(),
            subgraph,
        );
        if pressure_for_node > max_pressure_for_other_nodes {
            max_pressure_for_other_nodes = pressure_for_node
        }
    }

    pressure_for_this_node + max_pressure_for_other_nodes
}

fn proboscidea_volcanium(i: &str) -> (String, String) {
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
        .unwrap();

    let mut graph = Graph {
        nodes: HashMap::new(),
    };

    for line in i.lines() {
        let captures = re.captures(line).unwrap();
        let n = Node {
            label: captures[1].to_string(),
            flow_rate: captures[2].parse::<u64>().unwrap(),
            neighbours: captures[3]
                .split(", ")
                .into_iter()
                .map(|s| (s.to_string(), 1))
                .collect(),
        };

        graph.nodes.insert(n.label.clone(), n);
    }
    let d_graph = dijkstraed_graph(graph);
    let d_graph = zero_out_graph(d_graph);

    let part1 = calculate_max_pressures(d_graph.clone());
    let part2 = calculate_max_pressures_with_elephant(d_graph);

    (part1.to_string(), part2.to_string())
}
