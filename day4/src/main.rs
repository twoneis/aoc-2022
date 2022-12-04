use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let split = input.lines();

    let mut contained_line_counter = 0;
    let mut overlap_line_counter = 0;

    for lines in split {
        let mut line_numbers: Vec<i32> = Vec::new();
        let mut temp_num: String = String::new();

        for char_in_line in lines.chars() {
            if char_in_line.is_ascii_digit() {
                temp_num.push(char_in_line);
            } else {
                line_numbers.push(temp_num.parse::<i32>().unwrap());
                temp_num.clear();
            }
        }
        line_numbers.push(temp_num.parse::<i32>().unwrap());

        if (line_numbers[0] >= line_numbers[2] && line_numbers[1] <= line_numbers[3])
            || (line_numbers[0] <= line_numbers[2] && line_numbers[1] >= line_numbers[3])
        {
            contained_line_counter += 1;
        }

        if (line_numbers[1] >= line_numbers[2] && line_numbers[0] <= line_numbers[3])
            || (line_numbers[3] >= line_numbers[0] && line_numbers[2] <= line_numbers[1])
        {
            overlap_line_counter += 1;
        }
    }

    println!("{contained_line_counter}");
    println!("{overlap_line_counter}");
}
