use std::collections::HashMap;
use std::path::{Path, PathBuf};

enum FsEntry {
    Directory(String, Vec<FsEntry>),
    File(usize),
}

fn parse_entry(pwd: &Path, line: &str) -> FsEntry {
    let params = line.split(' ').collect::<Vec<_>>();

    if params[0] == "dir" {
        FsEntry::Directory(
            pwd.join(Path::new(params[1])).to_str().unwrap().to_string(),
            Vec::new(),
        )
    } else {
        let length = params[0].parse::<usize>().unwrap();
        FsEntry::File(length)
    }
}

fn resolve_dir(root: &mut [FsEntry], directories: &mut HashMap<PathBuf, Vec<FsEntry>>) {
    for entry in root.iter_mut() {
        if let FsEntry::Directory(path, _) = entry {
            let mut real_entries = directories.remove(Path::new(path)).unwrap();
            resolve_dir(&mut real_entries, directories);
            *entry = FsEntry::Directory(path.clone(), real_entries);
        }
    }
}

fn populate(
    root_path: String,
    root: &[FsEntry],
    directories_sizes: &mut HashMap<String, usize>,
) -> usize {
    let mut total = 0;

    for entry in root {
        match entry {
            FsEntry::Directory(path, other) => {
                total += populate(path.clone(), other, directories_sizes);
            }

            FsEntry::File(l) => {
                total += *l;
            }
        }
    }

    directories_sizes.insert(root_path, total);
    total
}

const UPDATE_SIZE: usize = 30000000;
const FILESYSTEM_SIZE: usize = 70000000;

fn main() {
    let input = include_str!("../input.txt");
    let mut path = PathBuf::from("/");
    let mut directories = HashMap::new();

    for command in input.split("$ ").skip(1) {
        let lines = command
            .split('\n')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let (cmd_line, output) = lines.split_first().unwrap();
        let cmd_args = cmd_line.split(' ').collect::<Vec<_>>();
        let (&cmd, args) = cmd_args.split_first().unwrap();

        match cmd {
            "cd" => {
                if args[0] == ".." {
                    path.pop();
                } else {
                    path.push(args[0]);
                }
            }

            "ls" => {
                directories.insert(
                    path.clone(),
                    output
                        .iter()
                        .map(|x| parse_entry(path.as_path(), x))
                        .collect::<Vec<FsEntry>>(),
                );
            }
            _ => unimplemented!(),
        }
    }

    let mut root = directories.remove(Path::new("/")).unwrap();
    resolve_dir(&mut root, &mut directories);

    let mut directories_sizes = HashMap::new();
    populate("/".into(), &root, &mut directories_sizes);

    let total: usize = directories_sizes.values().filter(|&x| *x <= 100000).sum();

    println!("p1: {}", total);

    let used_size = directories_sizes.get("/").unwrap();
    let need_at_least = UPDATE_SIZE - (FILESYSTEM_SIZE - used_size);

    let mut candidates: Vec<_> = directories_sizes
        .values()
        .filter_map(|&x| (x > need_at_least).then_some(x))
        .collect();

    candidates.sort();
    println!("p2: {}", candidates.first().unwrap());
}
