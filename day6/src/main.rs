use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");

    let mut recent_four_chars: VecDeque<char> = VecDeque::new();
    let mut buffer_counter = 0;
    let mut no_dups = false;

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
                break;
            }
        }
    }

    println!("{buffer_counter}");
}
