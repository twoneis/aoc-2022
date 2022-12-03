use std::fs;

fn main() {
    let input = fs::read_to_string("test_inputs.txt").expect("Error reading input file");
    let split_inputs = input.lines();

    for line in split_inputs {
        let items = line.split_at(line.len()/2);
        let duplicate_char = get_dup_letter(items);
        println!("{}", duplicate_char.to_digit(10).expect("not valid char"));
    }
}

fn get_dup_letter (items: (&str, &str)) -> char{
    for first_word_letter in items.0.chars() {
        for second_word_letter in items.0.chars() {
            if first_word_letter.eq(&second_word_letter) {
                return first_word_letter;
            }
        }
    }
    return 'Z';
}