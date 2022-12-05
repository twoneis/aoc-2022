use std::arch::x86_64::_SIDD_MASKED_POSITIVE_POLARITY;
use std::clone;
use std::collections::VecDeque;
use std::fs;
use std::str;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");

    let deck_len = 9;

    let mut initial_container_setup_raw: Vec<String> = Vec::new();
    let mut move_instructions_raw: Vec<String> = Vec::new();

    split_input(
        &input,
        &mut initial_container_setup_raw,
        &mut move_instructions_raw,
    );

    let mut containers_part_one = setup_containers_init(&initial_container_setup_raw, deck_len);
    let mut containers_part_two = setup_containers_init(&initial_container_setup_raw, deck_len);

    let move_instructions = get_formatted_instructions(move_instructions_raw);

    move_part_one(&mut containers_part_one, &move_instructions);
    move_part_two(&mut containers_part_two, &move_instructions);

    print_output(containers_part_one);
    println!();
    print_output(containers_part_two);
}

fn split_input(
    input: &str,
    initial_container_raw: &mut Vec<String>,
    move_instructions_raw: &mut Vec<String>,
) {
    for line in input.lines() {
        if line.contains("[") {
            initial_container_raw.push(line.to_owned());
        } else if line.contains("move") {
            move_instructions_raw.push(line.to_owned());
        }
    }
}

fn setup_containers_init(
    initial_container_setup: &Vec<String>,
    deck_len: usize,
) -> Vec<VecDeque<char>> {
    let mut containers: Vec<VecDeque<char>> = vec![VecDeque::new(); deck_len];

    for line in initial_container_setup {
        let containers_on_level = line
            .as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let mut stack_num = 0;
        for container in containers_on_level {
            if !container.is_empty() {
                for chars in container.chars() {
                    if chars.is_alphanumeric() {
                        containers[stack_num].push_back(chars);
                    }
                }
            }
            stack_num += 1;
        }
    }

    return containers;
}

fn get_formatted_instructions(instructions_raw: Vec<String>) -> Vec<Vec<i32>> {
    let mut formatted_instructions: Vec<Vec<i32>> = Vec::new();
    let mut i = 0;

    for line in instructions_raw {
        formatted_instructions.push(Vec::new());
        for words in line.split_whitespace() {
            if words.trim().parse::<i32>().is_ok() {
                formatted_instructions[i]
                    .push(words.trim().parse::<i32>().expect("can't parse to i32"))
            }
        }
        i += 1;
    }

    return formatted_instructions;
}

fn move_part_one(containers: &mut Vec<VecDeque<char>>, instructions: &Vec<Vec<i32>>) {
    for instruction in instructions {
        let mut i = 0;
        while i < instruction[0] {
            let temp_container = containers[(instruction[1] - 1) as usize].pop_front().expect("tried to move from empty stack");
            containers[(instruction[2] - 1) as usize].push_front(temp_container);
            i += 1;
        }
    }
}

fn move_part_two(containers: &mut Vec<VecDeque<char>>, instructions: &Vec<Vec<i32>>) {
    for instruction in instructions {
        let mut containers_on_arm: VecDeque<char> = VecDeque::new();
        let mut i = 0;
        while i < instruction[0] {
            let temp_container = containers[(instruction[1] - 1) as usize].pop_front().expect("tried to move from empty stack");
            containers_on_arm.push_front(temp_container);
            i += 1;
        }
        while !containers_on_arm.is_empty() {
            let temp_container = containers_on_arm.pop_front().expect("tried taking from empty arm");
            containers[(instruction[2] - 1) as usize].push_front(temp_container);
        }
    }
}

fn print_output(containers: Vec<VecDeque<char>>) {
    for stacks in containers {
        for chars in stacks {
            print!("{chars} ");
        }
        print!("\n");
    }
}
