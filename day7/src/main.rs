use std::{fs, vec};

struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn new() -> ArenaTree<T> {
        return ArenaTree { arena: Vec::new() };
    }

    fn node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn size(&self) -> usize {
        self.arena.len()
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

    let mut tree = ArenaTree::new();
    let root = Node::new(0, "root".to_owned());
    tree.node(root.val);
    let mut current_dir = root.idx;

    for line in line_split {
        if line.contains("$") {
            current_dir = handle_command(line, &tree, &mut current_dir);
        } else {
            let node_to_add = handle_output(line, &tree, current_dir);

            tree.arena[current_dir].children.push(node_to_add.idx);
            tree.arena.push(node_to_add);
        }
    }

    let leaves = get_leaves(&tree);
    println!(
        "The sum of all directories with a file size less or equal to 100,000 is {}",
        solve_part_one(&leaves, &mut tree)
    );

    println!("The smallest directory that could be deleted to make enough space for the update has a size of {}", solve_part_two(&leaves, &mut tree));
}

fn solve_part_one(leaves: &Vec<usize>, tree: &mut ArenaTree<String>) -> u128 {
    let mut sum_of_dirs = 0;

    for leave in leaves {
        let mut current_node: usize = leave.to_owned();
        while tree.arena[current_node].parent.is_some() {
            let size = tree.arena[current_node].size;
            current_node = tree.arena[current_node].parent.unwrap();
            tree.arena[current_node].size += size;
        }
    }

    for dir in &tree.arena {
        if !dir.children.is_empty() && dir.size <= 100000 {
            sum_of_dirs += dir.size;
        }
    }

    return sum_of_dirs;
}

fn solve_part_two(leaves: &Vec<usize>, tree: &mut ArenaTree<String>) -> u128 {
    let mut sum_of_dirs = 0;

    for leave in leaves {
        let mut current_node: usize = leave.to_owned();
        while tree.arena[current_node].parent.is_some() {
            let size = tree.arena[current_node].size;
            current_node = tree.arena[current_node].parent.unwrap();
            tree.arena[current_node].size += size;
        }
    }

    for dir in &tree.arena {
        if !dir.children.is_empty() && dir.size <= 100000 {
            sum_of_dirs += dir.size;
        }
    }

    return sum_of_dirs;
}

fn get_leaves(tree: &ArenaTree<String>) -> Vec<usize> {
    let mut leaves: Vec<usize> = Vec::new();
    for node in tree.arena.iter() {
        if node.children.len() == 0 {
            leaves.push(node.idx);
        }
    }
    return leaves;
}

fn handle_command(line: &str, tree: &ArenaTree<String>, current_dir: &mut usize) -> usize {
    if line.contains("cd") {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split[1] == "cd" {
            if split[2] == ".." {
                return tree.arena[current_dir.clone()].parent.unwrap();
            } else if split[2] == "/" {
                let mut root_dir = *current_dir;
                while tree.arena[current_dir.clone()].parent.is_some() {
                    root_dir = tree.arena[current_dir.clone()].parent.unwrap();
                }
                return root_dir;
            }
            for child in tree.arena[current_dir.clone()].children.clone() {
                if tree.arena[child].val == split[2] {
                    return tree.arena[child].idx;
                }
            }
        }
    }
    return current_dir.clone();
}

fn handle_output(line: &str, tree: &ArenaTree<String>, current_dir: usize) -> Node<String> {
    let split: Vec<&str> = line.split_whitespace().collect();
    let mut new_node = Node::new(tree.size(), split[1].to_owned());
    new_node.parent = Some(current_dir);
    if !line.contains("dir") {
        new_node.size = split[0].parse::<u128>().unwrap();
    }

    new_node
}
