use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let lines = input.lines();

    let mut line_count = 0;
    for _line in input.lines() {
        line_count += 1;
    }

    let mut tree_grid = vec![vec![0; line_count]; line_count];

    let mut row_num = 0;
    for line in lines {
        let mut column_num = 0;
        for tree in line.chars() {
            let tree_val = tree as i32 - '0' as i32;
            tree_grid[row_num][column_num] = tree_val;
            column_num += 1;
        }
        row_num += 1;
    }

    let mut visible_tree_counter = 0;
    let mut max_scenic_score = 0;

    let mut row_num = 0;
    for tree_row in tree_grid.clone() {
        let mut column_num = 0;
        for tree in tree_row {
            if get_scenic_score(row_num, column_num, tree_grid.clone()) > max_scenic_score {
                max_scenic_score = get_scenic_score(row_num, column_num, tree_grid.clone());
            }
            if test_tree_visibility(row_num, column_num, tree_grid.clone()) {
                print!("{}", tree);
                print!(
                    "{} ",
                    get_scenic_score(row_num, column_num, tree_grid.clone())
                );
                visible_tree_counter += 1;
            } else {
                print!(" ");
                print!(
                    "{} ",
                    get_scenic_score(row_num, column_num, tree_grid.clone())
                );
            }
            column_num += 1;
        }
        row_num += 1;
        print!("\n")
    }

    println!(
        "A total of {} trees are visible from outside the forest",
        visible_tree_counter
    );

    println!("The maximum scenic score possible is {}", max_scenic_score);
}

fn test_tree_visibility(x: usize, y: usize, tree_grid: Vec<Vec<i32>>) -> bool {
    let tree_to_test = tree_grid[x][y];

    let mut tree_visible_left = true;
    let mut tree_visible_right = true;
    let mut tree_visible_front = true;
    let mut tree_visible_back = true;

    let mut row_num = 0;
    for tree_row in tree_grid {
        let mut column_num = 0;
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

fn get_scenic_score(x: usize, y: usize, tree_grid: Vec<Vec<i32>>) -> i32 {
    let grid_size = tree_grid.len() - 1;
    let tree_to_test = tree_grid[x][y];

    let mut row_num = x;
    let mut column_num = y;

    let mut scenic_score_left = 0;
    while column_num > 0 {
        scenic_score_left += 1;
        column_num -= 1;
        if tree_grid[row_num][column_num] >= tree_to_test {
            break;
        }
    }
    column_num = y;

    let mut scenic_score_right = 0;
    while column_num < grid_size {
        scenic_score_right += 1;
        column_num += 1;
        if tree_grid[row_num][column_num] >= tree_to_test {
            break;
        }
    }
    column_num = y;

    let mut scenic_score_front = 0;
    while row_num > 0 {
        scenic_score_front += 1;
        row_num -= 1;
        if tree_grid[row_num][column_num] >= tree_to_test {
            break;
        }
    }
    row_num = x;

    let mut scenic_score_back = 0;
    while row_num < grid_size {
        scenic_score_back += 1;
        row_num += 1;
        if tree_grid[row_num][column_num] >= tree_to_test {
            break;
        }
    }

    let scenic_score =
        scenic_score_left * scenic_score_right * scenic_score_front * scenic_score_back;

    return scenic_score;
}
