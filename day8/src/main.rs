use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let lines = input.lines();

    let mut line_count = 0;
    for line in input.lines() {
        line_count += 1;
    }

    let mut tree_grid = vec![vec![0; line_count]; line_count];

    let mut row_num = 0;
    let mut column_num = 0;
    for line in lines {
        column_num = 0;
        for tree in line.chars() {
            let tree_val = tree as i32 - '0' as i32;
            tree_grid[row_num][column_num] = tree_val;
            column_num += 1;
        }
        row_num += 1;
    }

    for tree_row in tree_grid {
        for tree in tree_row {
            print!("{} ", tree);
        }
        print!("\n")
    }
}
