use std::{env::current_dir, fs, borrow::{Borrow, BorrowMut}, slice::SliceIndex};

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
    size: Option<i32>,
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
            size: None,
            parent: None,
            children: vec![],
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let line_split = input.lines();

    let mut tree = ArenaTree::new();
    let root = Node::new(0, "root");
    tree.node(root.val);
    let mut current_dir = root.idx;

    for line in line_split {
        if line.contains("$") {
            current_dir = handle_command(line, &tree, &mut current_dir);
        } else {
            handle_output(line);
        }
    }
}

fn handle_command(line: &str, tree: &ArenaTree<&str>, current_dir: &mut usize) -> usize {
    if line.contains("cd") {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split[1] == "cd" {
            if split[2] == ".." {
                return  tree.arena[current_dir.clone()].parent.unwrap();
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

fn handle_output(line: &str) {
    if line.contains("dir") {
        
    }
}
