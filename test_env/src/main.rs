fn main() {
    // Read the input data from the file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    // Get the number of rows and columns in the grid
    let rows = grid.len();
    let cols = grid[0].len();

    // Initialize the count of visible trees
    let mut count = 0;

    // Iterate through each row and column
    for i in 0..rows {
        for j in 0..cols {
            // Check if the current tree is visible
            let mut visible = true;
            for k in 0..rows {
                if i != k && grid[k][j] > grid[i][j] {
                    visible = false;
                    break;
                }
            }
            for k in 0..cols {
                if j != k && grid[i][k] > grid[i][j] {
                    visible = false;
                    break;
                }
            }
            // If the tree is visible based on its height,
            // check whether all of the trees between it and the edge are shorter
            if visible {
                for k in 0..rows {
                    if i != k && grid[k][j] <= grid[i][j] {
                        visible = false;
                        break;
                    }
                }
                for k in 0..cols {
                    if j != k && grid[i][k] <= grid[i][j] {
                        visible = false;
                        break;
                    }
                }
            }
            // If the tree is visible, increment the count
            if visible || i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                count += 1;
            }
        }
    }

    // Print the result
    println!("{} trees are visible from outside the grid.", count);
}
