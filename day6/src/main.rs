use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut recent_four_chars: VecDeque<char> = VecDeque::new();
    let mut buffer_counter = 0;
    let mut no_dups: bool;

    for received_char in input.chars() {
        recent_four_chars.push_back(received_char);
        buffer_counter += 1;
        if recent_four_chars.len() > 4 {
            recent_four_chars.pop_front();
            let mut i = 0;
            no_dups = true;
            while i < recent_four_chars.len() {
                let mut j = i + 1;
                while j < recent_four_chars.len() {
                    if recent_four_chars[i] == recent_four_chars[j] {
                        no_dups = false;
                    }
                    j += 1;
                }
                i += 1;
            }
            if no_dups {
                return buffer_counter;
            }
        }
    }
    return buffer_counter;
}

fn part_two(input: &str) -> i32 {
    let mut recent_fourteen_chars: VecDeque<char> = VecDeque::new();
    let mut buffer_counter = 0;
    let mut no_dups: bool;

    for received_char in input.chars() {
        recent_fourteen_chars.push_back(received_char);
        buffer_counter += 1;
        if recent_fourteen_chars.len() > 14 {
            recent_fourteen_chars.pop_front();
            let mut i = 0;
            no_dups = true;
            while i < recent_fourteen_chars.len() {
                let mut j = i + 1;
                while j < recent_fourteen_chars.len() {
                    if recent_fourteen_chars[i] == recent_fourteen_chars[j] {
                        no_dups = false;
                    }
                    j += 1;
                }
                i += 1;
            }
            if no_dups {
                return buffer_counter;
            }
        }
    }
    return buffer_counter;
}
