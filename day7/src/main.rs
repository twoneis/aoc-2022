use std::fs;

struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
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

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let line_split = input.lines();

    let mut Tree = ArenaTree::new();
    let root = Node::new(0, "root");
    Tree.node(root.val);

    for line in line_split {
        if line.contains("$") {
            handle_command(line);
        } else {
        }
    }
}

fn handle_command(line: &str) {
    if line.contains("cd") {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split[2] == "cd" {}
    }
}

fn handle_output() {}
