use std::fs;

fn main() {
    let inputs = fs::read_to_string("inputs.txt").expect("Error reading input file");
    let split_inputs = inputs.lines();

    let mut player_two_score_part_one = 0;
    let mut player_two_score_part_two = 0;

    for line in split_inputs {
        let mut split = line.split_whitespace();

        let player_one_move =
            part_one_player_one(split.next().expect("Error in splitting line; first part"));
        let player_two_move_part_one =
            part_one_player_two(split.next().expect("Error in splitting line; second part"));

        let player_two_move_part_two =
            part_two_player_two(player_one_move, player_two_move_part_one);

        player_two_score_part_one += player_two_move_part_one;

        if player_two_move_part_one == player_one_move + 1
            || (player_one_move == 3 && player_two_move_part_one == 1)
        {
            player_two_score_part_one += 6;
        } else if player_one_move == player_two_move_part_one {
            player_two_score_part_one += 3;
        }

        player_two_score_part_two += player_two_move_part_two;

        if player_two_move_part_two == player_one_move + 1
            || (player_one_move == 3 && player_two_move_part_two == 1)
        {
            player_two_score_part_two += 6;
        } else if player_one_move == player_two_move_part_two {
            player_two_score_part_two += 3;
        }
    }

    println!(
        "Player two score for part one is {}",
        player_two_score_part_one
    );
    println!(
        "Player two score for part two is {}",
        player_two_score_part_two
    );
}

fn part_one_player_one(player_move: &str) -> i32 {
    match player_move {
        "A" => return 1,
        "B" => return 2,
        "C" => return 3,
        _ => return 0,
    }
}

fn part_one_player_two(player_move: &str) -> i32 {
    match player_move {
        "X" => return 1,
        "Y" => return 2,
        "Z" => return 3,
        _ => return 0,
    }
}

fn part_two_player_two(player_one_move: i32, result: i32) -> i32 {
    let mut ret_val = 0;
    if result == 2 {
        ret_val = player_one_move;
    } else if result == 1 {
        ret_val = player_one_move - 1;
        if ret_val == 0 {
            ret_val = 3
        }
    } else if result == 3 {
        ret_val = player_one_move + 1;
        if ret_val == 4 {
            ret_val = 1
        }
    }
    return ret_val;
}
