use std::{fs, vec};

struct TreeEnv<T>
where
    T: PartialEq,
{
    env: Vec<Node<T>>,
}

impl<T> TreeEnv<T>
where
    T: PartialEq,
{
    fn new() -> TreeEnv<T> {
        return TreeEnv { env: Vec::new() };
    }

    fn node(&mut self, val: T) -> usize {
        let idx = self.env.len();
        self.env.push(Node::new(idx, val));
        idx
    }

    fn size(&self) -> usize {
        self.env.len()
    }
}

struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    size: u128,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            size: 0,
            parent: None,
            children: vec![],
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let line_split = input.lines();

    let mut tree = TreeEnv::new();
    tree.node("/".to_owned());
    let mut current_dir = tree.env[0].idx;

    for line in line_split {
        if line.contains("$") {
            current_dir = handle_command(line, &tree, &mut current_dir);
        } else {
            let node_to_add = handle_output(line, &tree, current_dir);

            tree.env[current_dir].children.push(node_to_add.idx);
            tree.env.push(node_to_add);
        }
    }

    let leaves = get_leaves(&tree);

    calculate_dir_size(&leaves, &mut tree);

    println!(
        "The sum of all directories with a file size less or equal to 100,000 is {}",
        solve_part_one(&tree)
    );

    println!("The smallest directory that could be deleted to make enough space for the update has a size of {}", solve_part_two(&tree));
}

fn calculate_dir_size(leaves: &Vec<usize>, tree: &mut TreeEnv<String>) {
    for leave in leaves {
        let mut current_node: usize = leave.to_owned();
        while tree.env[current_node].parent.is_some() {
            let size = tree.env[*leave].size;
            current_node = tree.env[current_node].parent.unwrap();
            tree.env[current_node].size += size;
            println!("updating size of {} from bubbeling up {} with size {}", tree.env[current_node].val, tree.env[*leave].val, size)
        }
    }
}

fn solve_part_one(tree: &TreeEnv<String>) -> u128 {
    let mut sum_of_dirs = 0;

    for dir in &tree.env {
        if !dir.children.is_empty() && dir.size <= 100000 {
            sum_of_dirs += dir.size;
        }
    }

    return sum_of_dirs;
}

fn solve_part_two(tree: &TreeEnv<String>) -> u128 {
    let mut size_of_smallest_possible = tree.env[0].size;

    let required_space = 30000000 - (70000000 - tree.env[0].size);

    for dir in &tree.env {
        if !dir.children.is_empty()
            && dir.size < size_of_smallest_possible
            && dir.size >= required_space
        {
            size_of_smallest_possible = dir.size;
        }
    }

    return size_of_smallest_possible;
}

fn get_leaves(tree: &TreeEnv<String>) -> Vec<usize> {
    let mut leaves: Vec<usize> = Vec::new();
    for node in tree.env.iter() {
        if node.children.len() == 0 {
            leaves.push(node.idx);
        }
    }
    return leaves;
}

fn handle_command(line: &str, tree: &TreeEnv<String>, current_dir: &mut usize) -> usize {
    if line.contains("cd") {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split[1] == "cd" {
            if split[2] == ".." {
                return tree.env[current_dir.clone()].parent.unwrap();
            } else if split[2] == "/" {
                let mut root_dir = *current_dir;
                while tree.env[current_dir.clone()].parent.is_some() {
                    root_dir = tree.env[current_dir.clone()].parent.unwrap();
                }
                return root_dir;
            }
            for child in tree.env[current_dir.clone()].children.clone() {
                if tree.env[child].val == split[2] {
                    return tree.env[child].idx;
                }
            }
        }
    }
    return current_dir.clone();
}

fn handle_output(line: &str, tree: &TreeEnv<String>, current_dir: usize) -> Node<String> {
    let split: Vec<&str> = line.split_whitespace().collect();
    let mut new_node = Node::new(tree.size(), split[1].to_owned());
    new_node.parent = Some(current_dir);
    if !line.contains("dir") {
        new_node.size = split[0].parse::<u128>().unwrap();
    }

    new_node
}
