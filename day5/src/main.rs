use std::collections::VecDeque;
use std::fs;
use std::str;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("error reading file");
    let lines = input.lines();
    let mut containers = vec![VecDeque::new(); 9];
    let mut containers_part_two = vec![VecDeque::new(); 9];

    for line in lines {
        if line.contains("[") {
            let subs = line
                .as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();

            let mut stack_num = 0;
            for container in subs {
                if !container.is_empty() {
                    for chars in container.chars() {
                        if chars.is_alphanumeric() {
                            containers[stack_num].push_back(chars);
                            containers_part_two[stack_num].push_back(chars);
                        }
                    }
                }
                stack_num += 1;
            }
        } else if line.contains("move") {
            let mut move_instructions = Vec::new();
            for words in line.split_whitespace() {
                if words.trim().parse::<i32>().is_ok() {
                    move_instructions.push(words.trim().parse::<i32>().expect("can't parse to i32"))
                }
            }

            let mut temp_containers_moving = VecDeque::new();

            let mut i = 0;
            while i < move_instructions[0] {
                let moving = containers[(move_instructions[1] - 1) as usize]
                    .pop_front()
                    .expect("err");
                containers[(move_instructions[2] - 1) as usize].push_front(moving);
                let moving_part_two = containers_part_two[(move_instructions[1] - 1) as usize].pop_front().expect("err");
                temp_containers_moving.push_back(moving_part_two);
                i += 1;
            }

            while !temp_containers_moving.is_empty() {
                containers_part_two[(move_instructions[2] - 1) as usize].push_front(temp_containers_moving.pop_back().expect("ai"));
            }
        }
    }

    for stacks in containers {
        for chars in stacks {
            print!("{chars} ",);
        }
        print!("\n");
    }
    println!();
    for stacks in containers_part_two {
        for chars in stacks {
            print!("{chars} ",);
        }
        print!("\n");
    }
}
