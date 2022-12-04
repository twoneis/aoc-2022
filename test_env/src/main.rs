use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("err reading file");
    let split = input.split("\n\n");

    let mut elf_count = 0;
    let mut highest = 0;

    for elf in split {
        elf_count += 1;
        let mut counting = 0;
        let lines = elf.lines();
        for calories in lines {
            if !calories.is_empty() {
                counting += calories.parse::<i32>().unwrap();
            }
        }
        println!("{counting}");
        if counting > highest {
            highest = counting;
        }
    }

    println!("{elf_count}");
    println!("{highest}");
}