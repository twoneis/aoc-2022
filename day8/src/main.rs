use std::fs;

fn main() {
    let input = fs::read_to_string("test.txt").expect("error reading file");
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

    let mut row_num = 0;
    let mut column_num = 0;
    for tree_row in tree_grid.clone() {
        column_num = 0;
        for tree in tree_row {
            if test_tree_visibility(row_num, column_num, tree_grid.clone()) {
                print!("{} ", tree);
            } else {
                print!("  ");
            }
            column_num += 1;
        }
        row_num += 1;
        print!("\n")
    }
}

fn test_tree_visibility(x: usize, y: usize, tree_grid: Vec<Vec<i32>>) -> bool {
    let tree_to_test = tree_grid[x][y];

    let mut tree_visible_left = true;
    let mut tree_visible_right = true;
    let mut tree_visible_front = true;
    let mut tree_visible_back = true;

    let mut row_num = 0;
    let mut column_num = 0;
    for tree_row in tree_grid {
        column_num = 0;
        for tree in tree_row {
            if tree >= tree_to_test && (column_num == y || row_num == x) {
                //println!("comparing {} against {}", tree, tree_to_test);

                if column_num == y && row_num < x {
                    tree_visible_left = false;
                    //println!("not visible from left");
                }
                if column_num == y && row_num > x {
                    tree_visible_right = false;
                    //println!("not visible from right");
                }
                if column_num < y && row_num == x {
                    tree_visible_back = false;
                    //println!("not visible from back");
                }
                if column_num > y && row_num == x {
                    tree_visible_front = false;
                    //println!("not visible from front");
                }
            }
            column_num += 1;
        }
        row_num += 1;
    }

    if tree_visible_left || tree_visible_right || tree_visible_front || tree_visible_back {
        return true;
    }
    return false;
}
