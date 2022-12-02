use std::fs;

fn main() {
    let inputs = fs::read_to_string("inputs.txt").expect("Error reading input file");
    let split_inputs = inputs.lines();

    for line in split_inputs {
        let mut player_one_move: i32 = 0;
        let mut player_two_move: i32 = 0;
        let mut split = line.split_whitespace();

        match split.next().expect("Error in splitting line; first part") {
            "A" => player_one_move = 1,
            "B" => player_one_move = 2,
            "C" => player_one_move = 3,
            _ => println!("problem?"),
        }

        match split.next().expect("Error in splitting line; second part") {
            "X" => player_two_move = 1,
            "Y" => player_two_move = 2,
            "Z" => player_two_move = 3,
            _ => println!("problem?"),
        }

        println!("{}, {}", player_one_move, player_two_move);
    }
}
