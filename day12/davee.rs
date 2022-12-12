use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;

fn node_type_to_height_map(ch: char) -> usize {
    if ch == 'S' {
        0
    } else if ch == 'E' {
        27
    } else {
        ch as usize - 'a' as usize + 1
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut graph: DiGraph<char, ()> = DiGraph::new();

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|node| (node_type_to_height_map(node), graph.add_node(node)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x_len = grid[0].len();
    let y_len = grid.len();

    for y in 0..y_len {
        for x in 0..x_len {
            let (cur_c, cur_node) = grid[y][x];

            // check adjacent +x
            if x + 1 < x_len {
                let (adj_c, adj_node) = grid[y][x + 1];

                if cur_c + 1 >= adj_c {
                    graph.add_edge(cur_node, adj_node, ());
                }
            }

            // check adjacent -x
            if x > 0 {
                let (adj_c, adj_node) = grid[y][x - 1];

                if cur_c + 1 >= adj_c {
                    graph.add_edge(cur_node, adj_node, ());
                }
            }

            // check adjacent +y
            if y + 1 < y_len {
                let (adj_c, adj_node) = grid[y + 1][x];

                if cur_c + 1 >= adj_c {
                    graph.add_edge(cur_node, adj_node, ());
                }
            }

            // check adjacent -y
            if y > 0 {
                let (adj_c, adj_node) = grid[y - 1][x];

                if cur_c + 1 >= adj_c {
                    graph.add_edge(cur_node, adj_node, ());
                }
            }
        }
    }

    let start = grid.iter().flatten().find(|(x, _)| *x == 0).unwrap().1;
    let end = grid.iter().flatten().find(|(x, _)| *x == 27).unwrap().1;

    let node_map = dijkstra(&graph, start, Some(end), |_| 1);

    println!("p1: {}", node_map[&end]);

    let mut min_rates = grid
        .iter()
        .flatten()
        .filter_map(|(x, start)| {
            (*x == 0 || *x == 1)
                .then(|| {
                    dijkstra(&graph, *start, Some(end), |_| 1)
                        .get(&end)
                        .cloned()
                })
                .flatten()
        })
        .collect::<Vec<_>>();

    min_rates.sort();
    println!("p2: {}", min_rates[0]);
}
