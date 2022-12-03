use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Error reading input file");
    let split_inputs = input.lines();
    let mut priority_sum = 0;

    for line in split_inputs {
        let items = line.split_at(line.len()/2);
        let duplicate_char = get_dup_letter(items);
        let priority = letter_to_u32(duplicate_char);
        priority_sum += priority;
    }

    println!("{}", priority_sum);
}

fn get_dup_letter (items: (&str, &str)) -> char{
    for first_word_letter in items.0.chars() {
        for second_word_letter in items.1.chars() {
            if first_word_letter.eq(&second_word_letter) {
                return first_word_letter;
            }
        }
    }
    return 'Z';
}

fn letter_to_u32 (letter: char) -> u32 {
    let mut ret_val = letter.to_digit(36).expect("not valid letter") - 9;
    if letter.is_uppercase() {
        ret_val += 26;
    }
    return ret_val;
}