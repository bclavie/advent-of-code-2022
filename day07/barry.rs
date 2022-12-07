use std::collections::HashMap;

use aoc2022::time_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/07");

#[time_run("07")]
fn main() {
    directory(INPUT)
}

#[derive(Debug)]
enum Node {
    File(File),
    Dir(Dir),
}

impl Node {
    fn absolute_path(&self) -> String {
        match self {
            Node::File(f) => f.absolute_path.clone(),
            Node::Dir(d) => d.absolute_path.clone(),
        }
    }

    fn size(&self, tree: &Tree) -> u64 {
        match self {
            Node::File(f) => f.size,
            Node::Dir(d) => d.size(tree),
        }
    }
}

// Actual trees structures are mega scuffed in Rust.
#[derive(Debug)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    pub fn insert_node(&mut self, absolute_path: String, n: Node) {
        self.nodes.insert(absolute_path, n);
    }

    pub fn get_dir_sizes(&self) -> Vec<(String, u64)> {
        let mut sizes = vec![];
        for node in self.nodes.values() {
            match node {
                Node::File(_) => continue,
                Node::Dir(d) => sizes.push((d.absolute_path.clone(), d.size(self))),
            }
        }
        sizes
    }
}

#[derive(Debug)]
struct File {
    absolute_path: String,
    size: u64,
}

#[derive(Debug)]
struct Dir {
    absolute_path: String,
    parent_path: Option<String>,
    childs: Vec<String>,
}

impl Dir {
    fn new(path: String, parent_path: Option<String>) -> Self {
        Self {
            absolute_path: match parent_path {
                Some(ref parent_path) => format!("{}/{}", parent_path.clone(), path),
                None => "".to_string(),
            },
            parent_path,
            childs: vec![],
        }
    }

    fn push_child(&mut self, child_path: String) {
        self.childs.push(child_path);
    }
}

impl Dir {
    fn size(&self, tree: &Tree) -> u64 {
        self.childs
            .iter()
            .fold(0, |acc, i| match &tree.nodes.get(i).unwrap() {
                Node::File(f) => acc + f.size,
                Node::Dir(d) => acc + d.size(tree),
            })
    }
}

fn directory(i: &str) -> String {
    let mut lines = i.lines();

    let mut tree = Tree {
        nodes: HashMap::new(),
    };
    let mut current_node_path: Option<String> = None;

    // Mega scuffed input parsing
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let dir_name = line.split_once("cd ").unwrap().1;

            if dir_name == ".." {
                let current_node = tree.nodes.get(&current_node_path.unwrap()).unwrap();
                // Move to the parent.
                current_node_path = match current_node {
                    Node::File(_) => panic!("unexpected node type"),
                    Node::Dir(d) => {
                        Some(d.parent_path.as_ref().expect("dir has no parent").clone())
                    }
                }
            } else {
                // Create new node for the directory.
                let mut dir = Dir::new(dir_name.to_string(), current_node_path.clone());

                // An ls always follows a cd to a new directory.
                let ls = lines.next().unwrap();
                debug_assert!(ls.starts_with("$ ls"));

                let ls_input = lines.take_while_ref(|l| !l.starts_with('$'));

                let absolute_path = dir.absolute_path.clone();
                current_node_path = Some(absolute_path.clone());

                let nodes = parse_ls_input(current_node_path.clone(), ls_input);
                for node in nodes {
                    dir.push_child(node.absolute_path());
                    tree.insert_node(node.absolute_path(), node);
                }

                // finally insert the node into the tree again as we may have now populated the childs.
                let node = Node::Dir(dir);
                tree.insert_node(absolute_path.clone(), node);
            }
        } else if line.starts_with("$ ls") {
            panic!("unexpected ls input: {}", line)
        } else {
            panic!("unexpected output line: {}", line)
        }
    }

    let mut sizes = tree.get_dir_sizes();
    sizes.sort_by(|(_, size_a), (_, size_b)| size_a.cmp(size_b));

    // Part 1
    // sizes
    //     .iter()
    //     .fold(
    //         0,
    //         |total, (_, size)| {
    //             if size < &100000 {
    //                 total + size
    //             } else {
    //                 total
    //             }
    //         },
    //     )
    //     .to_string()

    // Part 2
    let total_used_disk_space = tree.nodes.get("").unwrap().size(&tree);

    let target_disk_space = 40_000_000;
    for (_dir_name, dir_size) in sizes {
        if total_used_disk_space - dir_size <= target_disk_space {
            return dir_size.to_string();
        }
    }
    panic!("dir not found")
}

fn parse_ls_input<'a>(parent_path: Option<String>, i: impl Iterator<Item = &'a str>) -> Vec<Node> {
    let mut nodes = vec![];
    for line in i {
        if line.starts_with("dir") {
            let (_, name) = line.split_once(' ').unwrap();
            let node = Node::Dir(Dir::new(name.to_string(), parent_path.clone()));

            nodes.push(node);
        } else {
            let (size, name) = line.split_once(' ').expect("unexpected ls file output");
            let parent_path = parent_path.clone();
            let node = Node::File(File {
                absolute_path: format!("{}/{}", parent_path.unwrap_or_default(), name),
                size: size.parse().unwrap(),
            });

            nodes.push(node);
        }
    }
    nodes
}
